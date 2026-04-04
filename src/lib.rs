use std::process::Command;

use rofi_mode::Event;

pub struct ZedMode<'rofi> {
    api: rofi_mode::Api<'rofi>,
    entries: Vec<Entry>,
}

enum Entry {
    SearchRecents,
    OpenProject(String),
}

impl Entry {
    fn as_str(&self) -> &str {
        match self {
            Self::SearchRecents => "Search Recent Projects",
            Self::OpenProject(project) => project.as_str(),
        }
    }
}

fn get_menu_entries() -> Vec<Entry> {
    vec![Entry::SearchRecents]
}

// TODO: fetch real paths
fn get_recent_projects() -> Vec<Entry> {
    vec![
        Entry::OpenProject("/home/rwxae/code/dotfiles".to_string()),
        Entry::OpenProject("/home/rwxae/code/stylix".to_string()),
    ]
}

fn open_zed(path: &str) {
    Command::new("zeditor")
        .arg(path)
        .spawn()
        .expect("zeditor is not found");
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
                    self.entries = get_recent_projects();
                    return rofi_mode::Action::Reload;
                }
                Entry::OpenProject(path) => {
                    // TODO: handle potential errors
                    open_zed(&path);
                    return rofi_mode::Action::Exit;
                }
            },
            Event::Cancel { selected } => {
                let Some(selected) = selected else {
                    return rofi_mode::Action::Exit;
                };
                return match self.entries[*selected] {
                    Entry::SearchRecents => rofi_mode::Action::Exit,
                    Entry::OpenProject(_) => {
                        self.entries = get_menu_entries();
                        rofi_mode::Action::Reload
                    }
                };
            }
            Event::Complete { .. } => {}
            Event::CustomInput { .. } => {}
            Event::CustomCommand { .. } => {}
            Event::DeleteEntry { .. } => {}
        }
        rofi_mode::Action::Reload
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(&self.entries[line].as_str())
    }
}

rofi_mode::export_mode!(ZedMode);
