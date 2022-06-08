use egui::Widget;

pub struct HeroesWidget{
    pub hero: String
}

impl Widget for HeroesWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label(self.hero)
    }
}
