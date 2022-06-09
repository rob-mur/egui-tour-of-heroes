use crate::types::Hero;
use crate::widgets::heroes::HEROES;
use crate::widgets::HeroesWidget;

#[derive(Default)]
pub struct App {
    heroes: [Hero; 9],
    selected_hero: Option<usize>,
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            heroes: HEROES,
            selected_hero: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tour of Heroes");
            ui.add(HeroesWidget {
                heroes: &mut self.heroes,
                selected_hero: &mut self.selected_hero,
            });
        });
    }
}
