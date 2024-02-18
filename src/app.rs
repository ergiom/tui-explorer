use std::{error, fs::{read_dir, DirEntry}, path::{Path, PathBuf}};

use ratatui::widgets::ListState;

use crate::file_types::FileType;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub path: PathBuf,
    pub error: String,
    pub list_state: ListState,
    items: Vec<DirEntry>,
}

impl Default for App {
    fn default() -> Self {
        let path = std::env::current_dir().expect("Couldn't read current dir");

        Self {
            running: true,
            path: path.clone(),
            error: "".into(),
            list_state: ListState::default()
                .with_selected(Some(0)),
            items: dir_items(path.into_boxed_path()),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn current_dir(&self) -> Box<Path> {
        self.path.clone().into_boxed_path()
    }

    pub fn current_dir_items(&self) -> Vec<(FileType, String)> {
        self.items.iter()
            .filter_map(|entry| {
                let file_type = entry.file_type().ok().map(std::fs::FileType::into);
                let file_name = entry.file_name().into_string().ok();

                match (file_type, file_name) {
                    (Some(file_type), Some(file_name)) => Some((file_type, file_name)),
                    (_, _) => None,
                }
            })
            .collect()
    }

    pub fn select_next_item(&mut self) {
        if self.has_error() {
            return;
        }
        let total = self.items.len();

        if let Some(current) = self.list_state.selected() {
            let new_index = std::cmp::min(current + 1, total - 1);
            self.list_state.select(Some(new_index));
        }
    }

    pub fn select_previous_item(&mut self) {
        if self.has_error() {
            return;
        }
        if let Some(current) = self.list_state.selected() {
            let new_index = current.saturating_sub(1);
            self.list_state.select(Some(new_index));
        }
    }

    pub fn go_back(&mut self) {
        if self.has_error() {
            return;
        }
        if let Some(path) = self.path.parent() {
            let new_path = path.to_owned();
            self.items = self.dir_items(new_path.clone().into_boxed_path());
            self.path = PathBuf::from(new_path.to_path_buf());
            self.list_state.select(0.into());
        }
    }

    pub fn go_into(&mut self) {
        if self.has_error() {
            return;
        }
        let items = self.current_dir_items();
        if let Some((file_type, file_name)) = self.list_state.selected()
            .and_then(|i| items.get(i)) {
                if let FileType::DIR = file_type {
                    self.path.push(file_name);
                    self.items = self.dir_items(self.path.clone().into_boxed_path());
                    self.list_state.select(0.into());
                }
            }
    }

    pub fn has_error(&mut self) -> bool {
        !self.error.is_empty()
    }

    pub fn clear_error(&mut self) {
        if self.has_error() {
            self.error.clear();
            self.go_back();
        }
    }

    fn dir_items(&mut self, path: Box<Path>) -> Vec<DirEntry> {
        match read_dir(path) {
            Ok(iter) => {
                iter.filter_map(Result::ok)
                    .collect()

            },
            Err(error) => {
                self.error.push_str(error.to_string().as_str());
                Vec::new()
            },
        }
    }
}

fn dir_items(path: Box<Path>) -> Vec<DirEntry> {
    read_dir(path).expect("").filter_map(Result::ok).collect()
}
