use itertools::Itertools;
use rusqlite::{Connection, OpenFlags, fallible_iterator::FallibleIterator};
use std::{fs, io, path::PathBuf, process};

// https://zed.dev/docs/troubleshooting?highlight=0-stable#startup-and-workspace-issues
const ZED_DB_CHANNELS: [&str; 4] = ["0-stable", "0-preview", "0-nightly", "0-dev"];

// TODO: why paths is plural?
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

pub fn get_recent_projects() -> Option<Vec<String>> {
    let zed_db_dir = dirs::data_dir().map(|path| path.join("zed/db"))?;
    let entries = ZED_DB_CHANNELS
        .iter()
        .map(|channel| zed_db_dir.join(channel).join("db.sqlite"))
        .filter(|path| path.try_exists().unwrap_or(false))
        .filter_map(|path| query_recent_projects(&path).ok())
        .flatten()
        .unique_by(|project| project.clone())
        .filter(|project| fs::exists(project).unwrap_or(false))
        .collect();
    Some(entries)
}

pub fn open_project(path: &str) -> io::Result<std::process::Child> {
    process::Command::new("zeditor").arg(path).spawn()
}
