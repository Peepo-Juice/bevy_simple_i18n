use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

#[derive(Event, Reflect, Debug, Clone)]
pub struct UpdatedFontSize(pub f32);

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
#[component(on_add = on_add_font_size)]
pub struct I18nFontSize(pub f32);
impl I18nFontSize {
    pub fn new(size: f32) -> Self {
        Self(size.into())
    }
}
fn on_add_font_size(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let font = world
        .get::<I18nFontSize>(entity)
        .expect("I18nString requires this component");

    world.trigger_targets(UpdatedFontSize(font.0), entity);
}
