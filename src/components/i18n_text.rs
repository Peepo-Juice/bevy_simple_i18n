use crate::prelude::new::{I18nTranslation, UpdatedTranslation};

use super::new::I18nString;
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
              translation_q: Query<&I18nTranslation>,
              mut text_q: Query<&mut Text>| {
            let e = trigger.0;

            if let Ok(translated) = translation_q.get(e) {
                if let Ok(mut text) = text_q.get_mut(e) {
                    println!("translated.value().to_string() {:?}", translated.value().to_string());
                    text.0 = translated.value().to_string();
                } else {
                    println!("new");
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
