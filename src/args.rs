use clap::{Parser, Subcommand};

use crate::{information::Information, state::State};

#[derive(Debug, Subcommand)]
pub(super) enum Commands {
    /// Send information request of a given kind.
    #[clap(subcommand)]
    Information(Information),
    /// Send record request with a given ID and key.
    #[command(
        after_help = "Please refer to `enum RecordId` from the casper-node repository for valid record IDs."
    )]
    Record {
        #[clap(long, short)]
        id: u16,
        /// Hex-encoded key.
        #[clap(long, short)]
        key: String,
    },
    /// Retrieves data from the global state.
    #[clap(subcommand)]
    State(State),
    /// Sends a transaction to the network for inclusion.
    TryAcceptTransaction {
        #[clap(long, short)]
        transaction_file: String,
    },
}

/// A request to the binary access interface.
#[derive(Parser, Debug)]
pub(super) struct Args {
    #[clap(subcommand)]
    pub(super) commands: Commands,
    // TODO[RC]: Currently unused.
    #[clap(long, short, default_value = "false")]
    pub(super) verbose: bool,
    #[clap(long, short, default_value = "false", required = true)]
    pub(super) node_address: String,
}
