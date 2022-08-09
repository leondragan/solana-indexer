use dataloaders::{
    collection::{CollectionFloorPrice, CollectionNftCount},
    Batcher, Loader, TwitterBatcher,
};
use indexer_core::uuid::Uuid;
use objects::{
    ah_listing::AhListing,
    ah_offer::Offer as AhOffer,
    ah_purchase::Purchase as AhPurchase,
    auction_house::AuctionHouse,
    bid_receipt::BidReceipt,
    genopets::{GenoHabitat, GenoRentalAgreement},
    graph_connection::GraphConnection,
    listing::{Bid, Listing},
    listing_receipt::ListingReceipt,
    nft::{Collection, Nft, NftActivity, NftAttribute, NftCreator, NftFile, NftOwner},
    profile::TwitterProfile,
    purchase_receipt::PurchaseReceipt,
    spl_governance::{
        Governance, GovernanceConfig, MultiChoice, Proposal, ProposalOption, Realm, RealmConfig,
        TokenOwnerRecord, VoteChoice, VoteRecord,
    },
    stats::{MarketStats, MintStats},
    store_creator::StoreCreator,
    storefront::Storefront,
    wallet::Wallet,
};
use scalars::{
    markers::{StoreConfig, TokenMint},
    PublicKey,
};

use super::prelude::*;

#[derive(Clone)]
pub struct AppContext {
    pub(crate) shared: Arc<SharedData>,

    // Postgres dataloaders
    pub ah_listing_loader: Loader<Uuid, Option<AhListing>>,
    pub ah_listings_loader: Loader<PublicKey<Nft>, Vec<AhListing>>,
    pub auction_house_loader: Loader<PublicKey<AuctionHouse>, Option<AuctionHouse>>,
    pub auction_houses_loader: Loader<PublicKey<StoreConfig>, Vec<AuctionHouse>>,
    pub bid_receipt_loader: Loader<PublicKey<BidReceipt>, Option<BidReceipt>>,
    pub bid_receipts_loader: Loader<PublicKey<Nft>, Vec<BidReceipt>>,
    pub collection_count_loader: Loader<PublicKey<StoreCreator>, Option<i32>>,
    pub collection_floor_price_loader: Loader<PublicKey<Collection>, Option<CollectionFloorPrice>>,
    pub collection_loader: Loader<PublicKey<StoreCreator>, Vec<Nft>>,
    pub collection_nft_count_loader: Loader<PublicKey<Collection>, Option<CollectionNftCount>>,
    pub geno_rental_agreement_loader: Loader<PublicKey<GenoHabitat>, Option<GenoRentalAgreement>>,
    pub graph_connection_loader: Loader<PublicKey<GraphConnection>, Option<GraphConnection>>,
    pub listing_bids_loader: Loader<PublicKey<Listing>, Vec<Bid>>,
    pub listing_loader: Loader<PublicKey<Listing>, Option<Listing>>,
    pub listing_nfts_loader: Loader<PublicKey<Listing>, Vec<(usize, Nft)>>,
    pub listing_receipt_loader: Loader<PublicKey<ListingReceipt>, Option<ListingReceipt>>,
    pub listing_receipts_loader: Loader<PublicKey<Nft>, Vec<ListingReceipt>>,
    pub market_stats_loader: Loader<PublicKey<StoreConfig>, Option<MarketStats>>,
    pub mint_stats_loader: Loader<PublicKey<AuctionHouse>, Option<MintStats>>,
    pub nft_activities_loader: Loader<PublicKey<Nft>, Vec<NftActivity>>,
    pub nft_attributes_loader: Loader<PublicKey<Nft>, Vec<NftAttribute>>,
    pub nft_by_mint_loader: Loader<PublicKey<TokenMint>, Option<Nft>>,
    pub nft_collection_loader: Loader<PublicKey<Nft>, Option<Collection>>,
    pub nft_creators_loader: Loader<PublicKey<Nft>, Vec<NftCreator>>,
    pub nft_files_loader: Loader<PublicKey<Nft>, Vec<NftFile>>,
    pub nft_loader: Loader<PublicKey<Nft>, Option<Nft>>,
    pub nft_owner_loader: Loader<PublicKey<Nft>, Option<NftOwner>>,
    pub offer_loader: Loader<Uuid, Option<AhOffer>>,
    pub offers_loader: Loader<PublicKey<Nft>, Vec<AhOffer>>,
    pub purchase_loader: Loader<Uuid, Option<AhPurchase>>,
    pub purchase_receipt_loader: Loader<PublicKey<PurchaseReceipt>, Option<PurchaseReceipt>>,
    pub purchase_receipts_loader: Loader<PublicKey<Nft>, Vec<PurchaseReceipt>>,
    pub purchases_loader: Loader<PublicKey<Nft>, Vec<AhPurchase>>,
    pub store_auction_houses_loader: Loader<PublicKey<AuctionHouse>, Option<AuctionHouse>>,
    pub store_creator_loader: Loader<PublicKey<StoreConfig>, Vec<StoreCreator>>,
    pub storefront_loader: Loader<PublicKey<Storefront>, Option<Storefront>>,
    pub twitter_handle_loader: Loader<PublicKey<Wallet>, Option<String>>,
    pub governance_loader: Loader<PublicKey<Governance>, Option<Governance>>,
    pub governance_config_loader: Loader<PublicKey<Governance>, Option<GovernanceConfig>>,
    pub realm_loader: Loader<PublicKey<Realm>, Option<Realm>>,
    pub realm_config_loader: Loader<PublicKey<Realm>, Option<RealmConfig>>,
    pub token_owner_record_loader: Loader<PublicKey<TokenOwnerRecord>, Option<TokenOwnerRecord>>,
    pub proposal_loader: Loader<PublicKey<Proposal>, Option<Proposal>>,
    pub proposal_options_loader: Loader<PublicKey<Proposal>, Vec<ProposalOption>>,
    pub proposal_multi_choice_loader: Loader<PublicKey<Proposal>, Option<MultiChoice>>,
    pub approve_vote_choices_loader: Loader<PublicKey<VoteRecord>, Vec<VoteChoice>>,
    pub vote_record_token_owner_loader: Loader<PublicKey<Wallet>, Vec<TokenOwnerRecord>>,

    // Twitter dataloaders
    pub twitter_profile_loader: Loader<String, Option<TwitterProfile>, TwitterBatcher>,
}

impl juniper::Context for AppContext {}

impl AppContext {
    pub(crate) fn new(shared: Arc<SharedData>) -> AppContext {
        let batcher = Batcher::new(shared.db.clone());
        let twitter_batcher = TwitterBatcher::new(shared.clone());

        Self {
            shared,

            ah_listing_loader: Loader::new(batcher.clone()),
            ah_listings_loader: Loader::new(batcher.clone()),
            auction_house_loader: Loader::new(batcher.clone()),
            auction_houses_loader: Loader::new(batcher.clone()),
            bid_receipt_loader: Loader::new(batcher.clone()),
            bid_receipts_loader: Loader::new(batcher.clone()),
            collection_count_loader: Loader::new(batcher.clone()),
            collection_floor_price_loader: Loader::new(batcher.clone()),
            collection_loader: Loader::new(batcher.clone()),
            collection_nft_count_loader: Loader::new(batcher.clone()),
            geno_rental_agreement_loader: Loader::new(batcher.clone()),
            graph_connection_loader: Loader::new(batcher.clone()),
            listing_bids_loader: Loader::new(batcher.clone()),
            listing_loader: Loader::new(batcher.clone()),
            listing_nfts_loader: Loader::new(batcher.clone()),
            listing_receipt_loader: Loader::new(batcher.clone()),
            listing_receipts_loader: Loader::new(batcher.clone()),
            market_stats_loader: Loader::new(batcher.clone()),
            mint_stats_loader: Loader::new(batcher.clone()),
            nft_activities_loader: Loader::new(batcher.clone()),
            nft_attributes_loader: Loader::new(batcher.clone()),
            nft_by_mint_loader: Loader::new(batcher.clone()),
            nft_collection_loader: Loader::new(batcher.clone()),
            nft_creators_loader: Loader::new(batcher.clone()),
            nft_files_loader: Loader::new(batcher.clone()),
            nft_loader: Loader::new(batcher.clone()),
            nft_owner_loader: Loader::new(batcher.clone()),
            offer_loader: Loader::new(batcher.clone()),
            offers_loader: Loader::new(batcher.clone()),
            purchase_loader: Loader::new(batcher.clone()),
            purchase_receipt_loader: Loader::new(batcher.clone()),
            purchase_receipts_loader: Loader::new(batcher.clone()),
            purchases_loader: Loader::new(batcher.clone()),
            store_auction_houses_loader: Loader::new(batcher.clone()),
            store_creator_loader: Loader::new(batcher.clone()),
            storefront_loader: Loader::new(batcher.clone()),
            governance_loader: Loader::new(batcher.clone()),
            governance_config_loader: Loader::new(batcher.clone()),
            realm_loader: Loader::new(batcher.clone()),
            realm_config_loader: Loader::new(batcher.clone()),
            token_owner_record_loader: Loader::new(batcher.clone()),
            approve_vote_choices_loader: Loader::new(batcher.clone()),
            proposal_loader: Loader::new(batcher.clone()),
            proposal_options_loader: Loader::new(batcher.clone()),
            proposal_multi_choice_loader: Loader::new(batcher.clone()),
            vote_record_token_owner_loader: Loader::new(batcher.clone()),
            twitter_handle_loader: Loader::new(batcher),

            twitter_profile_loader: Loader::new(twitter_batcher),
        }
    }

    #[inline]
    pub(crate) async fn wallet(&self, address: PublicKey<Wallet>) -> Result<Wallet> {
        let handle = self.twitter_handle_loader.load(address.clone()).await?;
        Ok(Wallet::new(address, handle))
    }
}
