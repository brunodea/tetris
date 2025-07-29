use bevy::prelude::*;

#[derive(PartialEq)]
pub enum State {
    Paused,
    Running,
}

#[derive(Resource)]
pub struct GameState(pub State);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState(State::Running));
    }
}
