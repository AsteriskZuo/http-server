use serde::Serialize;
use std::cmp::{Ord, Ordering};

/// A Directory entry used to display a File Explorer's entry.
/// This struct is directly related to the Handlebars template used
/// to power the File Explorer's UI
#[derive(Debug, Eq, Serialize)]
pub struct DirectoryEntry {
    display_name: String,
    is_dir: bool,
    size: String,
    entry_path: String,
    created_at: String,
    updated_at: String,
}

impl Ord for DirectoryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_dir && other.is_dir {
            return self.display_name.cmp(&other.display_name);
        }

        if self.is_dir && !other.is_dir {
            return Ordering::Less;
        }

        Ordering::Greater
    }
}

impl PartialOrd for DirectoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_dir && other.is_dir {
            return Some(self.display_name.cmp(&other.display_name));
        }

        if self.is_dir && !other.is_dir {
            return Some(Ordering::Less);
        }

        Some(Ordering::Greater)
    }
}

impl PartialEq for DirectoryEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.is_dir && other.is_dir {
            return self.display_name == other.display_name;
        }

        self.display_name == other.display_name
    }
}

/// The value passed to the Handlebars template engine.
/// All references contained in File Explorer's UI are provided
/// via the `DirectoryIndex` struct
#[derive(Debug, Serialize)]
pub struct DirectoryIndex {
    /// Directory listing entry
    entries: Vec<DirectoryEntry>,
}
