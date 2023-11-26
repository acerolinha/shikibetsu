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

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use regex::Regex;
    use std::fs;

    #[test]
    fn it_should_create_entry_from_dir_entry() {
        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("child_dir").create_dir_all().unwrap();
        let file = temp.child("file");
        file.touch().unwrap();
        temp.child("symlink").symlink_to_file(file.path()).unwrap();

        let entries = fs::read_dir(temp.path())
            .unwrap()
            .into_iter()
            .map(|e| Entry::from_dir_entry(&e.unwrap()))
            .collect::<Vec<Entry>>();

        assert_eq!(entries.len(), 3);
        assert_eq!(entries.iter().any(|e| e.kind == EntryKind::Dir), true);
        assert_eq!(entries.iter().any(|e| e.kind == EntryKind::File), true);
        assert_eq!(entries.iter().any(|e| e.kind == EntryKind::Symlink), true);
        assert_eq!(entries.iter().any(|e| e.name == "child_dir"), true);
        assert_eq!(entries.iter().any(|e| e.name == "file"), true);
        assert_eq!(entries.iter().any(|e| e.name == "symlink"), true);
    }

    #[test]
    fn it_should_create_display_options() {
        let args = Args::default();
        let display_options = DisplayOptions::from(&args);
        assert_eq!(display_options.show_emoji_icon, false);
        assert_eq!(display_options.show_modified_ts, false);
        assert_eq!(display_options.show_created_ts, false);
        assert_eq!(display_options.show_size, false);
        assert_eq!(display_options.show_permissions, false);
    }

    #[test]
    fn it_should_format_st_mode() {
        assert_eq!(Entry::format_st_mode(0o644), "[rw-|r--|r--]");
        assert_eq!(Entry::format_st_mode(0o755), "[rwx|r-x|r-x]");
        assert_eq!(Entry::format_st_mode(0o777), "[rwx|rwx|rwx]");
    }

    #[test]
    fn it_should_display_permissions() {
        let display_options = DisplayOptions {
            show_emoji_icon: false,
            show_modified_ts: false,
            show_created_ts: false,
            show_size: false,
            show_permissions: true,
        };

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("file").touch().unwrap();

        let mut read_dir = fs::read_dir(temp.path()).unwrap().into_iter();
        let file_entry = Entry::from_dir_entry(&read_dir.next().unwrap().unwrap());

        let re =
            Regex::new(r"^\[F\]─\[(r|-)(w|-)(x|-)\|(r|-)(w|-)(x|-)\|(r|-)(w|-)(x|-)\]─\[file\]$")
                .unwrap();

        assert!(re.is_match(file_entry.display(&display_options).as_str()));
    }

    #[test]
    fn it_should_display_modified_ts() {
        let display_options = DisplayOptions {
            show_emoji_icon: false,
            show_modified_ts: true,
            show_created_ts: false,
            show_size: false,
            show_permissions: false,
        };

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("file").touch().unwrap();

        let mut read_dir = fs::read_dir(temp.path()).unwrap().into_iter();
        let file_entry = Entry::from_dir_entry(&read_dir.next().unwrap().unwrap());
        assert_eq!(
            file_entry.display(&display_options),
            "[F]─[M: now           ]─[file]"
        );
    }

    #[test]
    fn it_should_display_created_ts() {
        let display_options = DisplayOptions {
            show_emoji_icon: false,
            show_modified_ts: false,
            show_created_ts: true,
            show_size: false,
            show_permissions: false,
        };

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("file").touch().unwrap();

        let mut read_dir = fs::read_dir(temp.path()).unwrap().into_iter();
        let file_entry = Entry::from_dir_entry(&read_dir.next().unwrap().unwrap());
        assert_eq!(
            file_entry.display(&display_options),
            "[F]─[C: now           ]─[file]"
        );
    }

    #[test]
    fn it_should_display_size() {
        let display_options = DisplayOptions {
            show_emoji_icon: false,
            show_modified_ts: false,
            show_created_ts: false,
            show_size: true,
            show_permissions: false,
        };

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("file").touch().unwrap();

        let mut read_dir = fs::read_dir(temp.path()).unwrap().into_iter();
        let file_entry = Entry::from_dir_entry(&read_dir.next().unwrap().unwrap());
        assert_eq!(
            file_entry.display(&display_options),
            "[F]─[S: 0 B       ]─[file]"
        );
    }

    #[test]
    fn it_should_display_default_icons() {
        let display_options = DisplayOptions {
            show_emoji_icon: false,
            show_modified_ts: false,
            show_created_ts: false,
            show_size: false,
            show_permissions: false,
        };

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("file").touch().unwrap();
        temp.child("dir").create_dir_all().unwrap();
        temp.child("symlink")
            .symlink_to_file(temp.child("file").path())
            .unwrap();

        let mut actual_display = String::new();

        for entry in fs::read_dir(temp.path()).unwrap().into_iter() {
            let entry = Entry::from_dir_entry(&entry.unwrap());
            actual_display.push_str(&entry.display(&display_options));
            actual_display.push('\n');
        }

        assert!(actual_display.contains("[F]─[file]"));
        assert!(actual_display.contains("[D]─[dir]"));
        assert!(actual_display.contains("[L]─[symlink]"));
    }

    #[test]
    fn it_should_display_emoji_icons() {
        let display_options = DisplayOptions {
            show_emoji_icon: true,
            show_modified_ts: false,
            show_created_ts: false,
            show_size: false,
            show_permissions: false,
        };

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("file").touch().unwrap();
        temp.child("dir").create_dir_all().unwrap();
        temp.child("symlink")
            .symlink_to_file(temp.child("file").path())
            .unwrap();

        let mut actual_display = String::new();

        for entry in fs::read_dir(temp.path()).unwrap().into_iter() {
            let entry = Entry::from_dir_entry(&entry.unwrap());
            actual_display.push_str(&entry.display(&display_options));
            actual_display.push('\n');
        }

        assert!(actual_display.contains("[📄]─[file]"));
        assert!(actual_display.contains("[📁]─[dir]"));
        assert!(actual_display.contains("[🔗]─[symlink]"));
    }
}
