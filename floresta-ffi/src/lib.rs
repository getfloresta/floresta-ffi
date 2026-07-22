uniffi::setup_scaffolding!("floresta");

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use bitcoin::hashes::Hash;

/// The Bitcoin network to run on.
#[derive(Debug, Clone, uniffi::Enum)]
pub enum Network {
    /// Bitcoin mainnet.
    Bitcoin,
    /// Bitcoin signet.
    Signet,
    /// Bitcoin testnet.
    Testnet,
    /// Bitcoin regtest.
    Regtest,
    /// Bitcoin testnet4.
    Testnet4,
}

impl From<Network> for bitcoin::Network {
    fn from(network: Network) -> bitcoin::Network {
        match network {
            Network::Bitcoin => bitcoin::Network::Bitcoin,
            Network::Signet => bitcoin::Network::Signet,
            Network::Testnet => bitcoin::Network::Testnet,
            Network::Regtest => bitcoin::Network::Regtest,
            Network::Testnet4 => bitcoin::Network::Testnet4,
        }
    }
}

#[derive(Debug, Clone, uniffi::Enum)]
/// Configures the assume-valid behavior for script validation.
pub enum AssumeValidArg {
    /// Validate all scripts from genesis.
    Disabled,

    /// Use Floresta's hard-coded block hash.
    Hardcoded,

    /// Use a user-provided block hash (64-character hex string).
    UserInput { block_hash: String },
}

#[derive(Debug, Clone, uniffi::Record)]
/// A pre-computed Utreexo accumulator state.
pub struct AssumeUtreexoValue {
    /// The block hash at which this accumulator state is valid.
    pub block_hash: String,

    /// The block height at which this accumulator state is valid.
    pub height: u32,

    /// The Utreexo accumulator roots at this block, as hex strings.
    pub roots: Vec<String>,

    /// The number of leaves in the Utreexo accumulator at this block.
    pub leaves: u64,
}

#[derive(Debug, uniffi::Error)]
/// Error returned by the Floresta FFI layer.
pub enum FlorestaFfiError {
    /// The daemon failed to start, with an error message.
    StartError { details: String },
}

impl std::fmt::Display for FlorestaFfiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartError { details } => write!(f, "{details}"),
        }
    }
}

impl std::error::Error for FlorestaFfiError {}

/// A Floresta Bitcoin node instance.
///
/// Wraps the Floresta daemon and a Tokio runtime. Create with [`Florestad::new`]
/// for defaults or [`Florestad::from_config`] for custom settings. Call
/// [`Florestad::start`] to begin syncing and [`Florestad::stop`] before exit.
#[derive(uniffi::Object)]
pub struct Florestad {
    rt: tokio::runtime::Runtime,
    florestad: floresta_node::Florestad,
}

#[uniffi::export]
impl Florestad {
    /// Create a new Floresta node with default configuration.
    ///
    /// Uses Bitcoin mainnet and the `$HOME/.floresta` data directory.
    /// Falls back to the system temp directory if `$HOME` is not set.
    #[uniffi::constructor]
    pub fn new() -> Arc<Florestad> {
        let _rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(4)
            .thread_name("florestad")
            .build()
            .expect("failed to create tokio runtime");

        let datadir = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir())
            .join(".floresta");
        let config = floresta_node::Config::new(bitcoin::Network::Bitcoin, datadir);
        let florestad = floresta_node::Florestad::from_config(config);
        Arc::new(Self { rt: _rt, florestad })
    }

    /// Create a new Floresta node with the given configuration.
    #[uniffi::constructor]
    pub fn from_config(config: Config) -> Arc<Florestad> {
        let _rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(4)
            .thread_name("florestad")
            .build()
            .expect("failed to create tokio runtime");

        let florestad = floresta_node::Florestad::from_config(config.into());
        Arc::new(Self { rt: _rt, florestad })
    }

    /// Start the node.
    ///
    /// Begins syncing the blockchain, serving the Electrum and JSON-RPC
    /// interfaces, and watching configured wallets. Returns an error if
    /// the data directory is not writable or initialization fails.
    pub fn start(&self) -> Result<(), FlorestaFfiError> {
        self.rt.block_on(async {
            self.florestad
                .start()
                .await
                .map_err(|e| FlorestaFfiError::StartError {
                    details: e.to_string(),
                })
        })
    }

    /// Gracefully stop the node.
    ///
    /// Waits for all pending operations to finish and flushes data to disk.
    /// Always call this before exiting to avoid data corruption.
    pub fn stop(&self) {
        self.rt.block_on(async {
            self.florestad.stop().await;
        });
    }
}

/// Configuration for the Floresta daemon.
#[derive(Clone, uniffi::Record)]
pub struct Config {
    /// Path to the data directory. Must be readable and writable.
    pub datadir: String,

    /// The Bitcoin network to run on.
    pub network: Network,

    /// Disable DNS seed nodes for peer discovery.
    #[uniffi(default = false)]
    pub disable_dns_seeds: bool,

    /// Which blocks are assumed to have valid scripts.
    pub assume_valid: AssumeValidArg,

    /// SLIP-132-encoded extended public keys to watch.
    #[uniffi(default = None)]
    pub wallet_xpub: Option<Vec<String>>,

    /// Output descriptors to watch.
    #[uniffi(default = None)]
    pub wallet_descriptor: Option<Vec<String>>,

    /// Path to a TOML configuration file.
    #[uniffi(default = None)]
    pub config_file: Option<String>,

    /// SOCKS5 proxy for outgoing connections.
    #[uniffi(default = None)]
    pub proxy: Option<String>,

    /// Whether to build compact block filters.
    #[uniffi(default = false)]
    pub cfilters: bool,

    /// Block height to start downloading compact filters from.
    #[uniffi(default = None)]
    pub filters_start_height: Option<i32>,

    /// ZMQ server address (requires zmq-server feature).
    #[uniffi(default = None)]
    pub zmq_address: Option<String>,

    /// Nodes to connect to exclusively.
    #[uniffi(default = [])]
    pub connect: Vec<String>,

    /// JSON-RPC server address (requires json-rpc feature).
    #[uniffi(default = None)]
    pub json_rpc_address: Option<String>,

    /// Whether to write logs to stdout.
    #[uniffi(default = false)]
    pub log_to_stdout: bool,

    /// Whether to write logs to a file.
    #[uniffi(default = false)]
    pub log_to_file: bool,

    /// Enable assume-utreexo mode.
    #[uniffi(default = false)]
    pub assume_utreexo: bool,

    /// Enable debug logging.
    #[uniffi(default = false)]
    pub debug: bool,

    /// User agent string advertised to peers.
    #[uniffi(default = "")]
    pub user_agent: String,

    /// Custom Utreexo accumulator state for assume-utreexo.
    #[uniffi(default = None)]
    pub assumeutreexo_value: Option<AssumeUtreexoValue>,

    /// Electrum server address.
    #[uniffi(default = None)]
    pub electrum_address: Option<String>,

    /// Whether to enable the Electrum TLS server.
    #[uniffi(default = false)]
    pub enable_electrum_tls: bool,

    /// Electrum TLS server address.
    #[uniffi(default = None)]
    pub electrum_address_tls: Option<String>,

    /// Path to the TLS private key file.
    #[uniffi(default = None)]
    pub tls_key_path: Option<String>,

    /// Path to the TLS certificate file.
    #[uniffi(default = None)]
    pub tls_cert_path: Option<String>,

    /// Whether to generate a self-signed TLS certificate.
    #[uniffi(default = false)]
    pub generate_cert: bool,

    /// Whether to allow v1 transport fallback.
    #[uniffi(default = false)]
    pub allow_v1_fallback: bool,

    /// Whether to backfill skipped blocks.
    #[uniffi(default = false)]
    pub backfill: bool,
}

impl From<Config> for floresta_node::Config {
    fn from(config: Config) -> floresta_node::Config {
        let mut cfg =
            floresta_node::Config::new(config.network.into(), PathBuf::from(&config.datadir));

        cfg.disable_dns_seeds = config.disable_dns_seeds;
        cfg.wallet_xpub = config.wallet_xpub;
        cfg.wallet_descriptor = config.wallet_descriptor;
        cfg.config_file = config.config_file.map(PathBuf::from);
        cfg.proxy = config.proxy;
        cfg.cfilters = config.cfilters;
        cfg.filters_start_height = config.filters_start_height;
        cfg.connect = config.connect;
        cfg.json_rpc_address = config.json_rpc_address;

        #[cfg(feature = "zmq-server")]
        {
            cfg.zmq_address = config.zmq_address;
        }

        cfg.log_to_stdout = config.log_to_stdout;
        cfg.log_to_file = config.log_to_file;
        cfg.assume_utreexo = config.assume_utreexo;
        cfg.debug = config.debug;
        cfg.user_agent = config.user_agent;
        cfg.electrum_address = config.electrum_address;
        cfg.enable_electrum_tls = config.enable_electrum_tls;
        cfg.electrum_address_tls = config.electrum_address_tls;
        cfg.tls_key_path = config.tls_key_path.map(PathBuf::from);
        cfg.tls_cert_path = config.tls_cert_path.map(PathBuf::from);
        cfg.generate_cert = config.generate_cert;
        cfg.allow_v1_fallback = config.allow_v1_fallback;
        cfg.backfill = config.backfill;

        cfg.assume_valid = match config.assume_valid {
            AssumeValidArg::Disabled => floresta_node::AssumeValidArg::Disabled,
            AssumeValidArg::Hardcoded => floresta_node::AssumeValidArg::Hardcoded,
            AssumeValidArg::UserInput { block_hash } => {
                let hash = bitcoin::BlockHash::from_str(&block_hash)
                    .unwrap_or_else(|_| bitcoin::BlockHash::all_zeros());
                floresta_node::AssumeValidArg::UserInput(hash)
            }
        };

        cfg.assumeutreexo_value = config.assumeutreexo_value.and_then(|v| {
            let hash = bitcoin::BlockHash::from_str(&v.block_hash)
                .ok()
                .unwrap_or_else(bitcoin::BlockHash::all_zeros);
            let roots: Vec<rustreexo::node_hash::BitcoinNodeHash> = v
                .roots
                .iter()
                .filter_map(|r| rustreexo::node_hash::BitcoinNodeHash::from_str(r).ok())
                .collect();
            if roots.len() != v.roots.len() {
                return None;
            }
            Some(floresta_node::AssumeUtreexoValue {
                block_hash: hash,
                height: v.height,
                roots,
                leaves: v.leaves,
            })
        });

        cfg
    }
}
