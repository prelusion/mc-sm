use bevy::{prelude::*};
use rand::Rng;
mod cam;


const TILE_SIZE: f32 = 40.0;  // Size of each tile
const GRID_WIDTH: usize = 10;  // Number of tiles per row
const GRID_HEIGHT: usize = 10;  // Number of tiles per column

fn main() {
    App::new()
        .insert_resource(cam::CameraVelocity::default())
        .add_plugins(DefaultPlugins)
        .add_event::<BoardModifiedEvent>()
        .add_systems(Startup, (setup))
        .add_systems(Update, (generateBoard, drawBoard, cam::keyboard_input, move_camera, handle_space_key))
        .run();

}

#[derive(Event)]
pub struct BoardModifiedEvent;


fn setup(mut commands: Commands, mut ev_board_modified: EventWriter<BoardModifiedEvent>) {
    info!("Spawning Camera...");
    commands.spawn(Camera2dBundle::default()).insert(Camera);  // Spawn a 2D camera and tag it with the `Camera` component
    info!("Camera spawned and tagged with Camera component.");

    
    generateBoard(commands, ev_board_modified);
}

fn move_camera(
    camera_velocity: Res<cam::CameraVelocity>,  // Access the camera velocity resource
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,  // Query for the camera entity's Transform
) {
    let delta_time = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.translation.x += camera_velocity.velocity.x * 500.0 * delta_time;  // Move based on x velocity
        transform.translation.y += camera_velocity.velocity.y * 500.0 * delta_time;  // Move based on y velocity
    }
}


#[derive(Component)]
struct Camera;  // Tag to identify the camera entity



#[derive(Component)]
struct Board {
    tiles: Vec<Tile>
}


#[derive(Component)]
struct Tile {
    x: f32,
    y: f32,
}


fn generateBoard(mut commands: Commands, mut ev_board_modified: EventWriter<BoardModifiedEvent>) {
    let mut tiles = Vec::new();

    // Generate a 10x10 grid of tiles, each with a size of TILE_SIZE
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = col as f32 * TILE_SIZE;
            let y = row as f32 * TILE_SIZE;

            // Add the tile to the tiles Vec
            tiles.push(Tile { x, y });
        }
    }

    // Create the Board and insert it into the entity
    let board = Board { tiles };
    commands.spawn_empty().insert(board);

    // Trigger event that the board has been modified
    ev_board_modified.send(BoardModifiedEvent);
}


fn handle_space_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut ev_board_modified: EventWriter<BoardModifiedEvent>,
    mut board_query: Query<&mut Board>,
) {
    // Check if the space key is pressed
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Access the board and its current tiles
        if let Ok(mut board) = board_query.get_single_mut() {
            let mut rng = rand::thread_rng();
            let mut new_tile_position: Option<(f32, f32)> = None;

            // Try to find an empty position for the new tile
            loop {
                let x = rng.gen_range(0, GRID_WIDTH) as f32 * TILE_SIZE;
                let y = rng.gen_range(0, GRID_HEIGHT) as f32 * TILE_SIZE;

                // Check if the tile already exists at the generated position
                if !board.tiles.iter().any(|tile| tile.x == x && tile.y == y) {
                    new_tile_position = Some((x, y));
                    break;
                }
            }

            // If a valid new tile position is found, add the tile
            if let Some((x, y)) = new_tile_position {
                board.tiles.push(Tile { x, y });
                ev_board_modified.send(BoardModifiedEvent);  // Trigger the event to update the board
            }
        }
    }
}



fn drawBoard(
    mut commands: Commands, 
    mut ev_board_modified: EventReader<BoardModifiedEvent>,  // EventReader for BoardModifiedEvent
    board_query: Query<&Board>, 
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    // Use `iter` on the EventReader to process each event
    for _event in ev_board_modified.read() {  // Corrected: Use `iter()` to iterate over events
        // Get the board data
        if let Ok(board) = board_query.get_single() {
            // Clear existing tiles (if necessary) - Optionally add logic to remove existing tiles before redrawing.
            
            // Spawn the tiles as individual entities with SpriteBundles
            for tile in &board.tiles {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(tile.x, tile.y, 0.0),  // Place the tile in the correct position
                            ..default()
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),  // Set tile size
                            color: Color::rgb(
                                rng.gen_range(0.0, 1.0), 
                                rng.gen_range(0.0, 1.0), 
                                rng.gen_range(0.0, 1.0),
                            ), 
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        }
    }
}





// fn board(mut commands: Commands, asset_server: Res<AssetServer>) {
//     //camera
//     let mut rng = rand::thread_rng();

//     let mut tiles = Vec::new();

//     // Generate a 10x10 grid of tiles, each with a size of TILE_SIZE
//     for row in 0..GRID_HEIGHT {
//         for col in 0..GRID_WIDTH {
//             // Calculate the tile positions based on the index
//             let x = col as f32 * TILE_SIZE;
//             let y = row as f32 * TILE_SIZE;

//             // Add the tile to the tiles Vec
//             tiles.push(Tile { x, y });
//         }
//     }

//     // Spawn the tiles as individual entities with SpriteBundles
//     for tile in &tiles {
//         commands.spawn((
//             SpriteBundle {
//                 transform: Transform {
//                     translation: Vec3::new(tile.x, tile.y, 0.0),  // Place the tile in the correct position
//                     ..default()
//                 },
//                 sprite: Sprite {
//                     custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),  // Set tile size
//                     color: Color::rgb(
//                         rng.gen_range(0.0, 1.0), 
//                         rng.gen_range(0.0, 1.0), 
//                         rng.gen_range(0.0, 1.0),
//                     ), 
//                     ..default()
//                 },
//                 ..default()
//             },
//         ));
//     }

//     // Create the Board and insert it into the entity (after spawning tiles)
//     let board = Board { tiles };
//     commands.spawn_empty().insert(board);
// }