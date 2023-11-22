use std::fmt::{self, Display, Formatter};
use std::path::Path;

use crate::args::Args;
use crate::entry::{Entry, EntryKind};
use clap::builder::PossibleValue;
use clap::ValueEnum;

pub struct EntriesHandler {
    filter_options: FilterOptions,
    sort_options: SortOptions,
}

struct FilterOptions {
    show_hidden: bool,
    show_only_dirs: bool,
    show_only_files: bool,
}

impl From<&Args> for FilterOptions {
    fn from(item: &Args) -> Self {
        FilterOptions {
            show_hidden: item.show_hidden,
            show_only_dirs: item.show_only_dirs,
            show_only_files: item.show_only_files,
        }
    }
}

#[derive(Clone)]
pub enum SortKey {
    Name,
    Created,
    Modified,
    Size,
}

impl ValueEnum for SortKey {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            SortKey::Name,
            SortKey::Created,
            SortKey::Modified,
            SortKey::Size,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            SortKey::Name => Some(PossibleValue::new("n")),
            SortKey::Created => Some(PossibleValue::new("c")),
            SortKey::Modified => Some(PossibleValue::new("m")),
            SortKey::Size => Some(PossibleValue::new("s")),
        }
    }
}

impl Display for SortKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            SortKey::Name => "n",
            SortKey::Created => "c",
            SortKey::Modified => "m",
            SortKey::Size => "s",
        };
        write!(f, "{s}")
    }
}

struct SortOptions {
    reverse: bool,
    sort_key: SortKey,
}

impl From<&Args> for SortOptions {
    fn from(item: &Args) -> Self {
        SortOptions {
            reverse: item.reverse,
            sort_key: item.sort_by.clone(),
        }
    }
}

impl EntriesHandler {
    pub fn new(args: &Args) -> Self {
        let filter_options = FilterOptions::from(args);
        let sort_options = SortOptions::from(args);

        Self {
            filter_options,
            sort_options,
        }
    }

    pub fn get_entries(&self, path: &Path) -> Vec<Entry> {
        let mut entries = std::fs::read_dir(path)
            .expect("Failed to read directory")
            .filter_map(Result::ok)
            .filter(|f| {
                self.filter_options.show_hidden || !f.file_name().to_string_lossy().starts_with('.')
            })
            .map(|dir_entry| Entry::from_dir_entry(&dir_entry))
            .collect::<Vec<_>>();

        if self.filter_options.show_only_dirs {
            entries = entries
                .into_iter()
                .filter(|e| e.kind == EntryKind::Dir)
                .collect::<Vec<_>>();
        }

        if self.filter_options.show_only_files {
            entries = entries
                .into_iter()
                .filter(|e| e.kind == EntryKind::File)
                .collect::<Vec<_>>();
        }

        match self.sort_options.sort_key {
            SortKey::Name => entries.sort_by(|a, b| a.name.cmp(&b.name)),
            SortKey::Created => entries.sort_by(|a, b| a.ctime.cmp(&b.ctime)),
            SortKey::Modified => entries.sort_by(|a, b| a.mtime.cmp(&b.mtime)),
            SortKey::Size => entries.sort_by(|a, b| a.size.cmp(&b.size)),
        }

        if self.sort_options.reverse {
            entries.reverse();
        }

        entries
    }

    pub fn get_entries_recursive(&self, path: &Path) -> Vec<Entry> {
        let mut entries = self.get_entries(path);

        for entry in entries.iter_mut() {
            if entry.kind == EntryKind::Dir {
                entry.children = self.get_entries_recursive(&path.join(&entry.name));
            }
        }

        entries
    }
}
