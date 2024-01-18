use clap::{Parser, ValueEnum};
use memflow::plugins::Inventory;

use crate::dma::Connector;

#[derive(Parser)]
#[command(author, version = version(), about, long_about = None)]
pub struct Cli {
    /// Specifies the connector type for DMA
    #[clap(value_enum, short, long, ignore_case = true, default_value_t = Connector::Qemu)]
    pub connector: Connector,

    /// Name of the Pcileech device
    #[clap(long, default_value_t = String::from("FPGA"))]
    pub pcileech_device: String,
}

fn version() -> String {
    let pkg_ver = env!("CARGO_PKG_VERSION");
    let git_hash = option_env!("VERGEN_GIT_SHA").unwrap_or("unknown");
    let commit_date = option_env!("VERGEN_GIT_COMMIT_DATE").unwrap_or("unknown");
    let avail_cons = {
        let inventory = Inventory::scan();
        inventory.available_connectors().join(", ")
    };

    format!(" {pkg_ver} (rev {git_hash})\nCommit Date: {commit_date}\nAvailable Connectors: {avail_cons}")
}

/// Wrapper because log::LevelFilter doesn't implement ValueEnum
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum Loglevel {
    Error,
    #[default]
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<Loglevel> for log::LevelFilter {
    fn from(val: Loglevel) -> Self {
        match val {
            Loglevel::Error => log::LevelFilter::Error,
            Loglevel::Warn => log::LevelFilter::Warn,
            Loglevel::Info => log::LevelFilter::Info,
            Loglevel::Debug => log::LevelFilter::Debug,
            Loglevel::Trace => log::LevelFilter::Trace,
        }
    }
}