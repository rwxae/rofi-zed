mod zed;

use rofi_mode::{Action, Event};

pub struct ZedMode<'rofi> {
    api: rofi_mode::Api<'rofi>,
    entries: Vec<String>,
}

impl<'rofi> rofi_mode::Mode<'rofi> for ZedMode<'rofi> {
    const NAME: &'static str = "zed\0";

    fn init(api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        let entries = zed::get_recent_projects().ok_or(())?;
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

    fn react(&mut self, event: Event, _input: &mut rofi_mode::String) -> Action {
        match &event {
            Event::Ok { selected, .. } => match zed::open_project(&self.entries[*selected]) {
                Ok(_) => Action::Exit,
                // TODO: report an error
                Err(_) => Action::Exit,
            },
            Event::Cancel { .. } => Action::Exit,
            _ => Action::Reset,
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
