use std::{error, fs::read_dir, path::{Path, PathBuf}};

use ratatui::widgets::ListState;

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
    items: Vec<String>,
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

    pub fn current_dir_items(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn select_next_item(&mut self) {
        let total = self.items.len();

        if let Some(current) = self.list_state.selected() {
            let new_index = std::cmp::min(current + 1, total - 1);
            self.list_state.select(Some(new_index));
        }
    }

    pub fn select_previous_item(&mut self) {
        if let Some(current) = self.list_state.selected() {
            let new_index = current.saturating_sub(1);
            self.list_state.select(Some(new_index));
        }
    }
}

fn dir_items(path: Box<Path>) -> Vec<String> {
    match read_dir(path) {
        Ok(iter) => {
            iter.filter_map(Result::ok)
                .map(|entry| entry.file_name())
                .map(|name| name.into_string())
                .filter_map(Result::ok)
                .collect()

        },
        Err(_) => todo!(),
    }
}
