use bevy::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    ui::widget::Text,
};

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(Text, I18nString)]
#[component(on_add = I18nText::on_add_hook)]
pub struct I18nText;

impl I18nText {
    pub fn new(key: &str) -> (Self, I18nString) {
        (I18nText, I18nString::new(key))
    }

    fn on_add_hook(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        world
            .commands()
            .entity(entity)
            .observe(Self::update_translation);

        world
            .commands()
            .entity(entity)
            .observe(Self::updated_font_size);
        world.commands().entity(entity).observe(Self::update_font);
    }

    fn update_translation(
        trigger: Trigger<UpdatedTranslation>,
        mut commands: Commands,
        mut text_q: Query<&mut Text>,
    ) {
        let e = trigger.entity();

        if let Ok(mut text) = text_q.get_mut(e) {
            text.0 = trigger.0.clone();
        } else {
            commands.entity(e).insert(Text::new(trigger.0.clone()));
        }
    }

    fn updated_font_size(trigger: Trigger<UpdatedFontSize>, mut text_q: Query<&mut TextFont>) {
        let e = trigger.entity();

        if let Ok(mut bevy_font) = text_q.get_mut(e) {
            bevy_font.font_size = trigger.event().0;
        }
    }

    fn update_font(trigger: Trigger<UpdatedFont>, mut font_q: Query<&mut TextFont>) {
        let e = trigger.entity();

        if let Ok(mut bevy_font) = font_q.get_mut(e) {
            bevy_font.font = trigger.event().0.clone();
        }
    }
}

#[cfg(feature = "numbers")]
use fixed_decimal::FixedDecimal;

use super::i18n_font::UpdatedFont;
use super::i18n_font_size::UpdatedFontSize;
use super::i18n_string::I18nString;
use super::i18n_translation::UpdatedTranslation;

#[derive(Reflect, Debug, Clone)]
pub enum InterpolationType {
    String(String),
    #[cfg(feature = "numbers")]
    Number(#[reflect(ignore)] FixedDecimal),
}
