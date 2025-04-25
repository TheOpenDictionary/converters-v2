use clap::{Subcommand, command};

use crate::args::{FreeDictArgs, WiktionaryArgs};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Wiktionary(WiktionaryArgs),
    #[command(name = "cedict")]
    CEDict,
    #[command(name = "freedict")]
    FreeDict(FreeDictArgs),
}
