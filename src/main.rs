mod api;
use eframe::egui;

const API_KEY: &str = "YOUR_API_NAGA_KEY";

#[derive(Default)]
pub struct Calculator {
    prompt: String,
    ai_response: String,
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let client = api::client(&API_KEY);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Calculate all you need!");

            ui.text_edit_singleline(&mut self.prompt);
            if ui.button("Ask (3 requests per min)").clicked() {
                ui.label("Generating...");
                self.ai_response = api::get_completion(client, &self.prompt);
                ui.label(&self.ai_response);
            }
            ui.label(&self.ai_response);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::<Calculator>::default()),
    );
}
