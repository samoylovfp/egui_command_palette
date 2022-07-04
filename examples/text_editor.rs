struct Editor {
    text: String,
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("panel").show(ctx, |ui|{
            if ui.button("uppercase").clicked() {
                self.text = self.text.chars().flat_map(|c|c.to_uppercase()).collect();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui|
			ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.text))
		);
    }
}

fn main() {
    eframe::run_native(
        "test",
        Default::default(),
        Box::new(|ui| {
            Box::new(Editor {
                text: String::from("hello"),
            })
        }),
    );
}
