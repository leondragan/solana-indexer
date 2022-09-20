ALTER TABLE COLLECTIONS_VOLUME
    ADD COLUMN _1D_FLOOR_PRICE NUMERIC DEFAULT 0,
    ADD COLUMN PREV_1D_FLOOR_PRICE NUMERIC DEFAULT 0,
    ADD COLUMN _1D_SALES_COUNT NUMERIC DEFAULT 0,
    ADD COLUMN PREV_1D_SALES_COUNT NUMERIC DEFAULT 0,
    ADD COLUMN _7D_FLOOR_PRICE NUMERIC DEFAULT 0,
    ADD COLUMN PREV_7D_FLOOR_PRICE NUMERIC DEFAULT 0,
    ADD COLUMN _7D_SALES_COUNT NUMERIC DEFAULT 0,
    ADD COLUMN PREV_7D_SALES_COUNT NUMERIC DEFAULT 0,
    ADD COLUMN _30D_FLOOR_PRICE NUMERIC DEFAULT 0,
    ADD COLUMN PREV_30D_FLOOR_PRICE NUMERIC DEFAULT 0,
    ADD COLUMN _30D_SALES_COUNT NUMERIC DEFAULT 0,
    ADD COLUMN PREV_30D_SALES_COUNT NUMERIC DEFAULT 0;

ALTER TABLE COLLECTIONS_VOLUME RENAME TO COLLECTION_TRENDS;

UPDATE COLLECTION_TRENDS SET _1D_VOLUME = 0 WHERE _1D_VOLUME IS NULL;
UPDATE COLLECTION_TRENDS SET _7D_VOLUME = 0 WHERE _7D_VOLUME IS NULL;
UPDATE COLLECTION_TRENDS SET _30D_VOLUME = 0 WHERE _30D_VOLUME IS NULL;
UPDATE COLLECTION_TRENDS SET _PREV_1D_VOLUME = 0 WHERE _PREV_1D_VOLUME IS NULL;
UPDATE COLLECTION_TRENDS SET _PREV_7D_VOLUME = 0 WHERE _PREV_7D_VOLUME IS NULL;
UPDATE COLLECTION_TRENDS SET _PREV_30D_VOLUME = 0 WHERE _PREV_30D_VOLUME IS NULL;

ALTER TABLE COLLECTION_TRENDS ALTER COLUMN _1D_VOLUME SET DEFAULT 0,
    ALTER COLUMN _7D_VOLUME SET DEFAULT 0,
    ALTER COLUMN _1D_VOLUME SET DEFAULT 0,
    ALTER COLUMN _30D_VOLUME SET DEFAULT 0,
    ALTER COLUMN _PREV_1D_VOLUME SET DEFAULT 0,
    ALTER COLUMN _PREV_7D_VOLUME SET DEFAULT 0,
    ALTER COLUMN _PREV_30D_VOLUME SET DEFAULT 0;

ALTER TABLE COLLECTION_TRENDS
    ADD COLUMN _1d_volume_change BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_1D_VOLUME,0) - NULLIF(_PREV_1D_VOLUME,0) ) / NULLIF(_PREV_1D_VOLUME,0), 0)) 
	STORED,
    ADD COLUMN _7d_volume_change BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_7D_VOLUME,0) - NULLIF(_PREV_7D_VOLUME,0) ) / NULLIF(_PREV_7D_VOLUME,0), 0)) 
	STORED,
    ADD COLUMN _30d_volume_change BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_30D_VOLUME,0) - NULLIF(_PREV_30D_VOLUME,0) ) / NULLIF(_PREV_30D_VOLUME,0), 0))
	STORED,
    ADD COLUMN _1d_floor_price_change BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_1D_FLOOR_PRICE,0) - NULLIF(PREV_1D_FLOOR_PRICE,0) ) / NULLIF(PREV_1D_FLOOR_PRICE,0), 0) ) 
	STORED,
    ADD COLUMN _7d_floor_price_change BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_7D_FLOOR_PRICE,0) - NULLIF(_PREV_7D_VOLUME,0) ) / NULLIF(_PREV_7D_VOLUME,0), 0)) 
	STORED,
    ADD COLUMN _30d_floor_price_change BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_30D_FLOOR_PRICE,0) - NULLIF(PREV_30D_FLOOR_PRICE,0) ) / NULLIF(PREV_30D_FLOOR_PRICE,0), 0)) 
	STORED,
    ADD COLUMN _1D_SALES_COUNT_CHANGE BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_1D_SALES_COUNT,0) - NULLIF(PREV_1D_SALES_COUNT,0) ) / NULLIF(PREV_1D_SALES_COUNT,0), 0)) 
	STORED,
    ADD COLUMN _7D_SALES_COUNT_CHANGE BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_7D_SALES_COUNT,0) - NULLIF(PREV_7D_SALES_COUNT,0) ) / NULLIF(PREV_7D_SALES_COUNT,0), 0)) 
	STORED,
    ADD COLUMN _30D_SALES_COUNT_CHANGE BIGINT GENERATED ALWAYS AS 
    (coalesce(100*( NULLIF(_30D_SALES_COUNT,0) - NULLIF(PREV_30D_SALES_COUNT,0) ) / NULLIF(PREV_30D_SALES_COUNT,0), 0)) 
	STORED;