use std::{
    fs::{DirEntry, FileType, Permissions},
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
    pub permissions: Permissions,
}

pub struct DisplayOptions {
    show_emoji_icon: bool,
    show_modified_ts: bool,
    show_created_ts: bool,
    show_size: bool,
    show_permissions: bool,
}

impl From<&Args> for DisplayOptions {
    fn from(item: &Args) -> Self {
        DisplayOptions {
            show_emoji_icon: item.show_emoji_icon,
            show_modified_ts: item.show_modified_ts,
            show_created_ts: item.show_created_ts,
            show_size: item.show_size,
            show_permissions: item.show_permissions,
        }
    }
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
            permissions: metadata.permissions(),
        }
    }

    pub fn display(&self, display_options: &DisplayOptions) -> String {
        let mut metadata = vec![];
        if display_options.show_modified_ts {
            metadata.push(format!(
                "-[M: {: <12}]",
                timeago::Formatter::new().convert(self.mtime.elapsed().unwrap())
            ));
        }
        if display_options.show_created_ts {
            metadata.push(format!(
                "-[C: {: <12}]",
                timeago::Formatter::new().convert(self.ctime.elapsed().unwrap())
            ));
        }
        if display_options.show_size {
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
            self.get_icon(display_options.show_emoji_icon),
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
