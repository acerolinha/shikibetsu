use clap::Parser;

use crate::entries_handler::SortKey;

#[derive(Parser)]
#[command(
    name = "shikibetsu",
    bin_name = "sb",
    about = "A command-line tool for listing files and directories.",
    version = "0.1.0",
    author = "Felipe Cardoso"
)]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: std::path::PathBuf,

    #[arg(short = 'a', long = "all", default_value = "false")]
    pub show_hidden: bool,

    #[arg(short = 'c', long = "created", default_value = "false")]
    pub show_created_ts: bool,

    #[arg(short = 'd', long = "dirs", default_value = "false")]
    pub show_only_dirs: bool,

    #[arg(short = 'e', long = "emoji", default_value = "false")]
    pub show_emoji_icon: bool,

    #[arg(short = 'f', long = "files", default_value = "false")]
    pub show_only_files: bool,

    #[arg(short = 'm', long = "modified", default_value = "false")]
    pub show_modified_ts: bool,

    #[arg(short = 'r', long = "reverse", default_value = "false")]
    pub reverse: bool,

    #[arg(short = 'S', long = "size", default_value = "false")]
    pub show_size: bool,

    #[arg(short = 's', long = "sort", default_value_t = SortKey::Name)]
    pub sort_by: SortKey,
}
