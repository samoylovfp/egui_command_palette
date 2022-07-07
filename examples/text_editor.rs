use egui::{Key, Modifiers};
use egui_command_palette::{Action, CommandPalette};

struct Editor {
    text: String,
    command_palette: CommandPalette,
}

const UPPERCASE_COMMAND: &str = "uppercase";
const LOWERCASE_COMMAND: &str = "lowercase";

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
            if ui.button("uppercase").clicked() {
                self.command_palette.activate(UPPERCASE_COMMAND)
            }
            if ui.button("lowercase").clicked() {
                self.command_palette.activate(LOWERCASE_COMMAND)
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.text),
            );
            ui.collapsing("test", |ui| println!("Inside collapsing is called"));
        });

        // FIXME: Switch to match
        if self.command_palette.is_activated(UPPERCASE_COMMAND) {
            self.text = self.text.chars().flat_map(|c| c.to_uppercase()).collect()
        }
        if self.command_palette.is_activated(LOWERCASE_COMMAND) {
            self.text = self.text.chars().flat_map(|c| c.to_lowercase()).collect()
        }
        self.command_palette.reset_activations();
    }
}

fn main() {
    eframe::run_native(
        "test",
        Default::default(),
        Box::new(|ui| {
            Box::new(Editor {
                text: String::from("hello"),
                command_palette: CommandPalette::new(vec![
                    Action::new(
                        String::from(UPPERCASE_COMMAND),
                        String::from("Make all the text uppercase"),
                    ),
                    Action::new(
                        String::from(LOWERCASE_COMMAND),
                        String::from("Make all the text lowercase"),
                    ),
                ]),
            })
        }),
    );
}
