use bevy::{prelude::*, math::vec3};
use std::f32::consts::PI;


const PLAYER_START_Y: f32 = 0.0;
const PLAYER_SIZE: Vec3 = Vec3::new(2.0, 2.0, 2.0);
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    //camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let values: [f32; 3] = [2.2, 4.4, 6.6];

    for z in values {
        for x in values {
            //paddle
            commands.spawn((
                PbrBundle {
                    transform: Transform {
                        translation: vec3(4. - x, PLAYER_START_Y, 4. - z - z),
                        rotation: Quat::from_rotation_x(-PI / 8.),
                        scale: PLAYER_SIZE,
                        ..default()
                    },
                    mesh: meshes.add(Cuboid::default()),
                    material: materials.add(StandardMaterial {
                        base_color:PLAYER_COLOR,
                        ..Default::default()
                    }),
                
                    ..default()
                },
                Player,
            ));
        }
    }
}
