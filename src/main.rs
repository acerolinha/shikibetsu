use clap::Parser;
use shikibetsu::Entry;

#[derive(Parser)]
#[command(
    name = "shikibetsu",
    bin_name = "sb",
    about = "A command-line tool for listing files and directories.",
    version = "0.1.0",
    author = "Felipe Cardoso"
)]
struct Args {
    #[arg(default_value = ".")]
    path: std::path::PathBuf,

    #[arg(short = 'e', long = "emoji", default_value = "false")]
    show_emoji_icon: bool,

    #[arg(short = 'a', long = "all", default_value = "false")]
    show_hidden: bool,

    #[arg(short = 'r', long = "reverse", default_value = "false")]
    reverse: bool,
}

fn main() {
    let args = Args::parse();

    let mut entries = std::fs::read_dir(args.path)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|f| args.show_hidden || !f.file_name().to_string_lossy().starts_with('.'))
        .map(|dir_entry| Entry::from_dir_entry(&dir_entry))
        .collect::<Vec<_>>();

    if args.reverse {
        entries.reverse();
    }

    for entry in entries {
        println!("{}", entry.display(args.show_emoji_icon));
    }
}
