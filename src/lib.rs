struct Mode<'rofi> {
    api: rofi_mode::Api<'rofi>,
    entries: Vec<String>
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "zed\0";

    fn init(api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        let mut entries = Vec::new();
        entries.push("nixos    <span weight='normal' alpha='50%'>~/code/nixos</span>".to_string());
        entries.push("nixos2    <span weight='normal' alpha='50%'>~/code/nixos2</span>".to_string());
        Ok(Self {
            api,
            entries
        })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_style(&self, _line: usize)  -> rofi_mode::Style {
        rofi_mode::Style::MARKUP
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        (&self.entries[line]).into()
    }

    fn entry_icon(&mut self, _line: usize, height: u32) -> Option<rofi_mode::cairo::Surface> {
        self.api
            .query_icon("zed", height)
            .wait(&mut self.api)
            .ok()
    }

    fn react(
        &mut self,
        _event: rofi_mode::Event,
        _input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        rofi_mode::Action::Exit
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(&self.entries[line])
    }
}

rofi_mode::export_mode!(Mode);
