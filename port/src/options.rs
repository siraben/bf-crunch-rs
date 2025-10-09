//! Command-line configuration for the Brainf**k cruncher CLI.

use clap::{ArgAction, Parser};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "bfcrunch",
    disable_help_flag = true,
    disable_help_subcommand = true,
    override_usage = "bfcrunch [--options] text [limit]",
    about = "Crunches BF programs to produce a given text."
)]
/// Parsed command-line options controlling the cruncher search.
pub struct Options {
    /// The text to produce.
    #[arg(value_name = "text", help = "The text to produce.")]
    pub text: String,

    /// The maximum BF program length to search for. If empty, the length of the shortest program found so far will be used (-r).
    #[arg(
        value_name = "limit",
        help = "The maximum BF program length to search for. If empty, the length of the shortest program found so far will be used (-r). Default = (empty)"
    )]
    pub limit: Option<i32>,

    /// The maximum length of the initialization segment. If empty, the program will run indefinitely.
    #[arg(
        long = "max-init",
        short = 'i',
        value_name = "#",
        help = "The maximum length of the initialization segment. If empty, the program will run indefinitely. Default = (empty)"
    )]
    pub max_init: Option<i32>,

    /// The minimum length of the initialization segment.
    #[arg(
        long = "min-init",
        short = 'I',
        value_name = "#",
        default_value_t = 14,
        hide_default_value = true,
        help = "The minimum length of the initialization segment. Default = 14"
    )]
    pub min_init: i32,

    /// The maximum tape size to consider. Programs that utilize more tape than this will be ignored.
    #[arg(
        long = "max-tape",
        short = 't',
        value_name = "#",
        default_value_t = 1250,
        hide_default_value = true,
        help = "The maximum tape size to consider. Programs that utilize more tape than this will be ignored. Default = 1250"
    )]
    pub max_tape: i32,

    /// The minimum tape size to consider. Programs that utilize less tape than this will be ignored.
    #[arg(
        long = "min-tape",
        short = 'T',
        value_name = "#",
        default_value_t = 1,
        hide_default_value = true,
        help = "The minimum tape size to consider. Programs that utilize less tape than this will be ignored. Default = 1"
    )]
    pub min_tape: i32,

    /// The maximum cost for any node.
    #[arg(
        long = "max-node-cost",
        short = 'n',
        value_name = "#",
        default_value_t = 20,
        hide_default_value = true,
        help = "The maximum cost for any node. Default = 20"
    )]
    pub max_node_cost: i32,

    /// The maximum number of iterations of the main loop.
    #[arg(
        long = "max-loops",
        short = 'l',
        value_name = "#",
        default_value_t = 30_000,
        hide_default_value = true,
        help = "The maximum number of iterations of the main loop. Default = 30000"
    )]
    pub max_loops: i32,

    /// The maximum length of the s-segment.
    #[arg(
        long = "max-slen",
        short = 's',
        value_name = "#",
        help = "The maximum length of the s-segment. Default = (empty)"
    )]
    pub max_slen: Option<i32>,

    /// The minimum length of the s-segment.
    #[arg(
        long = "min-slen",
        short = 'S',
        value_name = "#",
        default_value_t = 1,
        hide_default_value = true,
        help = "The minimum length of the s-segment. Default = 1"
    )]
    pub min_slen: i32,

    /// The maximum length of the c-segment.
    #[arg(
        long = "max-clen",
        short = 'c',
        value_name = "#",
        help = "The maximum length of the c-segment. Default = (empty)"
    )]
    pub max_clen: Option<i32>,

    /// The minimum length of the c-segment.
    #[arg(
        long = "min-clen",
        short = 'C',
        value_name = "#",
        default_value_t = 1,
        hide_default_value = true,
        help = "The minimum length of the c-segment. Default = 1"
    )]
    pub min_clen: i32,

    /// If set, the limit will be adjusted whenever a shorter program is found.
    #[arg(
        long = "rolling-limit",
        short = 'r',
        action = ArgAction::SetTrue,
        help = "If set, the limit will be adjusted whenever a shorter program is found."
    )]
    pub rolling_limit: bool,

    /// If set, each used cell used for output will be unique.
    #[arg(
        long = "unique-cells",
        short = 'u',
        action = ArgAction::SetTrue,
        help = "If set, each used cell used for output will be unique."
    )]
    pub unique_cells: bool,

    /// Print the full BF program for each solution.
    #[arg(
        long = "full-program",
        action = ArgAction::SetTrue,
        help = "Print the full BF program for each solution."
    )]
    pub full_program: bool,

    /// Display this help text.
    #[arg(short = '?', long = "help", action = ArgAction::Help, help = "Display this help text.")]
    pub _help: Option<bool>,
}
