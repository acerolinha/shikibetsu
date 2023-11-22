use std::{
    fs::{DirEntry, FileType},
    os::unix::fs::PermissionsExt,
    time::SystemTime,
};

use file_mode::Mode;
use humansize::DECIMAL;

use crate::args::Args;

pub struct Entry {
    pub kind: EntryKind,
    pub name: String,
    pub size: u64,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
    pub permissions: u32,
    pub children: Vec<Entry>,
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
            permissions: metadata.permissions().mode(),
            children: vec![],
        }
    }

    fn format_st_mode(st_mode: u32) -> String {
        let perms = Mode::from(st_mode)
            .to_string()
            .chars()
            .skip(1)
            .collect::<String>();
        let mut result = String::with_capacity(11);

        for (i, c) in perms.chars().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push('|');
            }
            result.push(c);
        }

        format!("[{}]", result)
    }

    pub fn display(&self, display_options: &DisplayOptions) -> String {
        let mut metadata = vec![];

        if display_options.show_permissions {
            metadata.push(format!("─{}", Self::format_st_mode(self.permissions)));
        }

        if display_options.show_modified_ts {
            metadata.push(format!(
                "─[M: {: <14}]",
                timeago::Formatter::new().convert(self.mtime.elapsed().unwrap())
            ));
        }

        if display_options.show_created_ts {
            metadata.push(format!(
                "─[C: {: <14}]",
                timeago::Formatter::new().convert(self.ctime.elapsed().unwrap())
            ));
        }

        if display_options.show_size {
            metadata.push(format!(
                "─[S: {: <10}]",
                humansize::format_size(self.size, DECIMAL)
            ));
        }

        let metadata = metadata.iter().fold(String::new(), |acc, e| acc + e);
        format!(
            "[{}]{}─[{}]",
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

    pub fn display_recursive(&self, display_options: &DisplayOptions, depth: usize) {
        println!(
            "{:<depth$}{}{}",
            "",
            if depth > 0 { "└" } else { "" },
            self.display(display_options),
        );

        for child in self.children.iter() {
            child.display_recursive(display_options, depth + 1);
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
