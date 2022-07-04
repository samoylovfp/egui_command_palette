use egui::{Id, Key};

pub struct CommandPalette {
    is_open: bool,
    search_term: String,
    prev_element_focus: Option<Id>,
}

impl CommandPalette {
    pub fn new() -> Self {
        CommandPalette {
            is_open: false,
            search_term: String::new(),
            prev_element_focus: None,
        }
    }
}

impl CommandPalette {
    /// Opens the search for commands window
    pub fn open(&mut self, prev_element_focus: Option<Id>) {
        if !self.is_open {
            self.prev_element_focus = dbg!(prev_element_focus);
        }
        self.is_open = true;
    }
    /// Renders the search for command window
    pub fn render(&mut self, ctx: &egui::Context) {
        if !self.is_open {
            return;
        }
        egui::Window::new("command palette")
            .collapsible(false)
            .show(ctx, |ui| {
                if ui.input().key_pressed(Key::Escape) {
                    self.is_open = false;
                }

                ui.label("Type the command");
                let search_input = ui.text_edit_singleline(&mut self.search_term);
                if self.is_open {
                    search_input.request_focus();
                } else if let Some(id) = self.prev_element_focus {
                    ctx.memory().request_focus(id);
                }
            });
    }
}
