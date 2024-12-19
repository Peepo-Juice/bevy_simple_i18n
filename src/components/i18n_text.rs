use crate::prelude::new::{I18nTranslation, UpdatedTranslation};

use super::new::{I18nFont, I18nString};
use bevy::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    ui::widget::Text,
};

#[cfg(feature = "numbers")]
use fixed_decimal::FixedDecimal;

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(I18nString)]
#[component(on_add = on_add_text)]
pub struct I18nText;
fn on_add_text(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.commands().entity(entity).observe(
        move |trigger: Trigger<UpdatedTranslation>,
              mut commands: Commands,
              translation_q: Query<(&I18nTranslation, &I18nFont)>,
              mut text_q: Query<(&mut Text, &mut TextFont)>| {
            let e = trigger.0;

            if let Ok((translated, font)) = translation_q.get(e) {
                if let Ok((mut text, mut text_font)) = text_q.get_mut(e) {
                    text.0 = translated.value().to_string();
                    text_font.font_size = font.size;
                } else {
                    commands
                        .entity(e)
                        .insert(Text::new(translated.value().to_string()));
                }
            }
        },
    );
}

#[derive(Reflect, Debug, Clone)]
pub enum InterpolationType {
    String(String),
    #[cfg(feature = "numbers")]
    Number(#[reflect(ignore)] FixedDecimal),
}
