use chrono::{offset::Local, NaiveDateTime};
use indexer_core::{
    db::{
        insert_into,
        models::Listing,
        tables::listings::{address, listings},
    },
    prelude::*,
    pubkeys,
    pubkeys::find_auction_data_extended,
};
use metaplex_auction::processor::{
    AuctionData, AuctionDataExtended, BidState, BidderMetadata, PriceFloor, BIDDER_METADATA_LEN,
};
use solana_sdk::pubkey::Pubkey;

use crate::{client::prelude::*, util, Client, RcAuctionKeys, ThreadPoolHandle};

pub fn process(client: &Client, keys: &RcAuctionKeys, _handle: ThreadPoolHandle) -> Result<()> {
    let (ext, _bump) = find_auction_data_extended(&keys.vault);

    let mut acct = client
        .get_account(&keys.auction)
        .context("Failed to get auction data")?;

    let auction = AuctionData::from_account_info(&util::account_as_info(
        &keys.auction,
        false,
        false,
        &mut acct,
    ))
    .context("Failed to parse AuctionData")?;

    let mut acct = client
        .get_account(&ext)
        .context("Failed to get extended auction data")?;

    let ext = AuctionDataExtended::from_account_info(&util::account_as_info(
        &ext, false, false, &mut acct,
    ))
    .context("Failed to parse AuctionDataExtended")?;

    // TODO: what timezone is any of this in????
    let addr = keys.auction.to_bytes();
    let auth_addr = auction.authority.to_bytes();
    let mint_addr = auction.token_mint.to_bytes();
    let store_owner_addr = keys.store_owner.to_bytes();
    let row = Listing {
        address: Borrowed(&addr),
        ends_at: auction
            .ended_at
            .map(|t| NaiveDateTime::from_timestamp(t, 0)),
        created_at: keys.created_at,
        ended: auction
            .ended(Local::now().naive_utc().timestamp())
            .context("Failed to check if auction was ended")?,
        authority: Borrowed(&auth_addr),
        token_mint: Borrowed(&mint_addr),
        store_owner: Borrowed(&store_owner_addr),
        last_bid: auction.last_bid,
        // TODO: horrible abuse of the NaiveDateTime struct but Metaplex does
        //       roughly the same thing with the solana UnixTimestamp struct.
        end_auction_gap: auction
            .end_auction_gap
            .map(|g| NaiveDateTime::from_timestamp(g, 0)),
        price_floor: match auction.price_floor {
            PriceFloor::None(_) => None,
            PriceFloor::MinimumPrice(p) => Some(
                p[0].try_into()
                    .context("Price floor is too high to store")?,
            ),
            PriceFloor::BlindedPrice(_) => Some(-1),
        },
        total_uncancelled_bids: count_bids(client, &keys.auction, &auction, &ext)?
            .map(|n| n.try_into().context("Bid count is too high to store!"))
            .transpose()?,
        gap_tick_size: ext.gap_tick_size_percentage.map(Into::into),
        instant_sale_price: ext
            .instant_sale_price
            .map(TryFrom::try_from)
            .transpose()
            .context("Instant sale price is too high to store")?,
        name: Borrowed(
            ext.name
                .as_ref()
                .map(|n| std::str::from_utf8(n))
                .transpose()
                .context("Couldn't parse auction name")?
                .unwrap_or("")
                .trim_end_matches('\0'),
        ),
    };

    let db = client.db()?;

    insert_into(listings)
        .values(&row)
        .on_conflict(address)
        .do_update()
        .set(&row)
        .execute(&db)
        .context("Failed to insert listing")?;

    Ok(())
}

fn count_bids(
    client: &Client,
    auction_key: &Pubkey,
    auction: &AuctionData,
    ext: &AuctionDataExtended,
) -> Result<Option<usize>> {
    // TODO: work-in-progress for proper accounting of bidder metadata.
    //       there's a lot of moving parts so this will have to be resolved as
    //       its own issue

    // let mut bidders = Vec::new();

    // let queried = if matches!(auction.bid_state, BidState::EnglishAuction { .. }) {
    //     let bids = client
    //         .get_program_accounts(pubkeys::auction(), RpcProgramAccountsConfig {
    //             filters: Some(vec![
    //                 RpcFilterType::DataSize(BIDDER_METADATA_LEN.try_into().unwrap()),
    //                 RpcFilterType::Memcmp(Memcmp {
    //                     offset: 32,
    //                     bytes: MemcmpEncodedBytes::Bytes(auction_key.to_bytes().into()),
    //                     encoding: None,
    //                 }),
    //             ]),
    //             ..RpcProgramAccountsConfig::default()
    //         })
    //         .context("Failed to retrieve bids for auction")?;

    //     bids.into_iter()
    //         .map(|(key, mut acct)| {
    //             BidderMetadata::from_account_info(&util::account_as_info(
    //                 &key, false, false, &mut acct,
    //             ))
    //         })
    //         .try_fold(0, |n, r| r.map(|b| {
    //             bidders.push(b.clone()); // TODO

    //             if b.cancelled { n } else { n + 1 }
    //         }))
    //         .context("Failed to count bids")
    //         .map(Some)
    // } else {
    //     Ok(None)
    // };

    // warn!("{:#?}", bidders);
    // warn!("{:#?}", auction.bid_state);

    if ext.instant_sale_price.is_some() {
        return Ok(None);
    }

    Ok(match auction.bid_state {
        BidState::EnglishAuction { ref bids, .. } => Some(bids.len()),
        BidState::OpenEdition { .. } => None,
    })
}
