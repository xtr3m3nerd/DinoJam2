use crate::states::AppState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

/// We can just access the `CurrentState`, and even use change detection!
pub fn debug_current_state(state: Res<CurrentState<AppState>>) {
    if state.is_changed() {
        println!("Detected state change to {:?}!", state);
    }
}

/// Despawn all entities with a given component type
pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
