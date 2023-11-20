use std::{
    fs::{DirEntry, FileType},
    time::SystemTime,
};

use humansize::DECIMAL;

use crate::args::Args;

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
        if args.show_modified_ts {
            metadata.push(format!(
                "-[M: {: <12}]",
                timeago::Formatter::new().convert(self.mtime.elapsed().unwrap())
            ));
        }
        if args.show_created_ts {
            metadata.push(format!(
                "-[C: {: <12}]",
                timeago::Formatter::new().convert(self.ctime.elapsed().unwrap())
            ));
        }
        if args.show_size {
            metadata.push(format!(
                "-[S: {: <10}]",
                humansize::format_size(self.size, DECIMAL)
            ));
        }
        let metadata = metadata
            .iter()
            .map(|e| format!("{}", e))
            .collect::<String>();
        format!(
            "[{}]{: <10}-[{}]",
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

#[derive(PartialEq)]
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
