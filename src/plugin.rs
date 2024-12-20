use crate::prelude::utils::get_formatter;
use crate::{
    // components::{I18nFont, I18nNumber, I18nText},
    // prelude::{new::I18nLocale, I18nComponent, I18nText2d},
    prelude::{new::*, utils::translate_by_key},
    resources::{FontFolder, FontManager, FontsLoading, I18n},
    FONT_FAMILIES,
};
use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        schedule::{common_conditions::resource_exists, IntoSystemConfigs},
        system::{Commands, Query, Res, ResMut},
    },
    prelude::*,
    text::Font,
};
use std::path::Path;

/// Initializes the `bevy_simple_i18n` plugin
///
/// # Example
/// ```
/// use bevy::prelude::*;
/// use bevy_simple_i18n::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(I18nPlugin)
///         .run();
/// }
/// ```
pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<FontManager>()
            .init_resource::<I18n>()
            .init_resource::<FontsLoading>()
            .add_systems(PreStartup, load_dynamic_fonts)
            .add_systems(
                Update,
                monitor_font_loading.run_if(resource_exists::<FontsLoading>),
            )
            .add_systems(PreUpdate, update_translations)
            .add_systems(PreUpdate, update_font_size)
            .add_systems(
                PreUpdate,
                notify_font_change
                    .run_if(resource_removed::<FontsLoading>.or(resource_changed::<I18n>)),
            );
    }
}

// this should be handled by observers, but they are not powerful enough yet
fn update_translations(
    mut commands: Commands,
    // font_manager: bevy::ecs::system::Res<FontManager>,
    i18n: Res<I18n>,
    changed_query: Query<(), Or<(Changed<I18nString>, Changed<I18nLocale>)>>,
    mut text_query: Query<(
        Entity,
        &I18nString,
        &mut I18nTranslation,
        Option<&I18nLocale>,
    )>,
) {
    bevy::log::debug!("Updating translations");

    // currently this always iterates over every entity
    // it would be nice if it didnt do that if a single entity was changed
    if i18n.is_changed() || !changed_query.is_empty() {
        for (entity, string, mut translation, locale) in &mut text_query {
            let new_text = translate_by_key(&locale.locale(), &string.key, &string.args);
            // only update if the text has changed
            if translation.translation().cmp(&new_text).is_ne() {
                translation.set(&new_text);
                commands.trigger_targets(UpdatedTranslation(new_text), entity);
            }
        }
    }
}

fn update_font_size(
    mut commands: Commands,
    changed_query: Query<(Entity, &I18nFontSize), Changed<I18nFontSize>>,
) {
    for (e, font) in &changed_query {
        commands.trigger_targets(UpdatedFontSize(font.0), e);
    }
}

/// This will update Text and Text2D when Font changes
fn notify_font_change(
    mut commands: Commands,
    font_manager: Res<FontManager>,
    mut query: Query<(Entity, &I18nFont, Option<&I18nLocale>)>,
) {
    for (e, font, locale) in &mut query {
        let font = font_manager.get(&font.family(), locale.locale());
        commands.trigger_targets(UpdatedFont(font), e);
    }
}

/// Loads the dynamic fonts specified in the [FONT_FAMILIES] constant that's generated by the build script
///
/// TODO: Make the loading state more controllable
fn load_dynamic_fonts(
    mut font_manager: ResMut<FontManager>,
    asset_server: Res<bevy::asset::AssetServer>,
) {
    for dyn_font in FONT_FAMILIES.iter() {
        bevy::log::debug!("Loading dynamic font family: {}", dyn_font.family);
        let mut font_folder = FontFolder::default();
        font_folder.fallback = asset_server.load(Path::new(dyn_font.path).join("fallback.ttf"));
        for font in dyn_font.locales.iter() {
            bevy::log::debug!("Loading font: {}", font);
            let locale = font.split('.').next().expect("Locale is required");
            let path = Path::new(dyn_font.path).join(font);
            let handler: Handle<Font> = asset_server.load(path);
            font_folder.fonts.insert(locale.to_string(), handler);
        }
        font_manager.insert(dyn_font.family.to_string(), font_folder);
    }
}

/// Monitors the font loading state and removes the [FontsLoading] resource when all fonts are loaded
///
/// TODO: Make the loading state more controllable
fn monitor_font_loading(
    mut commands: Commands,
    font_manager: Res<FontManager>,
    asset_server: Res<AssetServer>,
) {
    for folder in font_manager.fonts.values() {
        for font in folder.fonts.values() {
            if !asset_server.is_loaded(font.id()) {
                return;
            }
        }
    }
    commands.remove_resource::<FontsLoading>();
    bevy::log::debug!("All fonts loaded");
}
