use std::path::Path;

use crate::args::Args;
use crate::entry::{Entry, EntryKind};

pub struct EntriesHandler {
    filter_options: FilterOptions,
    sort_options: SortOptions,
}

struct FilterOptions {
    show_hidden: bool,
    show_only_dirs: bool,
}

impl From<&Args> for FilterOptions {
    fn from(item: &Args) -> Self {
        FilterOptions {
            show_hidden: item.show_hidden,
            show_only_dirs: item.show_only_dirs,
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

        if self.sort_options.reverse {
            entries.reverse();
        }

        entries
    }
}
