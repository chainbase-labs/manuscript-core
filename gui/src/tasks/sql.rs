pub const SOLANA_INIT_SQL: &str = r#"
CREATE TABLE sol_pool_infos (
                                id BIGSERIAL PRIMARY KEY,
                                dex_type VARCHAR(255) NOT NULL DEFAULT 'raydium',
                                lp_address VARCHAR(255) NOT NULL,
                                pool_address VARCHAR(255) NOT NULL,
                                base_address VARCHAR(255) NOT NULL,
                                base_symbol VARCHAR(255),
                                base_name VARCHAR(255),
                                quote_address VARCHAR(255) NOT NULL,
                                quote_symbol VARCHAR(255),
                                quote_name VARCHAR(255),
                                base_decimals INTEGER,
                                quote_decimals INTEGER,
                                market_cap NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                liquidity NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                base_reserve NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                quote_reserve NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                initial_liquidity NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                initial_base_reserve NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                initial_quote_reserve NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                creation_timestamp BIGINT,
                                base_reserve_value NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                quote_reserve_value NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                quote_vault_address VARCHAR(255),
                                base_vault_address VARCHAR(255),
                                logo_uri TEXT,
                                meta_data_ipfs_uri VARCHAR(255),
                                creator VARCHAR(255),
                                is_pump_fun_official BOOLEAN NOT NULL DEFAULT FALSE,
                                is_reversed BOOLEAN NOT NULL DEFAULT FALSE,
                                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_lq ON sol_pool_infos (liquidity);
CREATE INDEX idx_creator ON sol_pool_infos (creator);
CREATE INDEX idx_pool_address ON sol_pool_infos (pool_address);
CREATE INDEX idx_base_quote ON sol_pool_infos (base_address, quote_address);
CREATE INDEX idx_quote_address ON sol_pool_infos (quote_address);
CREATE INDEX idx_quote_symbol ON sol_pool_infos (quote_symbol);
CREATE INDEX idx_quote_name ON sol_pool_infos (quote_name);

CREATE TABLE sol_token_infos (
                                 id BIGSERIAL PRIMARY KEY,
                                 address VARCHAR(255) NOT NULL,
                                 token_name VARCHAR(255) NOT NULL DEFAULT '',
                                 symbol VARCHAR(255) NOT NULL DEFAULT '',
                                 decimals BIGINT NOT NULL,
                                 creator VARCHAR(255),
                                 creation_timestamp BIGINT,
                                 logo_uri TEXT,
                                 meta_data_ipfs_uri TEXT,
                                 biggest_pool_address VARCHAR(255),
                                 open_timestamp BIGINT NOT NULL DEFAULT 0,
                                 holder_count BIGINT NOT NULL DEFAULT 0,
                                 circulating_supply BIGINT NOT NULL DEFAULT 0,
                                 total_supply BIGINT NOT NULL DEFAULT 1000000000,
                                 max_supply BIGINT NOT NULL DEFAULT 1000000000,
                                 tag VARCHAR(255),
                                 is_new BOOLEAN NOT NULL DEFAULT FALSE,
                                 mint_authority BOOLEAN NOT NULL DEFAULT FALSE,
                                 freeze_authority BOOLEAN NOT NULL DEFAULT FALSE,
                                 bonding_curve VARCHAR(255),
                                 associated_bonding_curve VARCHAR(255),
                                 is_ipfs_migration BOOLEAN NOT NULL DEFAULT FALSE,
                                 is_pump_fun BOOLEAN NOT NULL DEFAULT FALSE,
                                 created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                 updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_sol_token_infos_address ON sol_token_infos (address);
CREATE INDEX idx_soltokeninfo_address_token_name ON sol_token_infos (address, token_name);
CREATE INDEX idx_soltokeninfo_created_at_id ON sol_token_infos (created_at, id);
CREATE INDEX idx_is_ipfs_migration_index ON sol_token_infos (is_ipfs_migration);

CREATE TABLE sol_swap_events (
                                 id BIGSERIAL PRIMARY KEY,
                                 dex_type VARCHAR(255) NOT NULL DEFAULT 'raydium',
                                 event_type VARCHAR(255) NOT NULL DEFAULT 'swap',
                                 event_type_id SMALLINT NOT NULL DEFAULT 0,
                                 tx_hash VARCHAR(255) NOT NULL,
                                 slot_number BIGINT,
                                 tx_index BIGINT NOT NULL DEFAULT 0,
                                 event_index BIGINT NOT NULL DEFAULT 0,
                                 pool_address VARCHAR(255) NOT NULL,
                                 base_address VARCHAR(255) NOT NULL,
                                 quote_address VARCHAR(255) NOT NULL,
                                 base_amount_str VARCHAR(255) NOT NULL,
                                 quote_amount_str VARCHAR(255) NOT NULL,
                                 base_amount BIGINT NOT NULL DEFAULT 0,
                                 quote_amount BIGINT NOT NULL DEFAULT 0,
                                 maker_address VARCHAR(255),
                                 maker_tags VARCHAR(255),
                                 event_time BIGINT NOT NULL DEFAULT 0,
                                 virtual_sol_reserves BIGINT,
                                 virtual_token_reserves BIGINT,
                                 real_sol_reserves BIGINT,
                                 real_token_reserves BIGINT,
                                 is_buy BOOLEAN NOT NULL DEFAULT FALSE,
                                 amount_usd NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                 price_usd NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                 price_quote NUMERIC(65, 18) NOT NULL DEFAULT 0,
                                 created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                 updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_covering ON sol_swap_events (pool_address, slot_number, tx_index, event_index, price_usd);
CREATE INDEX idx_created_at ON sol_swap_events (created_at);
CREATE INDEX idx_et_ei ON sol_swap_events (event_time, event_index);
CREATE INDEX idx_event_type ON sol_swap_events (event_type);
CREATE INDEX idx_event_time_pool_address ON sol_swap_events (event_time, pool_address);
CREATE INDEX idx_monitor_combined ON sol_swap_events (maker_address, slot_number DESC, tx_index DESC, event_index DESC);
CREATE INDEX idx_maker_address ON sol_swap_events (maker_address);
CREATE INDEX idx_quote ON sol_swap_events (quote_address);
CREATE INDEX idx_order_slot_tx_event ON sol_swap_events (slot_number DESC, tx_index DESC, event_index DESC);
"#;