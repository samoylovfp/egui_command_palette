//! disrupts the way you write actions
//! Unfortunately
//! the palette cannot activate a piece of code
//! that is written in a hidden widget
//! since the "ui" function of it is never called

use std::{collections::HashSet, hash::Hash};

use egui::{Id, Key, Modifiers};

pub struct CommandPalette<A> {
    is_open: bool,
    search_term: String,
    prev_element_focus: Option<Id>,
    actions: HashSet<A>,
    recently_activated: HashSet<A>,
}

pub trait Action: PartialEq + Eq + Hash + Clone {
    fn cmd(&self) -> &str;
    fn description(&self) -> &str;
}

impl<A: Action> CommandPalette<A> {
    pub fn new(actions: Vec<A>) -> Self {
        CommandPalette {
            is_open: false,
            search_term: String::new(),
            prev_element_focus: None,
            actions: actions.into_iter().collect(),
            recently_activated: HashSet::new(),
        }
    }

    pub fn activate(&mut self, action: A) {
        self.recently_activated.insert(action);
    }
    pub fn get_activations(&mut self) -> HashSet<A> {
        std::mem::take(&mut self.recently_activated)
    }

    /// Opens the search for commands window
    /// You can use `ctx.memory().focus()` to make commandpalette return focus
    /// after it is done
    pub fn open(&mut self, return_focus_to: Option<Id>) {
        if !self.is_open {
            self.prev_element_focus = return_focus_to;
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
                        for action in self.actions.clone() {
                            if !action.cmd().contains(&self.search_term) {
                                continue;
                            }
                            let enter_pressed =
                                ui.input_mut().consume_key(Modifiers::NONE, Key::Enter);
                            let action_clicked = ui
                                .vertical(|ui| {
                                    let b = ui.button(action.cmd());
                                    ui.label(action.description());
                                    b.clicked()
                                })
                                .inner;
                            if action_clicked || enter_pressed {
                                activated_this_render.insert(action);
                                self.close(ctx);
                                self.search_term.clear();
                            }
                            ui.end_row()
                        }
                    })
            });
        self.recently_activated.extend(activated_this_render);
    }

    fn close(&mut self, ctx: &egui::Context) {
        self.is_open = false;
        if let Some(w) = self.prev_element_focus {
            ctx.memory().request_focus(w);
        }
    }
    // pub fn register(&mut self, command: &str, description: &str) {}
}
