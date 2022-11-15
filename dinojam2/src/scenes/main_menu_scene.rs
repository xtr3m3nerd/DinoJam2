use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy::app::AppExit;
use crate::states::AppState;
use crate::util;
use iyes_loopless::prelude::*;

pub struct MainMenuScenePlugin;

impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        // menu setup (state enter) systems
        app.add_enter_system(AppState::MainMenu, setup_menu);
        // menu cleanup (state exit) systems
        app.add_exit_system(AppState::MainMenu, util::despawn_with::<MainMenu>);

        // menu stuff
        app.add_system_set(
            ConditionSet::new()
            .run_in_state(AppState::MainMenu)
            .with_system(close_on_esc)
            .with_system(butt_interact_visual)
            // our menu button handlers
            .with_system(butt_exit.run_if(on_butt_interact::<ExitButt>))
            .with_system(butt_game.run_if(on_butt_interact::<EnterButt>))
            .into()
        );
    }
}

/// Marker for the main menu entity
#[derive(Component)]
struct MainMenu;

/// Marker for the "Exit App" button
#[derive(Component)]
struct ExitButt;

/// Marker for the "Enter Game" button
#[derive(Component)]
struct EnterButt;

/// Change button color on interaction
fn butt_interact_visual(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = UiColor(Color::rgb(0.75, 0.75, 0.75));
            }
            Interaction::Hovered => {
                *color = UiColor(Color::rgb(0.8, 0.8, 0.8));
            }
            Interaction::None => {
                *color = UiColor(Color::rgb(1.0, 1.0, 1.0));
            }
        }
    }
}

/// Condition to help with handling multiple buttons
///
/// Returns true when a button identified by a given component is clicked.
fn on_butt_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

/// Handler for the Exit Game button
fn butt_exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

/// Handler for the Enter Game button
fn butt_game(mut commands: Commands) {
    // queue state transition
    commands.insert_resource(NextState(AppState::InGame));
}

/// Construct the main menu UI
fn setup_menu(mut commands: Commands, ass: Res<AssetServer>) {
    let butt_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..Default::default()
    };
    let butt_textstyle = TextStyle {
        font: ass.load("fonts/Roboto-Medium.ttf"),
        font_size: 24.0,
        color: Color::BLACK,
    };

    let menu = commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::rgb(0.5, 0.5, 0.5)),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                //align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
    .insert(MainMenu)
        .id();

    let butt_enter = commands
        .spawn_bundle(ButtonBundle {
            style: butt_style.clone(),
            ..Default::default()
        })
    .with_children(|btn| {
        btn.spawn_bundle(TextBundle {
            text: Text::from_section("Enter Game", butt_textstyle.clone()),
            ..Default::default()
        });
    })
    .insert(EnterButt)
        .id();

    let butt_exit = commands
        .spawn_bundle(ButtonBundle {
            style: butt_style.clone(),
            ..Default::default()
        })
    .with_children(|btn| {
        btn.spawn_bundle(TextBundle {
            text: Text::from_section("Exit Game", butt_textstyle.clone()),
            ..Default::default()
        });
    })
    .insert(ExitButt)
        .id();

    commands
        .entity(menu)
        .push_children(&[butt_enter, butt_exit]);
}
