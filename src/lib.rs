//! disrupts the way you write actions
//! Unfortunately
//! the palette cannot activate a piece of code
//! that is written in a hidden widget
//! since the "ui" function of it is never called

use std::{collections::HashSet, hash::Hash};

use egui::{Id, Key};

pub struct CommandPalette {
    is_open: bool,
    search_term: String,
    prev_element_focus: Option<Id>,
    actions: HashSet<Action>,
    recently_activated: HashSet<String>,
}

pub struct Action {
    command: String,
    description: String,
}

impl Action {
    pub const fn new(command: String, description: String) -> Self {
        Action {
            command,
            description,
        }
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.command == other.command
    }
}

impl Eq for Action {}

impl Hash for Action {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.command.hash(state);
    }
}

impl CommandPalette {
    pub fn new(actions: Vec<Action>) -> Self {
        CommandPalette {
            is_open: false,
            search_term: String::new(),
            prev_element_focus: None,
            actions: actions.into_iter().collect(),
            recently_activated: HashSet::new(),
        }
    }

    pub fn activate(&mut self, action: &str) {
        self.recently_activated.insert(action.to_string());
    }
    pub fn is_activated(&self, action: &str) -> bool {
        self.recently_activated.contains(action)
    }
    pub fn reset_activations(&mut self) {
        self.recently_activated.clear();
    }
}

impl CommandPalette {
    /// Opens the search for commands window
    /// You can use `ctx.memory().focus()` to make commandpalette return focus
    /// after it is done
    pub fn open(&mut self, return_focus_to: Option<Id>) {
        if !self.is_open {
            self.prev_element_focus = dbg!(return_focus_to);
        }
        self.is_open = true;
    }
    /// Renders the search for command window
    pub fn render(&mut self, ctx: &egui::Context) {
        let mut activated_this_render = HashSet::new();
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

                egui::Grid::new("palette_action_list")
                    .striped(true)
                    .num_columns(1)
                    .show(ui, |ui| {
                        for action in &self.actions {
                            if !action.command.contains(&self.search_term) {
                                continue;
                            }
                            let action_clicked = ui
                                .vertical(|ui| {
                                    let b = ui.button(&action.command);
                                    ui.label(&action.description);
                                    b.clicked()
                                })
                                .inner;
                            if action_clicked {
                                activated_this_render.insert(action.command.clone());
                                if let Some(w) = self.prev_element_focus {
                                    ctx.memory().request_focus(w);
                                }
                                self.is_open = false;
                            }
                            ui.end_row()
                        }
                    })
            });
        self.recently_activated.extend(activated_this_render);
    }
    // pub fn register(&mut self, command: &str, description: &str) {}
}
