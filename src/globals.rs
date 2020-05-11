use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Globals {
    #[structopt(flatten)]
    operating: OperatingOptions,

    #[structopt(flatten)]
    convenience: ConvenienceOptions,

    #[structopt(flatten)]
    account: AccountOptions,

    #[structopt(flatten)]
    private_transactions: PrivateTransactions,

    #[structopt(flatten)]
    ui_options: UIOptons,

    #[structopt(flatten)]
    networking: NetworkingOptions,

    #[structopt(flatten)]
    ipc: IPCOptions,

    #[structopt(flatten)]
    http_json_rpc: HTTP_JSON_RPC_Options,

    #[structopt(flatten)]
    light_client: LightClientOptions,

    #[structopt(flatten)]
    websockets: WebsocketsOptions,

    #[structopt(flatten)]
    secret_store: SecretStoreOptions,

    #[structopt(flatten)]
    sealing_mining: SealingMiningOptions,

    #[structopt(flatten)]
    internal: InternalOptions,

    #[structopt(flatten)]
    miscellaneous: MiscellaneousOptions,

    #[structopt(flatten)]
    footprint: FootPrintOptions,

    #[structopt(flatten)]
    import_export: ImportExportOptions,

    #[structopt(flatten)]
    snapshot: SnapshotOptions,

    #[structopt(flatten)]
    legacy: LegacyOptions,
}

// TODO: Implement the default values
#[derive(StructOpt, Debug)]
pub struct OperatingOptions {
    #[structopt(
        long = "no-download",
        help = "Normally new releases will be downloaded ready for updating. This disables it. Not recommended."
    )]
    no_download: bool,
    #[structopt(
        long = "no-consensus",
        help = "Force the binary to run even if there are known issues regarding consensus. Not recommended."
    )]
    no_consensus: bool,

    #[structopt(
        long,
        help = "Experimental: run in light client mode. Light clients synchronize a bare minimum of data and fetch necessary data on-demand from the network. Much lower in storage, potentially higher in bandwidth. Has no effect with subcommands."
    )]
    light: bool,

    #[structopt(
        long = "no-hardcoded-sync",
        help = "By default, if there is no existing database the light client will automatically jump to a block hardcoded in the chain's specifications. This disables this feature."
    )]
    no_hardcoded_sync: bool,

    #[structopt(
        long = "force-direct",
        help = "Run the originally installed version of Parity, ignoring any updates that have since been installed."
    )]
    force_direct: bool,

    #[structopt(
        default_value = "last",
        name = "MODE",
        long,
        help = "Set the operating mode. MODE can be one of: last - Uses the last-used mode, active if none; active - Parity continuously syncs the chain; passive - Parity syncs initially, then sleeps and wakes regularly to resync; dark - Parity syncs only when the JSON-RPC is active; offline - Parity doesn't sync."
    )]
    mode: String,

    #[structopt(
        default_value = "300",
        long = "mode-timeout",
        name = "TIMEOUT_IN_SECS",
        help = "Specify the number of seconds before inactivity timeout occurs when mode is dark or passive"
    )]
    mode_timeout: u64,

    #[structopt(
        long = "mode-alarm",
        default_value = "3600",
        name = "ALARM_IN_SECS",
        help = "Specify the number of seconds before auto sleep reawake timeout occurs when mode is passive"
    )]
    mode_alarm: u64,

    #[structopt(
        long = "auto-update",
        default_value = "critical",
        name = "SET",
        help = "Set a releases set to automatically update and install. SET can be one of: all - All updates in the our release track; critical - Only consensus/security updates; none - No updates will be auto-installed."
    )]
    auto_update: String,

    #[structopt(
        long = "auto-update-delay",
        default_value = "100",
        name = "DELAY_NUM",
        help = "Specify the maximum number of blocks used for randomly delaying updates."
    )]
    auto_update_delay: u16,

    #[structopt(
        long = "auto-update-check-frequency",
        default_value = "20",
        name = "FREQUENCY_NUM",
        help = "Specify the number of blocks between each auto-update check."
    )]
    auto_update_check_frequency: u16,

    #[structopt(
        long = "release-track",
        default_value = "current",
        name = "TRACK",
        help = "Set which release track we should use for updates. TRACK can be one of: stable - Stable releases; nightly - Nightly releases (unstable); testing - Testing releases (do not use); current - Whatever track this executable was released on."
    )]
    release_track: String,

    #[structopt(
        long,
        default_value = "foundation",
        name = "CHAIN",
        help = "Specify the blockchain type. CHAIN may be either a JSON chain specification file or ethereum, classic, classic-no-phoenix, poacore, xdai, volta, ewc, musicoin, ellaism, mix, callisto, ethercore, mordor, ropsten, kovan, rinkeby, goerli, kotti, poasokol, testnet, evantestcore, evancore or dev."
    )]
    chain: String,

    #[structopt(
        long = "keys-path",
        default_value = "$BASE/keys",
        name = "KEYS_PATH",
        help = "Specify the path for JSON key files to be found"
    )]
    keys_path: String,

    #[structopt(
        name = "NAME",
        long,
        default_value = "",
        help = "Specify your node's name."
    )]
    identity: String,

    #[structopt(
        short = "d",
        long = "base-path",
        name = "BASE_PATH",
        help = "Specify the base data storage path."
    )]
    base_path: Option<String>,

    #[structopt(
        name = "DB_PATH",
        long = "db-path",
        help = "Specify the database directory path"
    )]
    db_path: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct ConvenienceOptions {
    #[structopt(
        long = "unsafe-expose",
        help = "All servers will listen on external interfaces and will be remotely accessible. It's equivalent with setting the following: --[ws,jsonrpc,secretstore,stratum,dapps,secretstore-http]-interface=all --*-hosts=all    This option is UNSAFE and should be used with great care!"
    )]
    unsafe_expose: bool,

    #[structopt(
        short,
        long,
        name = "CONFIG",
        default_value = "$BASE/config.toml",
        help = "Specify a configuration. CONFIG may be either a configuration file or a preset: dev, insecure, dev-insecure, mining, or non-standard-ports."
    )]
    config: String,

    #[structopt(
        long = "ports-shift",
        name = "SHIFT",
        default_value = "0",
        help = "Add SHIFT to all port numbers Parity is listening on. Includes network port and all servers (HTTP JSON-RPC, WebSockets JSON-RPC, SecretStore)."
    )]
    ports_shift: u16,
}

#[derive(StructOpt, Debug)]
pub struct AccountOptions {
    #[structopt(
        long = "fast-unlock",
        help = "Use drastically faster unlocking mode. This setting causes raw secrets to be stored unprotected in memory, so use with care."
    )]
    fast_unlock: bool,

    #[structopt(
        long = "keys-iteration",
        default_value = "10240",
        help = "Specify the number of iterations to use when deriving key from the password (bigger is more secure)",
        name = "NUM"
    )]
    keys_iteration: u32,

    #[structopt(
        long = "accounts-refresh",
        default_value = "5",
        help = "Specify the cache time of accounts read from disk. If you manage thousands of accounts set this to 0 to disable refresh.",
        name = "TIME"
    )]
    accounts_refresh: u64,

    #[structopt(
        long,
        help = "Unlock UNLOCK_ACCOUNTS for the duration of the execution. UNLOCK_ACCOUNTS is a comma-delimited list of addresses.",
        name = "UNLOCK_ACCOUNTS"
    )]
    unlock: Option<String>,

    #[structopt(
        long = "enable-signing-queue",
        name = "BOOLEAN",
        help = "Enables the signing queue for external transaction signing either via CLI or personal_unlockAccount, turned off by default."
    )]
    enable_signing_queue: bool,

    #[structopt(
        long,
        name = "FILE",
        help = "Provide a file containing a password for unlocking an account. Leading and trailing whitespace is trimmed."
    )]
    password: Vec<String>,
}

#[derive(StructOpt, Debug)]
pub struct PrivateTransactions {
    #[structopt(long = "private-tx-enabled", help = "Enable private transactions.")]
    private_enabled: bool,

    #[structopt(
        long = "private-state-offchain",
        help = "Store private state offchain (in the local DB)."
    )]
    private_state_offchain: bool,

    #[structopt(
        long = "private-signer",
        long = "ACCOUNT",
        help = "Specify the account for signing public transaction created upon verified private transaction."
    )]
    private_signer: Option<String>,

    #[structopt(
        long = "private-validators",
        name = "ACCOUNTS",
        help = "Specify the accounts for validating private transactions. ACCOUNTS is a comma-delimited list of addresses."
    )]
    private_validators: Option<String>,

    #[structopt(
        long = "private-account",
        name = "PRIVATE_ACCOUNT",
        help = "Specify the account for signing requests to secret store."
    )]
    private_account: Option<String>,

    #[structopt(
        long = "private-sstore-url",
        name = "URL",
        help = "Specify secret store URL used for encrypting private transactions."
    )]
    private_sstore_url: Option<String>,

    #[structopt(
        long = "private-sstore-threshold",
        name = "THRESHOLD_NUM",
        help = "Specify secret store threshold used for encrypting private transactions."
    )]
    private_sstore_threshold: Option<String>,

    #[structopt(
        long = "private-passwords",
        name = "PASS_FILE",
        help = "Provide a file containing passwords for unlocking accounts (signer, private account, validators)."
    )]
    private_passwords: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct UIOptons {
    #[structopt(
        default_value = "$BASE/signer",
        long = "ui-path",
        help = "Specify directory where Trusted UIs tokens should be stored."
    )]
    ui_path: String,
}

#[derive(StructOpt, Debug)]
pub struct NetworkingOptions {
    #[structopt(
        long = "no-warp",
        help = "Disable syncing from the snapshot over the network."
    )]
    no_warp: bool,

    #[structopt(long = "no-discovery", help = "Disable new peer discovery.")]
    no_discovery: bool,

    #[structopt(long = "reserved-only", help = "Connect only to reserved nodes.")]
    reserved_only: bool,

    #[structopt(
        long = "no-ancient-blocks",
        help = "Disable downloading old blocks after snapshot restoration or warp sync. Not recommended."
    )]
    no_ancient_blocks: bool,

    #[structopt(long = "no-serve-light", help = "Disable serving of light peers.")]
    no_serve_light: bool,

    #[structopt(
        long = "warp-barrier",
        name = "WARP_BARRIER_NUM",
        help = "When warp enabled never attempt regular sync before warping to block WARP_BARRIER_NUM."
    )]
    warp_barrier: Option<u64>,

    #[structopt(
        default_value = "30303",
        long,
        name = "PORT",
        help = "Override the port on which the node should listen."
    )]
    port: u16,

    #[structopt(
        default_value = "all",
        long,
        name = "IP",
        help = "Network interfaces. Valid values are 'all', 'local' or the ip of the interface you want parity to listen to."
    )]
    interface: String,

    #[structopt(
        long = "min-peers",
        name = "MIN_NUM",
        help = "Try to maintain at least MIN_NUM peers."
    )]
    min_peers: Option<u16>,

    #[structopt(
        long = "max-peers",
        name = "MAX_NUM",
        help = "Try to maintain at least MAX_NUM peers."
    )]
    max_peers: Option<u16>,

    #[structopt(
        long = "snapshot-peers",
        default_value = "0",
        name = "SNAPSHOT_NUM",
        help = "Allow additional SNAPSHOT_NUM peers for a snapshot sync."
    )]
    snapshot_peers: u16,

    #[structopt(
        long,
        default_value = "any",
        help = "Specify method to use for determining public address. Must be one of: any, none, upnp, extip:<IP>.",
        name = "METHOD"
    )]
    nat: String,

    #[structopt(
        default_value = "all",
        long = "allow-ips",
        help = "Filter outbound connections. Must be one of: private - connect to private network IP addresses only; public - connect to public network IP addresses only; all - connect to any IP address.",
        name = "FILTER"
    )]
    allow_ips: String,

    #[structopt(
        default_value = "64",
        long = "max-pending-peers",
        help = "Allow up to PENDING_NUM pending connections.",
        name = "PENDING_NUM"
    )]
    max_pending_peers: u16,

    #[structopt(
        long = "network-id",
        help = "Override the network identifier from the chain we are on.",
        name = "INDEX"
    )]
    network_id: Option<u64>,

    #[structopt(
        long,
        name = "BOOTNODES",
        help = "Override the bootnodes from our chain. NODES should be comma-delimited enodes."
    )]
    bootnodes: Option<String>,

    #[structopt(
        long = "node-key",
        name = "NODE_KEY",
        help = "Specify node secret key, either as 64-character hex string or input to SHA3 operation."
    )]
    node_key: Option<String>,

    #[structopt(
        long = "reserved-peers",
        name = "RESERVED_PEERS_FILE",
        help = "Provide a file containing enodes, one per line. These nodes will always have a reserved slot on top of the normal maximum peers."
    )]
    reserved_peers: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct IPCOptions {
    #[structopt(
        help = "Provide a file containing enodes, one per line. These nodes will always have a reserved slot on top of the normal maximum peers.",
        long = "no-ipc"
    )]
    no_ipc: bool,

    #[structopt(
        help="Provide a file containing enodes, one per line. These nodes will always have a reserved slot on top of the normal maximum peers.",
        long="ipc-path",
        name="IPC_PATH",
        default_value=IPCOptions::ipc_path_default()
    )]
    ipc_path: String, // TODO: Check if the default value actually works like this

    #[structopt(
        long = "ipc-chmod",
        default_value = "660",
        name = "IPC_CHMOD_NUM",
        help = "Specify octal value for ipc socket permissions (unix/bsd only)"
    )]
    ipc_chmod: String,

    #[structopt(
        default_value = "web3,eth,pubsub,net,parity,parity_pubsub,parity_accounts,private,traces,rpc,parity_transactions_pool",
        long = "ipc-apis",
        name = "IPC_APIS",
        help = "Specify custom API set available via JSON-RPC over IPC using a comma-delimited list of API names. Possible names are: all, safe, web3, net, eth, pubsub, personal, signer, parity, parity_pubsub, parity_accounts, parity_set, traces, rpc, secretstore. You can also disable a specific API by putting '-' in the front, example: all,-personal. 'safe' enables the following APIs: web3, net, eth, pubsub, parity, parity_pubsub, traces, rpc"
    )]
    ipc_apis: String,
}

impl<'a> IPCOptions {
    fn ipc_path_default() -> &'a str {
        if cfg!(windows) {
            r"\\.\pipe\jsonrpc.ipc"
        } else {
            "$BASE/jsonrpc.ipc"
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct HTTP_JSON_RPC_Options {
    #[structopt(
        long = "json-rpc-allow-missing-blocks",
        help = "RPC calls will return 'null' instead of an error if ancient block sync is still in progress and the block information requested could not be found"
    )]
    jsonrpc_allow_missing_blocks: bool,

    #[structopt(long = "no-jsonrpc", help = "Disable the HTTP JSON-RPC API server.")]
    no_jsonrpc: bool,

    #[structopt(
        long = "jsonrpc-experimental",
        help = "Enable experimental RPCs. Enable to have access to methods from unfinalised EIPs in all namespaces"
    )]
    jsonrpc_experimental: bool,

    #[structopt(
        long = "jsonrpc-port",
        default_value = "8545",
        name = "JSONRPC_PORT",
        help = "Specify the port portion of the HTTP JSON-RPC API server."
    )]
    jsonrpc_port: u16,

    #[structopt(
        default_value = "local",
        long = "jsonrpc-interface",
        name = "JSONRPC_IP",
        help = "Specify the hostname portion of the HTTP JSON-RPC API server, JSONRPC_IP should be an interface's IP address, or all (all interfaces) or local."
    )]
    jsonrpc_interface: String,

    #[structopt(
        long = "jsonrpc-apis",
        name = "JSONRPC_APIS",
        default_value = "web3,eth,pubsub,net,parity,private,parity_pubsub,traces,rpc,parity_transactions_pool",
        help = "Specify the APIs available through the HTTP JSON-RPC interface using a comma-delimited list of API names. Possible names are: all, safe, debug, web3, net, eth, pubsub, personal, signer, parity, parity_pubsub, parity_accounts, parity_set, traces, rpc, secretstore. You can also disable a specific API by putting '-' in the front, example: all,-personal. 'safe' enables the following APIs: web3, net, eth, pubsub, parity, parity_pubsub, traces, rpc"
    )]
    jsonrpc_apis: String,

    #[structopt(
        long = "jsonrpc-hosts",
        default_value = "none",
        name = "JSONRPC_HOSTS",
        help = "List of allowed Host header values. This option will validate the Host header sent by the browser, it is additional security against some attack vectors. Special options: \"all\", \"none\",."
    )]
    jsonrpc_hosts: String,

    #[structopt(
        long = "jsonrpc-threads",
        name = "JSONRPC_THREADS_NUM",
        help = "DEPRECATED, DOES NOTHING"
    )]
    jsonrpc_threads: Option<usize>,

    #[structopt(
        long = "jsonrpc-server-threads",
        name = "JSONRPC_SERVER_THREADS",
        default_value = "4",
        help = "Enables multiple threads handling incoming connections for HTTP JSON-RPC server."
    )]
    jsonrpc_server_threads: usize, // TODO: See if this breaks anything - this was originally an Option<usize>, but default_values is not compatible with that

    #[structopt(
        name = "JSONRPC_CORS_URL",
        long = "jsonrpc-cors",
        default_value = "none",
        help = "Specify CORS header for HTTP JSON-RPC API responses. Special options: \"all\", \"none\"."
    )]
    jsonrpc_cors: String,

    #[structopt(
        long = "jsonrpc-max-payload",
        name = "JSONRPC_MAX_MB",
        help = "Specify maximum size for HTTP JSON-RPC requests in megabytes."
    )]
    jsonrpc_max_payload: Option<usize>,

    #[structopt(
        default_value = "60",
        name = "POLL_LIFETIME_SECS",
        long = "poll-lifetime",
        help = "Set the RPC filter lifetime to S seconds. The filter has to be polled at least every S seconds , otherwise it is removed."
    )]
    poll_lifetime: u32,
}

#[derive(StructOpt, Debug)]
pub struct WebsocketsOptions {
    #[structopt(help = "Disable the WebSockets JSON-RPC server.", long = "no-ws")]
    no_ws: bool,

    #[structopt(
        default_value = "8546",
        long = "ws-port",
        name = "WS_PORT",
        help = "Specify the port portion of the WebSockets JSON-RPC server."
    )]
    ws_port: u16,

    #[structopt(
        default_value = "local",
        long = "ws-interface",
        name = "WS_INTERFACE_IP",
        help = "Specify the hostname portion of the WebSockets JSON-RPC server, IP should be an interface's IP address, or all (all interfaces) or local."
    )]
    ws_interface: String,

    #[structopt(
        default_value = "web3,eth,pubsub,net,parity,parity_pubsub,private,traces,rpc,parity_transactions_pool",
        help = "Specify the JSON-RPC APIs available through the WebSockets interface using a comma-delimited list of API names. Possible names are: all, safe, web3, net, eth, pubsub, personal, signer, parity, parity_pubsub, parity_accounts, parity_set, traces, rpc, secretstore. You can also disable a specific API by putting '-' in the front, example: all,-personal. 'safe' enables the following APIs: web3, net, eth, pubsub, parity, parity_pubsub, traces, rpc",
        long = "ws-apis",
        name = "WS_APIS"
    )]
    ws_apis: String,

    #[structopt(
        default_value = "parity://*,chrome-extension://*,moz-extension://*",
        help = "Specify Origin header values allowed to connect. Special options: \"all\", \"none\".",
        long = "ws-origins",
        name = "WS_ORIGINS_URL"
    )]
    ws_origins: String,

    #[structopt(
        default_value = "none",
        help = "List of allowed Host header values. This option will validate the Host header sent by the browser, it is additional security against some attack vectors. Special options: \"all\", \"none\".",
        long = "ws-hosts",
        name = "WS_HOSTS"
    )]
    ws_hosts: String,

    #[structopt(
        default_value = "100",
        help = "Maximum number of allowed concurrent WebSockets JSON-RPC connections.",
        long = "ws=-connections",
        name = "WS_MAX_CONN"
    )]
    ws_max_connections: usize,
}

#[derive(StructOpt, Debug)]
pub struct LightClientOptions {
    #[structopt(
        long = "on-demand-time-window",
        help = "Specify the maximum time to wait for a successful response",
        name = "RESPONSE_SECS"
    )]
    on_demand_response_time_window: Option<u64>,

    #[structopt(
        long = "on-demand-start-backoff",
        name = "BACKOFF_START_SECS",
        help = "Specify light client initial backoff time for a request"
    )]
    on_demand_request_backoff_start: Option<u64>,

    #[structopt(
        long = "on-demand-end-backoff",
        name = "BACKOFF_END_SECS",
        help = "Specify light client maximam backoff time for a request"
    )]
    on_demand_request_backoff_max: Option<u64>,

    #[structopt(
        long = "on-demand-max-backoff-rounds",
        name = "BACKOFF_MAX_ROUNDS_TIMES",
        help = "Specify light client maximam number of backoff iterations for a request"
    )]
    on_demand_request_backoff_rounds_max: Option<u64>,

    #[structopt(
        long = "on-demand-consecutive-failures",
        name = "MAX_CONSECUTIVE_FAILURE_TIMES",
        help = "Specify light client the number of failures for a request until it gets exponentially backed off"
    )]
    on_demand_request_consecutive_failures: Option<u64>,
}

#[derive(StructOpt, Debug)]
pub struct SecretStoreOptions {
    #[structopt(help = "Disable Secret Store functionality.", long = "no-secretstore")]
    no_secretstore: bool,

    #[structopt(help = "Disable Secret Store HTTP API.", long = "no-secretstore-http")]
    no_secretstore_http: bool,

    #[structopt(
        help = "Do not run servers set change session automatically when servers set changes. This option has no effect when servers set is read from configuration file.",
        long = "no-secretstore-auto-migrate"
    )]
    no_secretstore_auto_migrate: bool,

    #[structopt(
        default_value = "none",
        name = "HTTP_CORS_URLS",
        help = "Specify CORS header for Secret Store HTTP API responses. Special options: \"all\", \"none\".",
        long = "no-secretstore-http-cors"
    )]
    no_secretstore_http_cors: String,

    #[structopt(
        default_value = "registry",
        help = "Secret Store permissioning contract address source: none, registry (contract address is read from 'secretstore_acl_checker' entry in registry) or address.",
        long = "no-secretstore-act-contract"
    )]
    no_secretstore_act_contract: String, // TODO: Check if this still actually works, because in the original implementation, this was an Option<String>

    #[structopt(
        long = "secrestore-contract",
        name = "SECRETSTORE_SOURCE",
        help = "Secret Store Service contract address source: none, registry (contract address is read from 'secretstore_service' entry in registry) or address."
    )]
    secretstore_contract: Option<String>,

    #[structopt(
        long = "secretstore-srv-gen-contract",
        name = "GEN_SOURCE",
        help = "Secret Store Service server key generation contract address source: none, registry (contract address is read from 'secretstore_service_srv_gen' entry in registry) or address."
    )]
    secretstore_srv_gen_contract: Option<String>,

    #[structopt(
        help = "Secret Store Service server key retrieval contract address source: none, registry (contract address is read from 'secretstore_service_srv_retr' entry in registry) or address.",
        name = "RETR_SOURCE",
        long = "secretstore-srv-retr-contract"
    )]
    secretstore_srv_retr_contract: Option<String>,

    #[structopt(
        help = "Secret Store Service document key store contract address source: none, registry (contract address is read from 'secretstore_service_doc_store' entry in registry) or address.",
        name = "DOC_STORE_SOURCE",
        long = "secretstore-doc-store-contract"
    )]
    secretstore_doc_store_contract: Option<String>,

    #[structopt(
        help = "Secret Store Service document key shadow retrieval contract address source: none, registry (contract address is read from 'secretstore_service_doc_sretr' entry in registry) or address.",
        name = "DOC_SRETR_SOURCE",
        long = "secretstore-doc-sretr-contract"
    )]
    secretstore_doc_sretr_contract: Option<String>,

    #[structopt(
        help = "Comma-separated list of other secret store cluster nodes in form NODE_PUBLIC_KEY_IN_HEX@NODE_IP_ADDR:NODE_PORT.",
        name = "SECRETSTORE_NODES",
        long = "secretstore-nodes",
        default_value = ""
    )]
    secretstore_nodes: String,

    #[structopt(
        name = "SET_CONTRACT_SOURCE",
        long = "secretstore-server-set-contract",
        default_value = "registry",
        help = "Secret Store server set contract address source: none, registry (contract address is read from 'secretstore_server_set' entry in registry) or address."
    )]
    secretstore_server_set_contract: String,

    #[structopt(
        help = "Specify the hostname portion for listening to Secret Store Key Server internal requests, IP should be an interface's IP address, or local.",
        name = "SECRETSTORE_IP",
        long = "secretstore-interface-ip",
        default_value = "local"
    )]
    secretstore_interface: String,

    #[structopt(
        default_value = "8083",
        long = "secretstore-port",
        name = "SECRETSTORE_PORT",
        help = "Specify the port portion for listening to Secret Store Key Server internal requests."
    )]
    secretstore_port: u16,

    #[structopt(
        long = "secretstore-http-interface",
        default_value = "local",
        help = "Specify the hostname portion for listening to Secret Store Key Server HTTP requests, IP should be an interface's IP address, or local.",
        name = "SECRETSTORE_HTTP_INTERFACE"
    )]
    secretstore_http_interface: String,

    #[structopt(
        default_value = "8082",
        long = "secretstore-http-port",
        name = "SECRETSTORE_HTTP_PORT",
        help = "Specify the port portion for listening to Secret Store Key Server HTTP requests."
    )]
    secretstore_http_port: u16,

    #[structopt(
        default_value = "$BASE/secretstore",
        name = "SECRETSTORE_PATH",
        long = "secretstore-path",
        help = "Specify directory where Secret Store should save its data."
    )]
    secretstore_path: String,

    #[structopt(
        long = "secretstore-secret",
        name = "SECRETSTORE_SECRET",
        help = "Hex-encoded secret key of this node."
    )]
    secretstore_secret: Option<String>,

    #[structopt(
        long = "secretstore-admin-public",
        name = "SECRETSTORE_ADMIN_PUBLIC",
        help = "Hex-encoded public key of secret store administrator."
    )]
    secretstore_admin_public: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct SealingMiningOptions {
    #[structopt(
        help = "Force the node to author new blocks as if it were always sealing/mining.",
        long = "force-sealing"
    )]
    force_sealing: bool,

    #[structopt(
        help = "Force the node to author new blocks when a new uncle block is imported.",
        long = "reseal-on-uncle"
    )]
    reseal_on_uncle: bool,

    #[structopt(
        help = "Move solved blocks from the work package queue instead of cloning them. This gives a slightly faster import speed, but means that extra solutions submitted for the same work package will go unused.",
        long = "remove-solved"
    )]
    remove_solved: bool,

    #[structopt(
        help = "Local transactions sent through JSON-RPC (HTTP, WebSockets, etc) will be treated as 'external' if the sending account is unknown.",
        long = "tx-queue-no-unfamiliar-locals"
    )]
    tx_queue_no_unfamiliar_locals: bool,

    #[structopt(
        help = "Always refuse service transactions.",
        long = "refuse-service-transactions"
    )]
    refuse_service_transactions: bool,

    #[structopt(
        help = "Pending block will be created with maximal possible gas limit and will execute all transactions in the queue. Note that such block is invalid and should never be attempted to be mined.",
        long = "infinite-pending-block"
    )]
    infinite_pending_block: bool,

    #[structopt(
        help = "Don't save pending local transactions to disk to be restored whenever the node restarts.",
        long = "no-persistent-txqueue"
    )]
    no_persistent_txqueue: bool,

    // For backward compatibility; Stratum should be enabled if the config file
    // contains a `[stratum]` section and it is not explicitly disabled (disable = true)
    #[structopt(help = "Run Stratum server for miner push notification.", long)]
    stratum: bool,

    #[structopt(
        long = "reseal-on-txs",
        default_value = "own",
        name = "RESEAL_TXS_SET",
        help = "Specify which transactions should force the node to reseal a block. SET is one of: none - never reseal on new transactions; own - reseal only on a new local transaction; ext - reseal only on a new external transaction; all - reseal on all new transactions."
    )]
    reseal_on_txs: String,

    #[structopt(
        help = "Specify the minimum time between reseals from incoming transactions. MS is time measured in milliseconds.",
        long = "reseal-min-period",
        name = "RESEAL_MIN_MS",
        default_value = "2000"
    )]
    reseal_min_period: u64,

    #[structopt(
        long = "reseal-max-period",
        name = "RESEAL_MAX_MS",
        default_value = "120000",
        help = "Specify the maximum time between reseals from incoming transactions. MS is time measured in milliseconds."
    )]
    reseal_max_period: u64,

    #[structopt(
        name = "WORK_QUEUE_SIZE_ITEMS",
        long = "work-queue-size",
        default_value = "20",
        help = "Specify the number of historical work packages which are kept cached lest a solution is found for them later. High values take more memory but result in fewer unusable solutions."
    )]
    work_queue_size: usize,

    #[structopt(
        long = "relay-set",
        default_value = "cheap",
        name = "RELAY_SET",
        help = "Set of transactions to relay. SET may be: cheap - Relay any transaction in the queue (this may include invalid transactions); strict - Relay only executed transactions (this guarantees we don't relay invalid transactions, but means we relay nothing if not mining); lenient - Same as strict when mining, and cheap when not."
    )]
    relay_set: String,

    #[structopt(
        long = "usd-per-tx",
        default_value = "0.0001",
        name = "USD_PER_TX",
        help = "Amount of USD to be paid for a basic transaction. The minimum gas price is set accordingly."
    )]
    usd_per_tx: String,

    #[structopt(
        help = "USD value of a single ETH. SOURCE may be either an amount in USD, a web service or 'auto' to use each web service in turn and fallback on the last known good value.",
        name = "USD_PER_ETH_SOURCE",
        default_value = "auto",
        long = "usd-per-eth"
    )]
    usd_per_eth: String,

    #[structopt(
        long = "price-update-period",
        default_value = "hourly",
        name = "PRICE_UPDATE_T",
        help = "PRICE_UPDATE_T will be allowed to pass between each gas price update. PRICE_UPDATE_T may be daily, hourly, a number of seconds, or a time string of the form \"2 days\", \"30 minutes\" etc.."
    )]
    price_update_period: String,

    #[structopt(
        help = "Amount of gas per block to target when sealing a new block.",
        long = "gas-floor-target",
        default_value = "8000000",
        name = "GAS_FLOOR"
    )]
    gas_floor_target: String,

    #[structopt(
        help = "A cap on how large we will raise the gas limit per block due to transaction volume.",
        long = "gas-cap",
        default_value = "10000000",
        name = "GAS_CAP"
    )]
    gas_cap: String,

    #[structopt(
        help = "Maximum amount of memory that can be used by the transaction queue. Setting this parameter to 0 disables limiting.",
        long = "tx-queue-mem-limit",
        default_value = "4",
        name = "TX_QUEUE_LIMIT_MB"
    )]
    tx_queue_mem_limit: u32,

    #[structopt(
        help = "Maximum amount of transactions in the queue (waiting to be included in next block).",
        long = "tx-queue-size",
        default_value = "8192",
        name = "TX_QUEUE_SIZE_LIMIT"
    )]
    tx_queue_size: usize,

    #[structopt(
        help = "Maximum number of transactions per sender in the queue. By default it's 1% of the entire queue, but not less than 16.",
        long = "tx-queue-per-sender",
        name = "TX_QUEUE_PER_SENDER_LIMIT"
    )]
    tx_queue_per_sender: Option<usize>,

    #[structopt(
        help = "Specify local accounts for which transactions are prioritized in the queue. ACCOUNTS is a comma-delimited list of addresses.",
        long = "tx-queue-locals",
        name = "TX_QUEUE_LOCAL_ACCOUNTS"
    )]
    tx_queue_locals: Option<String>,

    #[structopt(
        help = "Prioritization strategy used to order transactions in the queue. S may be: gas_price - Prioritize txs with high gas price",
        long = "tx-queue-strategy",
        default_value = "gas_price",
        name = "TX_QUEUE_S"
    )]
    tx_queue_strategy: String,

    #[structopt(
        help = "Interface address for Stratum server.",
        long = "stratum-interface",
        default_value = "local",
        name = "STRATUM_IP"
    )]
    stratum_interface: String,

    #[structopt(
        help = "Port for Stratum server to listen on.",
        long = "stratum-port",
        default_value = "8008",
        name = "STRATUM_PORT"
    )]
    stratum_port: u16,

    #[structopt(
        help = "Minimum amount of Wei per GAS to be paid for a transaction to be accepted for mining. Overrides --usd-per-tx.",
        long = "min-gas-price",
        name = "MIN_GAS_PRICE_STRING"
    )]
    min_gas_price: Option<u64>,

    #[structopt(
        help = "Set PCT percentile gas price value from last 100 blocks as default gas price when sending transactions.",
        long = "gas-price-percentile",
        default_value = "50",
        name = "PCT"
    )]
    gas_price_percentile: usize,

    #[structopt(
        help = "Specify the block author (aka \"coinbase\") address for sending block rewards from sealed blocks. NOTE: MINING WILL NOT WORK WITHOUT THIS OPTION.",
        long,
        name = "ADDRESS"
    )]
    author: Option<String>, // Sealing / Mining Option

    #[structopt(
        help = "Specify the address which should be used to sign consensus messages and issue blocks. Relevant only to non-PoW chains.",
        long = "engine-signer",
        name = "ENGINE_SIGNER_ADDRESS"
    )]
    engine_signer: Option<String>,

    #[structopt(
        help = "Apply a limit of GAS as the maximum amount of gas a single transaction may have for it to be mined.",
        long = "tx-gas-limit",
        name = "TX_GAS_LIMIT"
    )]
    tx_gas_limit: Option<String>,

    #[structopt(
        help = "Maximal time for processing single transaction. If enabled senders of transactions offending the limit will get other transactions penalized.",
        long = "tx-time-limit",
        name = "TX_TIME_LIMIT_MS"
    )]
    tx_time_limit: Option<u64>,

    #[structopt(
        long = "extra-data",
        help = "Specify a custom extra-data for authored blocks, no more than 32 characters.",
        name = "EXTRA_DATA_STRING"
    )]
    extra_data: Option<String>,

    #[structopt(
        name = "NOTIFY_WORK_URLS",
        help = "URLs to which work package notifications are pushed. URLS should be a comma-delimited list of HTTP URLs.",
        long = "notify-work"
    )]
    notify_work: Option<String>,

    #[structopt(
        name = "STARTUM_SECRET_STRING",
        long = "stratum-secret",
        help = "Secret for authorizing Stratum server for peers."
    )]
    stratum_secret: Option<String>,

    #[structopt(
        default_value = "12",
        long = "max-round-blocks-to-import",
        name = "MAX_ROUND_BLOCKS_S",
        help = "Maximal number of blocks to import for each import round."
    )]
    max_round_blocks_to_import: usize,
}

#[derive(StructOpt, Debug)]
pub struct InternalOptions {
    #[structopt(
        long = "can-restart",
        help = "Executable will auto-restart if exiting with 69"
    )]
    can_restart: bool,
}

#[derive(StructOpt, Debug)]
pub struct MiscellaneousOptions {
    #[structopt(help = "Don't use terminal color codes in output.", long = "no-color")]
    no_color: bool,

    // version flag is automatically provided by structopt
    #[structopt(long = "no-config", help = "Don't load a configuration file.")]
    no_config: bool,

    #[structopt(
        short = "l",
        long,
        name = "LOGGING",
        help = "Specify the general logging level (error, warn, info, debug or trace). It can also be set for a specific module, example: '-l sync=debug,rpc=trace'"
    )]
    logging: Option<String>,

    #[structopt(
        long = "log-file",
        name = "LOG_FILENAME",
        help = "Specify a filename into which logging should be appended"
    )]
    log_file: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct FootPrintOptions {
    #[structopt(
        help = "Automatically scale amount of verifier threads based on workload. Not guaranteed to be faster.",
        long = "scale-verifiers"
    )]
    scale_verifiers: bool,

    #[structopt(
        help = "Indicates if full transaction tracing should be enabled. Works only if client had been fully synced with tracing enabled. BOOL may be one of auto, on, off. auto uses last used value of this option (off if it does not exist).",
        long,
        name = "TRACING_BOOL",
        default_value = "auto"
    )]
    tracing: String,

    #[structopt(
        long,
        name = "PRUNING_METHOD",
        default_value = "auto",
        help = "Configure pruning of the state/storage trie. PRUNING_METHOD may be one of auto, archive, fast: archive - keep all state trie data. No pruning. fast - maintain journal overlay. Fast but 50MB used. auto - use the method most recently synced or default to fast if none synced."
    )]
    pruning: String,

    #[structopt(
        long = "pruning-history",
        default_value = "128",
        help = "Set a minimum number of recent states to keep in memory when pruning is active.",
        name = "PRUNING_HISTORY_NUM"
    )]
    pruning_history: u64,

    #[structopt(
        long = "pruning-memory",
        name = "PRUNING_MEMORY_MB",
        help = "The ideal amount of memory in megabytes to use to store recent states. As many states as possible will be kept within this limit, and at least --pruning-history states will always be kept.",
        default_value = "64"
    )]
    pruning_memory: usize,

    #[structopt(
        help = "Override database cache size.",
        long = "cache-size-db",
        default_value = "128",
        name = "CACHE_SIZE_DB_MB"
    )]
    cache_size_db: u32,

    #[structopt(
        help = "Specify the preferred size of the blockchain cache in megabytes.",
        long = "cache-size-blocks",
        name = "CACHE_SIZE_BLOCKS_MB",
        default_value = "8"
    )]
    cache_size_blocks: u32,

    #[structopt(
        help = "Specify the maximum size of memory to use for block queue.",
        long = "cache-size-queue",
        name = "CACHE_SIZE_QUEUE_MB",
        default_value = "40"
    )]
    cache_size_queue: u32,

    #[structopt(
        help = "Specify the maximum size of memory to use for the state cache.",
        long = "cache-size-state",
        name = "CACHE_SIZE_STATE",
        default_value = "25"
    )]
    cache_size_state: u32,

    #[structopt(
        help = "Database compaction type. TYPE may be one of: ssd - suitable for SSDs and fast HDDs; hdd - suitable for slow HDDs; auto - determine automatically.",
        default_value = "auto",
        long = "db-compaction",
        name = "DB_COMPACTION_TYPE"
    )]
    db_compaction: String,

    #[structopt(
        help = "Build appropriate information to allow enumeration of all accounts and storage keys. Doubles the size of the state database. BOOL may be one of on, off or auto.",
        default_value = "auto",
        long = "fat-db",
        name = "FAT_DB_BOOL"
    )]
    fat_db: String,

    #[structopt(
        help = "Set total amount of discretionary memory to use for the entire system, overrides other cache and queue options.",
        long = "cache-size",
        name = "CACHE_SIZE_MB"
    )]
    cache_size: Option<u32>,

    #[structopt(
        help = "Amount of verifier threads to use or to begin with, if verifier auto-scaling is enabled.",
        name = "NUM_VERIFIERS_INT",
        long = "num-verifiers"
    )]
    num_verifiers: Option<usize>,
}

#[derive(StructOpt, Debug)]
pub struct ImportExportOptions {
    #[structopt(long = "no-seal-check", help = "Skip block seal check.")]
    no_seal_check: bool,
}

#[derive(StructOpt, Debug)]
pub struct SnapshotOptions {
    #[structopt(
        help = "Disable automated snapshots which usually occur once every 5000 blocks.",
        long = "no-periodic-snapshots"
    )]
    no_periodic_snapshot: bool,

    #[structopt(
        help = "Enables multiple threads for snapshots creation.",
        long = "snapshot-threads",
        name = "SNAPSHOT_THREADS_NUM"
    )]
    snapshot_threads: bool,
}

#[derive(StructOpt, Debug)]
pub struct LegacyOptions {
    // TODO: These options were hidden from config, so should we not include them?
    #[structopt(
        long,
        help = "Run in Geth-compatibility mode. Sets the IPC path to be the same as Geth's. Overrides the --ipc-path and --ipcpath options. Alters RPCs to reflect Geth bugs. Includes the personal_ RPC by default."
    )]
    geth: bool,

    #[structopt(
        help = "Attempt to import keys from Geth client.",
        long = "import-geth-keys"
    )]
    import_geth_keys: bool,
    // Removed legacy flags
    // TODO: Figure out if it's okay to remove these flags
}
