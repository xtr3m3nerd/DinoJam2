use bevy::prelude::*;
use iyes_loopless::prelude::*;

//use std::time::Duration;
use rand::prelude::*;

use crate::states::AppState;
use crate::util;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        // app.add_fixed_timestep(
        //     Duration::from_millis(125),
        //     // give it a label
        //     "my_fixed_update",
        // );

        // game cleanup (state exit) systems
        app.add_exit_system(AppState::InGame, util::despawn_with::<MySprite>);
        // in-game stuff
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(back_to_menu_on_esc)
                .with_system(clear_on_del)
                .with_system(spin_sprites.run_if_not(spacebar_pressed))
                .into(),
        );
        app.add_system(
            spawn_sprite
                // only in-game!
                .run_in_state(AppState::InGame)
                // only while the spacebar is pressed
                .run_if(spacebar_pressed),
        );
        // app.add_fixed_timestep_system(
        //     "my_fixed_update", 0,
        //     spawn_sprite
        //     // only in-game!
        //     .run_in_state(AppState::InGame)
        //     // only while the spacebar is pressed
        //     .run_if(spacebar_pressed)
        // );
    }
}

/// Marker for our in-game sprites
#[derive(Component)]
struct MySprite;

/// Reset the in-game state when pressing delete
fn clear_on_del(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Delete) || kbd.just_pressed(KeyCode::Back) {
        commands.insert_resource(NextState(AppState::InGame));
    }
}

/// Transition back to menu on pressing Escape
fn back_to_menu_on_esc(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(AppState::MainMenu));
    }
}

/// Condition system for holding the space bar
fn spacebar_pressed(kbd: Res<Input<KeyCode>>) -> bool {
    kbd.pressed(KeyCode::Space)
}

/// Spawn a MySprite entity
fn spawn_sprite(mut commands: Commands) {
    let mut rng = thread_rng();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(rng.gen(), rng.gen(), rng.gen(), 0.5),
                custom_size: Some(Vec2::new(64., 64.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                rng.gen_range(-420.0..420.0),
                rng.gen_range(-420.0..420.0),
                rng.gen_range(0.0..100.0),
            ),
            ..Default::default()
        })
        .insert(MySprite);
}

/// Rotate all the sprites
fn spin_sprites(mut q: Query<&mut Transform, With<MySprite>>, t: Res<Time>) {
    for mut transform in q.iter_mut() {
        transform.rotate(Quat::from_rotation_z(1.0 * t.delta_seconds()));
    }
}
