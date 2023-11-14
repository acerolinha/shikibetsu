use clap::Parser;

#[derive(Parser)]
#[command(
    name = "sb",
    about = "A command-line tool for listing files and directories.",
    version = "0.1.0",
    author = "Felipe Cardoso"
)]
struct Args {
    #[arg(default_value = ".")]
    path: std::path::PathBuf,

    #[arg(short, long, default_value = "false")]
    emoji: bool,
}

fn main() {
    let args = Args::parse();

    let entries = std::fs::read_dir(args.path)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    for entry in entries {
        let entry_type = match (entry.file_type(), args.emoji) {
            (Ok(t), _) if t.is_dir() => {
                if args.emoji {
                    "📁"
                } else {
                    "D"
                }
            }
            (Ok(t), _) if t.is_file() => {
                if args.emoji {
                    "📄"
                } else {
                    "F"
                }
            }
            (Ok(t), _) if t.is_symlink() => {
                if args.emoji {
                    "🔗"
                } else {
                    "L"
                }
            }
            _ => "?",
        };
        println!("{} {}", entry_type, entry.file_name().to_string_lossy());
    }
}
