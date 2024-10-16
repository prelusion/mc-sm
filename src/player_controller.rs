
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, (noclip, player_look, player_laser, set_selected).in_set(Playing))
        .add_systems(Update, player_move.in_set(Playing).run_if(in_state(PlayerMode::Normal)))
        .add_systems(Update, noclip_move.in_set(Playing).run_if(in_state(PlayerMode::NoClip)))
        .add_systems(OnExit(GameState::GenWorld), set_play_ground)
        .init_resource::<InputState>()
        .init_resource::<SelectedBlock>()
        .init_state::<PlayerMode>();
    }
}
