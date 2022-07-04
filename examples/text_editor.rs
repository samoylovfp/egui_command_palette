use egui::{Key, Modifiers};
use egui_command_palette::CommandPalette;

struct Editor {
    text: String,
    command_palette: CommandPalette,
}

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
                self.text = self.text.chars().flat_map(|c| c.to_uppercase()).collect();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.text),
            )
        });
    }
}

fn main() {
    eframe::run_native(
        "test",
        Default::default(),
        Box::new(|ui| {
            Box::new(Editor {
                text: String::from("hello"),
                command_palette: CommandPalette::new(),
            })
        }),
    );
}
