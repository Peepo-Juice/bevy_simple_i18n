use bevy::prelude::*;

use bevy_simple_i18n::prelude::*;
use new::{I18nFont, I18nLocale, I18nString};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the base plugin
        .add_plugins(I18nPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.),
            ..Default::default()
        })
        .with_children(|parent| {
            // Basic usage of the i18n text component
            parent.spawn((
                I18nText::new("hello"),
                // I18nString::new("hello"),
                // I18nFont {
                //     family: "NotoSans".to_string(),
                //     size: 12.,
                // },
            ));

            // Basic usage of the i18n number component
            parent.spawn((
                I18nText,
                I18nString::new("number").with_num_arg("number", 24501.20),
                I18nFont::new("NotoSans"),
            ));

            // Interpolation example
            parent.spawn((
                I18nText,
                I18nString::new("messages.hello").with_arg("name", "Bevy User"),
                // I18nFont {
                //     family: "NotoSans".to_string(),
                //     size: 12.,
                // },
            ));
        });

    // Basic usage of the Text2d implementation
    commands.spawn((
        I18nText2d,
        I18nString::new("text2d"),
        I18nFont::new("NotoSans"),
        Transform::from_xyz(300., 300., 0.),
    ));
}
