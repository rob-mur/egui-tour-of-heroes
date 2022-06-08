use crate::types::Hero;
use crate::widgets::HeroesWidget;

#[derive(Default)]
pub struct App {
    hero: Hero,
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tour of Heroes");
            ui.add(HeroesWidget {
                hero: &mut self.hero,
            });
        });
    }
}
