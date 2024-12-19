use super::new::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(I18nString)]
#[component(on_add = on_add_text)]
pub struct I18nText2d;
fn on_add_text(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    world.commands().entity(entity).observe(
        move |trigger: Trigger<UpdatedTranslation>,
              mut commands: Commands,
              translation_q: Query<(&I18nTranslation, &I18nFont)>,
              mut text_q: Query<(&mut Text2d, &mut TextFont)>| {
            let e = trigger.0;

            if let Ok((translated, font)) = translation_q.get(e) {
                if let Ok((mut text, mut text_font)) = text_q.get_mut(e) {
                    text.0 = translated.value().to_string();
                    text_font.font_size = font.size;
                } else {
                    commands
                        .entity(e)
                        .insert(Text2d::new(translated.value().to_string()));
                }
            }
        },
    );
}
