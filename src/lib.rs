use clap::Parser;
use std::{
    fs::{DirEntry, FileType},
    path::Path,
    time::SystemTime,
};

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

    #[arg(short = 'e', long = "emoji", default_value = "false")]
    show_emoji_icon: bool,

    #[arg(short = 'a', long = "all", default_value = "false")]
    show_hidden: bool,

    #[arg(short = 'r', long = "reverse", default_value = "false")]
    reverse: bool,

    #[arg(short = 'S', long = "size", default_value = "false")]
    show_size: bool,

    #[arg(short = 'm', long = "modified", default_value = "false")]
    show_modified_ts: bool,

    #[arg(short = 'c', long = "created", default_value = "false")]
    show_created_ts: bool,
}

pub struct Entry {
    pub kind: EntryKind,
    pub name: String,
    pub size: u64,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
}

impl Entry {
    pub fn from_dir_entry(dir_entry: &DirEntry) -> Self {
        let metadata = dir_entry.metadata().unwrap();
        Entry {
            kind: dir_entry.file_type().unwrap().into(),
            name: dir_entry.file_name().to_string_lossy().to_string(),
            size: metadata.len(),
            mtime: metadata.modified().unwrap(),
            ctime: metadata.created().unwrap(),
        }
    }

    pub fn display(&self, args: &Args) -> String {
        let mut metadata = vec![];
        if args.show_size {
            metadata.push(format!("-[{:8}]-", self.size));
        }
        if args.show_modified_ts {
            metadata.push(format!(
                "-[{:8}]-",
                timeago::Formatter::new().convert(self.mtime.elapsed().unwrap())
            ));
        }
        if args.show_created_ts {
            metadata.push(format!(
                "-[{:8}]-",
                timeago::Formatter::new().convert(self.ctime.elapsed().unwrap())
            ));
        }
        let metadata = metadata
            .iter()
            .map(|e| format!("{}", e))
            .collect::<String>();
        format!(
            "[{}]{}[{}]",
            self.get_icon(args.show_emoji_icon),
            metadata,
            self.name
        )
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

pub enum EntryKind {
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

struct FilterOptions {
    show_hidden: bool,
}

impl From<&Args> for FilterOptions {
    fn from(item: &Args) -> Self {
        FilterOptions {
            show_hidden: item.show_hidden,
        }
    }
}

struct SortOptions {
    reverse: bool,
}

impl From<&Args> for SortOptions {
    fn from(item: &Args) -> Self {
        SortOptions {
            reverse: item.reverse,
        }
    }
}

fn get_entries(
    path: &Path,
    filter_options: &FilterOptions,
    sort_options: &SortOptions,
) -> Vec<Entry> {
    let mut entries = std::fs::read_dir(path)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|f| filter_options.show_hidden || !f.file_name().to_string_lossy().starts_with('.'))
        .map(|dir_entry| Entry::from_dir_entry(&dir_entry))
        .collect::<Vec<_>>();

    if sort_options.reverse {
        entries.reverse();
    }

    entries
}

pub fn run_with_args(args: &Args) {
    let filter_options = FilterOptions::from(args);
    let sort_options = SortOptions::from(args);
    let entries = get_entries(&args.path, &filter_options, &sort_options);

    for entry in entries {
        println!("{}", entry.display(args));
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn it_should_throw_when_path_does_not_exist() {
        let mut cmd = Command::cargo_bin("shikibetsu").unwrap();

        cmd.arg("./this/path/does/not/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to read directory"));
    }
}
