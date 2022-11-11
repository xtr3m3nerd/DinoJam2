use bevy::{
    diagnostic::{
        Diagnostics,
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin,
    },
    prelude::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_system_to_stage(CoreStage::Last, ui_apply_fixed_z)
            .add_startup_system(setup)
            .add_system(text_update_system);
    }
}

#[derive(Component)]
pub struct Debug;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(15.0),
                    top: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
    )
        .insert(FpsText)
        .insert(Debug)
        .insert(UiFixedZ { z: 101.0 });
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{average:.2}");
            }
        }
    }
}

#[derive(Debug, Component)]
struct UiFixedZ {
    pub z: f32,
}

fn ui_apply_fixed_z(
    mut node_query: Query<(&mut Transform, &mut GlobalTransform, &UiFixedZ), With<Node>>,
) {
    for (mut transform, mut global_transform, fixed) in node_query.iter_mut() {
        transform.translation.z = fixed.z;
        global_transform.translation_mut().z = fixed.z;
    }
}
