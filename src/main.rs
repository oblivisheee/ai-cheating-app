mod api;
use eframe::{egui, egui::ComboBox};
use openai_api_rs::v1::api::Client;

const API_KEY: &str = "YOUR_API_NAGA_KEY";

#[derive(Default)]
pub struct Calculator {
    prompt: String,
    ai_response: String,
    generating_response: bool,
    selected_model: String,
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let client = api::client(&API_KEY);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Calculate all you need!");

            ui.text_edit_singleline(&mut self.prompt);

            ComboBox::from_label("Select Model")
                .selected_text(&self.selected_model)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_model, "GPT".to_string(), "GPT");
                    ui.selectable_value(&mut self.selected_model, "MISTRAL".to_string(), "Mistral");
                });
            if ui.button("Ask (3 requests per min)").clicked() {
                if !self.prompt.trim().is_empty() {
                    self.generating_response = true;
                    self.ai_response =
                        make_completion(self.prompt.clone(), client, self.selected_model.clone());
                    self.generating_response = false;
                } else {
                    ui.label("Prompt cannot be empty or contain only whitespace.");
                }
            } else if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                if !self.prompt.trim().is_empty() {
                    self.generating_response = true;
                    self.ai_response =
                        make_completion(self.prompt.clone(), client, self.selected_model.clone());
                    self.generating_response = false;
                } else {
                    ui.label("Prompt cannot be empty or contain only whitespace.");
                }
                if self.generating_response {
                    ui.label("Generating...");
                }
            }
            ui.label(&self.ai_response);
        });
    }
}

fn make_completion(prompt: String, client: Client, model: String) -> String {
    api::get_completion(client, prompt, model)
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::<Calculator>::default()),
    );
}
