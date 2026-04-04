use itertools::Itertools;
use std::{fs, io, path::PathBuf, process::Command};

use rofi_mode::Event;
use rusqlite::{Connection, OpenFlags, fallible_iterator::FallibleIterator};

pub struct ZedMode<'rofi> {
    api: rofi_mode::Api<'rofi>,
    entries: Vec<Entry>,
}

enum Entry {
    SearchRecents,
    Open(String),
}

impl Entry {
    fn as_str(&self) -> &str {
        match self {
            Self::SearchRecents => "Search Recent Projects",
            Self::Open(project) => project.as_str(),
        }
    }
}

fn get_menu_entries() -> Vec<Entry> {
    vec![Entry::SearchRecents]
}

// https://zed.dev/docs/troubleshooting?highlight=0-stable#startup-and-workspace-issues
const ZED_DB_CHANNELS: [&str; 4] = ["0-stable", "0-preview", "0-nightly", "0-dev"];

fn query_recent_projects(db: &PathBuf) -> rusqlite::Result<Vec<String>> {
    Connection::open_with_flags(db, OpenFlags::SQLITE_OPEN_READ_ONLY)?
        .prepare(
            "SELECT w.paths FROM workspaces w
            WHERE w.paths IS NOT NULL AND length(w.paths) > 0
            ORDER BY w.timestamp DESC",
        )?
        .query([])?
        .map(|row| row.get::<_, String>(0))
        .collect::<Vec<String>>()
}

fn get_recent_projects() -> Result<Vec<Entry>, &'static str> {
    let zed_db_dir = dirs::data_dir()
        .map(|path| path.join("zed/db"))
        .ok_or("Unable to determine a path to $XDG_DATA_HOME")?;
    let entries = ZED_DB_CHANNELS
        .iter()
        .map(|channel| zed_db_dir.join(channel).join("db.sqlite"))
        .filter(|path| path.try_exists().unwrap_or(false))
        .filter_map(|path| query_recent_projects(&path).ok())
        .flatten()
        .unique_by(|project| project.clone())
        .filter(|project| fs::exists(project).unwrap_or(false))
        .map(|project| Entry::Open(project))
        .collect();
    Ok(entries)
}

fn open_zed(path: &str) -> io::Result<std::process::Child> {
    Command::new("zeditor").arg(path).spawn()
}

impl<'rofi> rofi_mode::Mode<'rofi> for ZedMode<'rofi> {
    const NAME: &'static str = "zed\0";

    fn init(api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        let entries = get_menu_entries();
        Ok(Self { api, entries })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        (&self.entries[line]).as_str().into()
    }

    fn entry_icon(&mut self, _line: usize, height: u32) -> Option<rofi_mode::cairo::Surface> {
        self.api.query_icon("zed", height).wait(&mut self.api).ok()
    }

    fn react(&mut self, event: Event, _input: &mut rofi_mode::String) -> rofi_mode::Action {
        match &event {
            Event::Ok { selected, .. } => match &self.entries[*selected] {
                Entry::SearchRecents => {
                    let Ok(entries) = get_recent_projects() else {
                        // TODO: do something smarter
                        return rofi_mode::Action::Exit;
                    };
                    self.entries = entries;
                    rofi_mode::Action::Reload
                }
                Entry::Open(project) => {
                    if let Err(_) = open_zed(project) {
                        // TODO: do something smarter
                        return rofi_mode::Action::Exit;
                    }
                    rofi_mode::Action::Exit
                }
            },
            Event::Cancel { selected } => {
                let Some(selected) = selected else {
                    return rofi_mode::Action::Exit;
                };
                match self.entries[*selected] {
                    Entry::SearchRecents => rofi_mode::Action::Exit,
                    Entry::Open(_) => {
                        self.entries = get_menu_entries();
                        rofi_mode::Action::Reload
                    }
                }
            }
            _ => rofi_mode::Action::Reset,
            // Event::Complete { .. } => {}
            // Event::CustomInput { .. } => {}
            // Event::CustomCommand { .. } => {}
            // Event::DeleteEntry { .. } => {}
        }
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(&self.entries[line].as_str())
    }
}

rofi_mode::export_mode!(ZedMode);
