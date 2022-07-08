use egui::{Key, Modifiers};
use egui_command_palette::CommandPalette;

#[derive(Hash, Eq, PartialEq, Clone)]
enum Action {
    LOWERCASE,
    UPPERCASE,
}

impl egui_command_palette::Action for Action {
    fn cmd(&self) -> &str {
        match self {
            Action::LOWERCASE => "lowercase",
            Action::UPPERCASE => "uppercase",
        }
    }

    fn description(&self) -> &str {
        match self {
            Action::LOWERCASE => "Make all the text lowercase",
            Action::UPPERCASE => "Same as lowercase, but uppercase",
        }
    }
}

struct Editor {
    text: String,
    command_palette: CommandPalette<Action>,
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input_mut().consume_key(
            Modifiers {
                ctrl: true,
                shift: true,
                ..Modifiers::NONE
            },
            Key::P,
        ) {
            self.command_palette.open(ctx.memory().focus());
        }
        self.command_palette.render(ctx);
        egui::TopBottomPanel::top("panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("uppercase").clicked() {
                    self.command_palette.activate(Action::UPPERCASE)
                }
                if ui.button("lowercase").clicked() {
                    self.command_palette.activate(Action::LOWERCASE)
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.text).font(egui::TextStyle::Monospace),
            );
        });

        for action in self.command_palette.get_activations() {
            match action {
                Action::LOWERCASE => self.text = self.text.to_lowercase(),
                Action::UPPERCASE => self.text = self.text.to_uppercase(),
            }
        }
    }
}

fn main() {
    eframe::run_native(
        "test",
        Default::default(),
        Box::new(|_ui| {
            Box::new(Editor {
                text: String::from("Press Ctrl+Shift+P to bring up the command palette"),
                command_palette: CommandPalette::new(vec![Action::LOWERCASE, Action::UPPERCASE]),
            })
        }),
    );
}
