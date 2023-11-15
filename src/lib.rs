use std::fs::{DirEntry, FileType};

pub struct Entry {
    pub kind: EntryKind,
    pub name: String,
}

impl Entry {
    pub fn from_dir_entry(dir_entry: &DirEntry) -> Self {
        Entry {
            kind: dir_entry.file_type().unwrap().into(),
            name: dir_entry.file_name().to_string_lossy().to_string(),
        }
    }

    pub fn display(&self, use_emoji_icon: bool) -> String {
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
