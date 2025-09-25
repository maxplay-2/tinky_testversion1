use eframe::{egui, App};

#[derive(Default)]
struct AppState {
    ask_nickname: bool, // показывать ли поле для ввода
    nickname: String,   // введённый ник
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.ask_nickname {
                // Пока никнейм не спрашиваем — только кнопка
                if ui.button("Играть!").clicked() {
                    self.ask_nickname = true;
                }
            } else {
                ui.label("Введите ваш никнейм:");
                ui.text_edit_singleline(&mut self.nickname);

                if ui.button("Продолжить").clicked() {
                    println!("ник игрока : '{}' ", self.nickname);
                    // Можно тут, например, сбросить ask_nickname или перейти к игре
                    self.ask_nickname = false;
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "tinkytestversion1",
        options,
        Box::new(|_cc| Box::new(AppState::default())),
    )
}
