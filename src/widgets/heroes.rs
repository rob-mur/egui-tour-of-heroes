use crate::types::Hero;
use egui::Widget;
use std::borrow::Cow;

pub struct HeroesWidget<'a> {
    pub heroes: &'a mut [Hero; 9],
    pub selected_hero: &'a mut Option<usize>,
}

impl<'a> Widget for HeroesWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut response = ui.label("Hero Name:");
        for (idx, hero) in self.heroes.iter().enumerate() {
            response = ui.button(format!("Id: {}, Name: {}", hero.id, hero.name.as_ref()));
            if response.clicked() {
                *self.selected_hero = Some(idx);
            }
        }
        if let Some(hero) = *self.selected_hero {
            let hero = &mut self.heroes[hero];
            ui.heading(format!("{} Details", hero.name.to_uppercase()));
            ui.label(format!("id: {}", hero.id));
            ui.label("name: ");
            response = ui.text_edit_singleline(hero.name.to_mut());
        }
        response
    }
}

pub const HEROES: [Hero; 9] = [
    Hero {
        id: 12,
        name: Cow::Borrowed("Dr. Nice"),
    },
    Hero {
        id: 13,
        name: Cow::Borrowed("Bombasto"),
    },
    Hero {
        id: 14,
        name: Cow::Borrowed("Celeritas"),
    },
    Hero {
        id: 15,
        name: Cow::Borrowed("Magneta"),
    },
    Hero {
        id: 16,
        name: Cow::Borrowed("RubberMan"),
    },
    Hero {
        id: 17,
        name: Cow::Borrowed("Dynama"),
    },
    Hero {
        id: 18,
        name: Cow::Borrowed("Dr. IQ"),
    },
    Hero {
        id: 19,
        name: Cow::Borrowed("Magma"),
    },
    Hero {
        id: 20,
        name: Cow::Borrowed("Tornado"),
    },
];
