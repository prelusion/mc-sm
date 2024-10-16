use bevy::{prelude::*, math::vec3};
use rand::Rng;

const PLAYER_START_Y: f32 = 0.0;
const PLAYER_SIZE: Vec2 = Vec2::new(40.0, 40.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();

}



#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //camera
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2dBundle::default());

    let values: [f32; 5] = [40., 80., 120., 160., 200.];
    for z in values {
        for x in values {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: vec3(0. - x, PLAYER_START_Y - z, 0. - x),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(rng.gen_range(0.0..40.0), rng.gen_range(0.0..255.0), rng.gen_range(0.0..255.0)),
                        custom_size: Some(PLAYER_SIZE),
                        ..default()
                    },
                    ..default()
                },
                Player,
            ));
        }
    }
}
