use crate::globals::*;
use crate::parse_cli::*;

#[derive(Default, Debug, PartialEq)]
pub struct Args {
    pub cmd_daemon: bool,
    pub arg_daemon_pid_file: Option<String>,

    pub cmd_account: bool,
    pub cmd_account_new: bool,
    pub cmd_account_list: bool,
    pub cmd_account_import: bool,
    pub arg_account_import_path: Option<Vec<String>>,

    pub cmd_wallet: bool,
    pub cmd_wallet_import: bool,
    pub arg_wallet_import_path: Option<String>,

    pub cmd_import: bool,
    pub arg_import_format: Option<String>,
    pub arg_import_file: Option<String>,

    pub cmd_export: bool,
    pub cmd_export_blocks: bool,
    pub arg_export_blocks_format: Option<String>,
    pub arg_export_blocks_from: String,
    pub arg_export_blocks_to: String,
    pub arg_export_blocks_file: Option<String>,

    pub cmd_export_state: bool,
    pub flag_export_state_no_storage: bool,
    pub flag_export_state_no_code: bool,
    pub arg_export_state_min_balance: Option<String>,
    pub arg_export_state_max_balance: Option<String>,
    pub arg_export_state_at: String,
    pub arg_export_state_format: Option<String>,
    pub arg_export_state_file: Option<String>,

    pub cmd_signer: bool,
    pub cmd_signer_new_token: bool,
    pub cmd_signer_list: bool,
    pub cmd_signer_sign: bool,
    pub arg_signer_sign_id: Option<usize>,
    pub cmd_signer_reject: bool,
    pub arg_signer_reject_id: Option<usize>,

    pub cmd_snapshot: bool,
    pub arg_snapshot_at: String,
    pub arg_snapshot_file: Option<String>,

    pub cmd_restore: bool,
    pub arg_restore_file: Option<String>,

    pub cmd_tools: bool,
    pub cmd_tools_hash: bool,
    pub arg_tools_hash_file: Option<String>,

    pub cmd_db: bool,
    pub cmd_db_kill: bool,
    pub cmd_db_reset: bool,
    pub arg_db_reset_num: u32,

    pub cmd_export_hardcoded_sync: bool,

    pub cmd_dapp: bool,
    pub arg_dapp_path: Option<String>,

    pub flag_no_download: bool,
    pub flag_no_consensus: bool,
    pub flag_light: bool,
    pub flag_no_hardcoded_sync: bool,
    pub flag_force_direct: bool,
    pub arg_mode: String,
    pub arg_mode_timeout: u64,
    pub arg_mode_alarm: u64,
    pub arg_auto_update: String,
    pub arg_auto_update_delay: u16,
    pub arg_auto_update_check_frequency: u16,
    pub arg_release_track: String,
    pub arg_chain: String,
    pub arg_keys_path: String,
    pub arg_identity: String,
    pub arg_base_path: Option<String>,
    pub arg_db_path: Option<String>,
    pub flag_unsafe_expose: bool,
    pub arg_config: String,
    pub arg_ports_shift: u16,
    pub flag_fast_unlock: bool,
    pub arg_keys_iterations: u32,
    pub arg_accounts_refresh: u64,
    pub arg_unlock: Option<String>,
    pub arg_enable_signing_queue: bool,
    pub arg_password: Vec<String>,
    pub flag_private_enabled: bool,
    pub flag_private_state_offchain: bool,
    pub arg_private_signer: Option<String>,
    pub arg_private_validators: Option<String>,
    pub arg_private_account: Option<String>,
    pub arg_private_sstore_url: Option<String>,
    pub arg_private_sstore_threshold: Option<u32>,
    pub arg_private_passwords: Option<String>,
    pub arg_ui_path: String,
    pub flag_no_warp: bool,
    pub flag_no_discovery: bool,
    pub flag_reserved_only: bool,
    pub flag_no_ancient_blocks: bool,
    pub flag_no_serve_light: bool,
    pub arg_warp_barrier: Option<u64>,
    pub arg_port: u16,
    pub arg_interface: String,
    pub arg_min_peers: Option<u16>,
    pub arg_max_peers: Option<u16>,
    pub arg_snapshot_peers: u16,
    pub arg_nat: String,
    pub arg_allow_ips: String,
    pub arg_max_pending_peers: u16,
    pub arg_network_id: Option<u64>,
    pub arg_bootnodes: Option<String>,
    pub arg_node_key: Option<String>,
    pub arg_reserved_peers: Option<String>,
    pub flag_jsonrpc_allow_missing_blocks: bool,
    pub flag_no_jsonrpc: bool,
    pub flag_jsonrpc_no_keep_alive: bool,
    pub flag_jsonrpc_experimental: bool,
    pub arg_jsonrpc_port: u16,
    pub arg_jsonrpc_interface: String,
    pub arg_jsonrpc_apis: String,
    pub arg_jsonrpc_hosts: String,
    pub arg_jsonrpc_threads: Option<usize>,
    pub arg_jsonrpc_server_threads: Option<usize>,
    pub arg_jsonrpc_cors: String,
    pub arg_jsonrpc_max_payload: Option<usize>,
    pub arg_poll_lifetime: u32,
    pub flag_no_ws: bool,
    pub arg_ws_port: u16,
    pub arg_ws_interface: String,
    pub arg_ws_apis: String,
    pub arg_ws_origins: String,
    pub arg_ws_hosts: String,
    pub arg_ws_max_connections: usize,
    pub flag_no_ipc: bool,
    pub arg_ipc_path: String,
    pub arg_ipc_chmod: String,
    pub arg_ipc_apis: String,
    pub arg_on_demand_response_time_window: Option<u64>,
    pub arg_on_demand_request_backoff_start: Option<u64>,
    pub arg_on_demand_request_backoff_max: Option<u64>,
    pub arg_on_demand_request_backoff_rounds_max: Option<usize>,
    pub arg_on_demand_request_consecutive_failures: Option<usize>,
    pub flag_no_secretstore: bool,
    pub flag_no_secretstore_http: bool,
    pub flag_no_secretstore_auto_migrate: bool,
    pub arg_secretstore_http_cors: String,
    pub arg_secretstore_acl_contract: Option<String>,
    pub arg_secretstore_contract: Option<String>,
    pub arg_secretstore_srv_gen_contract: Option<String>,
    pub arg_secretstore_srv_retr_contract: Option<String>,
    pub arg_secretstore_doc_store_contract: Option<String>,
    pub arg_secretstore_doc_sretr_contract: Option<String>,
    pub arg_secretstore_nodes: String,
    pub arg_secretstore_server_set_contract: Option<String>,
    pub arg_secretstore_interface: String,
    pub arg_secretstore_port: u16,
    pub arg_secretstore_http_interface: String,
    pub arg_secretstore_http_port: u16,
    pub arg_secretstore_path: String,
    pub arg_secretstore_secret: Option<String>,
    pub arg_secretstore_admin_public: Option<String>,
    pub flag_force_sealing: bool,
    pub flag_reseal_on_uncle: bool,
    pub flag_remove_solved: bool,
    pub flag_tx_queue_no_unfamiliar_locals: bool,
    pub flag_tx_queue_no_early_reject: bool,
    pub flag_refuse_service_transactions: bool,
    pub flag_infinite_pending_block: bool,
    pub flag_no_persistent_txqueue: bool,
    pub flag_stratum: bool,
    pub arg_reseal_on_txs: String,
    pub arg_reseal_min_period: u64,
    pub arg_reseal_max_period: u64,
    pub arg_work_queue_size: usize,
    pub arg_relay_set: String,
    pub arg_usd_per_tx: String,
    pub arg_usd_per_eth: String,
    pub arg_price_update_period: String,
    pub arg_gas_floor_target: String,
    pub arg_gas_cap: String,
    pub arg_tx_queue_mem_limit: u32,
    pub arg_tx_queue_size: usize,
    pub arg_tx_queue_per_sender: Option<usize>,
    pub arg_tx_queue_locals: Option<String>,
    pub arg_tx_queue_strategy: String,
    pub arg_stratum_interface: String,
    pub arg_stratum_port: u16,
    pub arg_min_gas_price: Option<u64>,
    pub arg_gas_price_percentile: usize,
    pub arg_author: Option<String>,
    pub arg_engine_signer: Option<String>,
    pub arg_tx_gas_limit: Option<String>,
    pub arg_tx_time_limit: Option<u64>,
    pub arg_extra_data: Option<String>,
    pub arg_notify_work: Option<String>,
    pub arg_stratum_secret: Option<String>,
    pub arg_max_round_blocks_to_import: usize,
    pub flag_can_restart: bool,
    pub flag_no_color: bool,
    pub flag_version: bool,
    pub flag_no_config: bool,
    pub arg_logging: Option<String>,
    pub arg_log_file: Option<String>,
    pub flag_scale_verifiers: bool,
    pub arg_tracing: String,
    pub arg_pruning: String,
    pub arg_pruning_history: u64,
    pub arg_pruning_memory: usize,
    pub arg_cache_size_db: u32,
    pub arg_cache_size_blocks: u32,
    pub arg_cache_size_queue: u32,
    pub arg_cache_size_state: u32,
    pub arg_db_compaction: String,
    pub arg_fat_db: String,
    pub arg_cache_size: Option<u32>,
    pub arg_num_verifiers: Option<usize>,
    pub flag_no_seal_check: bool,
    pub flag_no_periodic_snapshot: bool,
    pub arg_snapshot_threads: Option<usize>,
    pub flag_whisper: bool,
    pub arg_whisper_pool_size: Option<usize>,
    pub flag_geth: bool,
    pub flag_import_geth_keys: bool,
}

impl Args {
    pub fn from_cli(&mut self, cli_args: ArgsInput) {
        self.from_subcommands(cli_args.clone());
        self.from_globals(cli_args);
    }

    pub fn from_subcommands(&mut self, cli_args: ArgsInput) {}

    pub fn from_globals(&mut self, cli_args: ArgsInput) {
        self.flag_no_download = cli_args.globals.operating.no_download;
        self.flag_no_consensus = cli_args.globals.operating.no_consensus;
        self.flag_light = cli_args.globals.operating.light;
        self.flag_no_hardcoded_sync = cli_args.globals.operating.light;
        self.flag_force_direct = cli_args.globals.operating.force_direct;
        self.arg_mode = cli_args.globals.operating.mode;
        self.arg_mode_timeout = cli_args.globals.operating.mode_timeout;
        self.arg_mode_alarm = cli_args.globals.operating.mode_alarm;
        self.arg_auto_update = cli_args.globals.operating.auto_update;
        self.arg_auto_update_delay = cli_args.globals.operating.auto_update_delay;
        self.arg_auto_update_check_frequency =
            cli_args.globals.operating.auto_update_check_frequency;
        self.arg_release_track = cli_args.globals.operating.release_track;
        self.arg_chain = cli_args.globals.operating.chain;
        self.arg_keys_path = cli_args.globals.operating.keys_path;
        self.arg_identity = cli_args.globals.operating.identity;
        self.arg_base_path = cli_args.globals.operating.base_path;
        self.arg_db_path = cli_args.globals.operating.db_path;
        self.flag_unsafe_expose = cli_args.globals.convenience.unsafe_expose;
        self.arg_config = cli_args.globals.convenience.config;
        self.arg_ports_shift = cli_args.globals.convenience.ports_shift;
        self.flag_fast_unlock = cli_args.globals.account.fast_unlock;
        self.arg_keys_iterations = cli_args.globals.account.keys_iterations;
        self.arg_accounts_refresh = cli_args.globals.account.accounts_refresh;
        self.arg_unlock = cli_args.globals.account.unlock;
        self.arg_enable_signing_queue = cli_args.globals.account.enable_signing_queue;
        self.arg_password = cli_args.globals.account.password;
        self.flag_private_enabled = cli_args.globals.private_transactions.private_enabled;
        self.flag_private_state_offchain =
            cli_args.globals.private_transactions.private_state_offchain;
        self.arg_private_signer = cli_args.globals.private_transactions.private_signer;
        self.arg_private_validators = cli_args.globals.private_transactions.private_validators;
        self.arg_private_account = cli_args.globals.private_transactions.private_account;
        self.arg_private_sstore_url = cli_args.globals.private_transactions.private_sstore_url;
        self.arg_private_sstore_threshold = cli_args
            .globals
            .private_transactions
            .private_sstore_threshold;
        self.arg_private_passwords = cli_args.globals.private_transactions.private_passwords;
        self.arg_ui_path = cli_args.globals.ui_options.ui_path;
        self.flag_no_warp = cli_args.globals.networking.no_warp;
        self.flag_no_discovery = cli_args.globals.networking.no_discovery;
        self.flag_reserved_only = cli_args.globals.networking.reserved_only;
        self.flag_no_ancient_blocks = cli_args.globals.networking.no_ancient_blocks;
        self.flag_no_serve_light = cli_args.globals.networking.no_serve_light;
        self.arg_warp_barrier = cli_args.globals.networking.warp_barrier;
        self.arg_port = cli_args.globals.networking.port;
        self.arg_interface = cli_args.globals.networking.interface;
        self.arg_min_peers = cli_args.globals.networking.min_peers;
        self.arg_max_peers = cli_args.globals.networking.max_peers;
        self.arg_snapshot_peers = cli_args.globals.networking.snapshot_peers;
        self.arg_nat = cli_args.globals.networking.nat;
        self.arg_allow_ips = cli_args.globals.networking.allow_ips;
        self.arg_max_pending_peers = cli_args.globals.networking.max_pending_peers;
        self.arg_network_id = cli_args.globals.networking.network_id;
        self.arg_bootnodes = cli_args.globals.networking.bootnodes;
        self.arg_node_key = cli_args.globals.networking.node_key;
        self.arg_reserved_peers = cli_args.globals.networking.reserved_peers;
        self.flag_jsonrpc_allow_missing_blocks =
            cli_args.globals.http_json_rpc.jsonrpc_allow_missing_blocks;
        self.flag_no_jsonrpc = cli_args.globals.http_json_rpc.no_jsonrpc;
        self.flag_jsonrpc_no_keep_alive = cli_args.globals.http_json_rpc.jsonrpc_no_keep_alive;
        self.flag_jsonrpc_experimental = cli_args.globals.http_json_rpc.jsonrpc_experimental;
        self.arg_jsonrpc_port = cli_args.globals.http_json_rpc.jsonrpc_port;
        self.arg_jsonrpc_interface = cli_args.globals.http_json_rpc.jsonrpc_interface;
        self.arg_jsonrpc_apis = cli_args.globals.http_json_rpc.jsonrpc_apis;
        self.arg_jsonrpc_hosts = cli_args.globals.http_json_rpc.jsonrpc_hosts;
        self.arg_jsonrpc_threads = cli_args.globals.http_json_rpc.jsonrpc_threads;
        self.arg_jsonrpc_server_threads =
            Some(cli_args.globals.http_json_rpc.jsonrpc_server_threads);
        self.arg_jsonrpc_cors = cli_args.globals.http_json_rpc.jsonrpc_cors;
        self.arg_jsonrpc_max_payload = cli_args.globals.http_json_rpc.jsonrpc_max_payload;
        self.arg_poll_lifetime = cli_args.globals.http_json_rpc.poll_lifetime;
        self.flag_no_ws = cli_args.globals.websockets.no_ws;
        self.arg_ws_port = cli_args.globals.websockets.ws_port;
        self.arg_ws_interface = cli_args.globals.websockets.ws_interface;
        self.arg_ws_apis = cli_args.globals.websockets.ws_apis;
        self.arg_ws_origins = cli_args.globals.websockets.ws_origins;
        self.arg_ws_hosts = cli_args.globals.websockets.ws_hosts;
        self.arg_ws_max_connections = cli_args.globals.websockets.ws_max_connections;
        self.flag_no_ipc = cli_args.globals.ipc.no_ipc;
        self.arg_ipc_path = cli_args.globals.ipc.ipc_path;
        self.arg_ipc_chmod = cli_args.globals.ipc.ipc_chmod;
        self.arg_ipc_apis = cli_args.globals.ipc.ipc_apis;
        self.arg_on_demand_response_time_window =
            cli_args.globals.light_client.on_demand_response_time_window;
        self.arg_on_demand_request_backoff_start = cli_args
            .globals
            .light_client
            .on_demand_request_backoff_start;
        self.arg_on_demand_request_backoff_max =
            cli_args.globals.light_client.on_demand_request_backoff_max;
        self.arg_on_demand_request_backoff_rounds_max = cli_args
            .globals
            .light_client
            .on_demand_request_backoff_rounds_max;
        self.arg_on_demand_request_consecutive_failures = cli_args
            .globals
            .light_client
            .on_demand_request_consecutive_failures;
        self.flag_no_secretstore = cli_args.globals.secret_store.no_secretstore;
        self.flag_no_secretstore_http = cli_args.globals.secret_store.no_secretstore_http;
        self.flag_no_secretstore_auto_migrate =
            cli_args.globals.secret_store.no_secretstore_auto_migrate;
        self.arg_secretstore_http_cors = cli_args.globals.secret_store.secretstore_http_cors;
        self.arg_secretstore_acl_contract =
            Some(cli_args.globals.secret_store.secretstore_acl_contract);
        self.arg_secretstore_contract = cli_args.globals.secret_store.secretstore_contract;
        self.arg_secretstore_srv_gen_contract =
            cli_args.globals.secret_store.secretstore_srv_gen_contract;
        self.arg_secretstore_srv_retr_contract =
            cli_args.globals.secret_store.secretstore_srv_retr_contract;
        self.arg_secretstore_doc_store_contract =
            cli_args.globals.secret_store.secretstore_doc_store_contract;
        self.arg_secretstore_doc_sretr_contract =
            cli_args.globals.secret_store.secretstore_doc_sretr_contract;
        self.arg_secretstore_nodes = cli_args.globals.secret_store.secretstore_nodes;
        self.arg_secretstore_server_set_contract = Some(
            cli_args
                .globals
                .secret_store
                .secretstore_server_set_contract,
        );
        self.arg_secretstore_interface = cli_args.globals.secret_store.secretstore_interface;
        self.arg_secretstore_port = cli_args.globals.secret_store.secretstore_port;
        self.arg_secretstore_http_interface =
            cli_args.globals.secret_store.secretstore_http_interface;
        self.arg_secretstore_http_port = cli_args.globals.secret_store.secretstore_http_port;
        self.arg_secretstore_path = cli_args.globals.secret_store.secretstore_path;
        self.arg_secretstore_secret = cli_args.globals.secret_store.secretstore_secret;
        self.arg_secretstore_admin_public = cli_args.globals.secret_store.secretstore_admin_public;
        self.flag_force_sealing = cli_args.globals.sealing_mining.force_sealing;
        self.flag_reseal_on_uncle = cli_args.globals.sealing_mining.reseal_on_uncle;
        self.flag_remove_solved = cli_args.globals.sealing_mining.remove_solved;
        self.flag_tx_queue_no_unfamiliar_locals = cli_args
            .globals
            .sealing_mining
            .tx_queue_no_unfamiliar_locals;
        self.flag_tx_queue_no_early_reject =
            cli_args.globals.sealing_mining.tx_queue_no_early_reject;
        self.flag_refuse_service_transactions =
            cli_args.globals.sealing_mining.refuse_service_transactions;
        self.flag_infinite_pending_block = cli_args.globals.sealing_mining.infinite_pending_block;
        self.flag_no_persistent_txqueue = cli_args.globals.sealing_mining.no_persistent_txqueue;
        self.flag_stratum = cli_args.globals.sealing_mining.stratum;
        self.arg_reseal_on_txs = cli_args.globals.sealing_mining.reseal_on_txs;
        self.arg_reseal_min_period = cli_args.globals.sealing_mining.reseal_min_period;
        self.arg_reseal_max_period = cli_args.globals.sealing_mining.reseal_max_period;
        self.arg_work_queue_size = cli_args.globals.sealing_mining.work_queue_size;
        self.arg_relay_set = cli_args.globals.sealing_mining.relay_set;
        self.arg_usd_per_tx = cli_args.globals.sealing_mining.usd_per_tx;
        self.arg_usd_per_eth = cli_args.globals.sealing_mining.usd_per_eth;
        self.arg_price_update_period = cli_args.globals.sealing_mining.price_update_period;
        self.arg_gas_floor_target = cli_args.globals.sealing_mining.gas_floor_target;
        self.arg_gas_cap = cli_args.globals.sealing_mining.gas_cap;
        self.arg_tx_queue_mem_limit = cli_args.globals.sealing_mining.tx_queue_mem_limit;
        self.arg_tx_queue_size = cli_args.globals.sealing_mining.tx_queue_size;
        self.arg_tx_queue_per_sender = cli_args.globals.sealing_mining.tx_queue_per_sender;
        self.arg_tx_queue_locals = cli_args.globals.sealing_mining.tx_queue_locals;
        self.arg_tx_queue_strategy = cli_args.globals.sealing_mining.tx_queue_strategy;
        self.arg_stratum_interface = cli_args.globals.sealing_mining.stratum_interface;
        self.arg_stratum_port = cli_args.globals.sealing_mining.stratum_port;
        self.arg_min_gas_price = cli_args.globals.sealing_mining.min_gas_price;
        self.arg_gas_price_percentile = cli_args.globals.sealing_mining.gas_price_percentile;
        self.arg_author = cli_args.globals.sealing_mining.author;
        self.arg_engine_signer = cli_args.globals.sealing_mining.engine_signer;
        self.arg_tx_gas_limit = cli_args.globals.sealing_mining.tx_gas_limit;
        self.arg_tx_time_limit = cli_args.globals.sealing_mining.tx_time_limit;
        self.arg_extra_data = cli_args.globals.sealing_mining.extra_data;
        self.arg_notify_work = cli_args.globals.sealing_mining.notify_work;
        self.arg_stratum_secret = cli_args.globals.sealing_mining.stratum_secret;
        self.arg_max_round_blocks_to_import =
            cli_args.globals.sealing_mining.max_round_blocks_to_import;
        self.flag_can_restart = cli_args.globals.internal.can_restart;
        self.flag_no_color = cli_args.globals.miscellaneous.no_color;
        self.flag_no_config = cli_args.globals.miscellaneous.no_config;
        self.arg_logging = cli_args.globals.miscellaneous.logging;
        self.arg_log_file = cli_args.globals.miscellaneous.log_file;
        self.flag_scale_verifiers = cli_args.globals.footprint.scale_verifiers;
        self.arg_tracing = cli_args.globals.footprint.tracing;
        self.arg_pruning = cli_args.globals.footprint.pruning;
        self.arg_pruning_history = cli_args.globals.footprint.pruning_history;
        self.arg_pruning_memory = cli_args.globals.footprint.pruning_memory;
        self.arg_cache_size_db = cli_args.globals.footprint.cache_size_db;
        self.arg_cache_size_blocks = cli_args.globals.footprint.cache_size_blocks;
        self.arg_cache_size_queue = cli_args.globals.footprint.cache_size_queue;
        self.arg_cache_size_state = cli_args.globals.footprint.cache_size_state;
        self.arg_db_compaction = cli_args.globals.footprint.db_compaction;
        self.arg_fat_db = cli_args.globals.footprint.fat_db;
        self.arg_cache_size = cli_args.globals.footprint.cache_size;
        self.arg_num_verifiers = cli_args.globals.footprint.num_verifiers;
        self.flag_no_seal_check = cli_args.globals.import_export.no_seal_check;
        self.flag_no_periodic_snapshot = cli_args.globals.snapshot.no_periodic_snapshot;
        self.arg_snapshot_threads = cli_args.globals.snapshot.snapshot_threads;
        self.flag_geth = cli_args.globals.legacy.geth;
        self.flag_import_geth_keys = cli_args.globals.legacy.import_geth_keys;
    }
}
