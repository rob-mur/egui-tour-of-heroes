use crate::types::Hero;
use egui::Widget;

pub struct HeroesWidget<'a> {
    pub hero: &'a mut Hero,
}

impl<'a> Widget for HeroesWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("Hero Name:");
        ui.text_edit_singleline(&mut self.hero.name)
    }
}
