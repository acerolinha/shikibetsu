use std::fs::{DirEntry, FileType};

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

    #[arg(short = 'e', long = "emoji", default_value = "false")]
    show_emoji_icon: bool,
}

struct Entry {
    kind: EntryKind,
    name: String,
}

impl Entry {
    fn from_dir_entry(dir_entry: &DirEntry) -> Self {
        Entry {
            kind: dir_entry.file_type().unwrap().into(),
            name: dir_entry.file_name().to_string_lossy().to_string(),
        }
    }

    fn display(&self, use_emoji_icon: bool) -> String {
        format!("[{}][{}]", self.get_icon(use_emoji_icon), self.name)
    }

    fn get_icon(&self, use_emoji_icon: bool) -> &str {
        match self.kind {
            EntryKind::Dir => {
                if use_emoji_icon {
                    "📁"
                } else {
                    "D"
                }
            }
            EntryKind::File => {
                if use_emoji_icon {
                    "📄"
                } else {
                    "F"
                }
            }
            EntryKind::Symlink => {
                if use_emoji_icon {
                    "🔗"
                } else {
                    "L"
                }
            }
        }
    }
}

enum EntryKind {
    Dir,
    File,
    Symlink,
}

impl From<FileType> for EntryKind {
    fn from(value: FileType) -> Self {
        if value.is_dir() {
            EntryKind::Dir
        } else if value.is_file() {
            EntryKind::File
        } else if value.is_symlink() {
            EntryKind::Symlink
        } else {
            panic!("Unknown file type")
        }
    }
}

fn main() {
    let args = Args::parse();

    let entries = std::fs::read_dir(args.path)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .map(|dir_entry| Entry::from_dir_entry(&dir_entry))
        .collect::<Vec<_>>();

    for entry in entries {
        println!("{}", entry.display(args.show_emoji_icon));
    }
}
