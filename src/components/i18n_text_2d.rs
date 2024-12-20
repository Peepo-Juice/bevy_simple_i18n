use super::new::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(I18nString)]
#[component(on_add = I18nText2d::on_add_hook)]
pub struct I18nText2d;
impl I18nText2d {
    pub fn new(key: impl Into<String>) -> (Self, I18nString) {
        (I18nText2d, I18nString::new(key))
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
        mut text_q: Query<&mut Text2d>,
    ) {
        let e = trigger.entity();

        if let Ok(mut text) = text_q.get_mut(e) {
            text.0 = trigger.0.clone();
        } else {
            commands.entity(e).insert(Text2d::new(trigger.0.clone()));
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
