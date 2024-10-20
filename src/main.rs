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
        .add_event::<AddTileEvent>()
        .add_systems(Startup, (setup))
        .add_systems(Update, (generateBoard, drawBoard, cam::keyboard_input, move_camera, trigger_tile_addition))
        .add_systems(PostUpdate, add_tile_system)
        .run();

}

#[derive(Event)]
pub struct BoardModifiedEvent;

#[derive(Event)]
pub struct AddTileEvent {
    pub x: f32,
    pub y: f32,
}

fn add_tile_system(
    mut commands: Commands, 
    mut add_tile_event: EventReader<AddTileEvent>,  // Listen for tile add events
    mut board_query: Query<&mut Board>,
    mut ev_board_modified: EventWriter<BoardModifiedEvent>,  // To trigger redraw
) {
    if let Ok(mut board) = board_query.get_single_mut() {
        for event in add_tile_event.read() {
            info!("Add Tile System:");
            // Add the new tile to the board
            let new_tile = Tile { x: event.x, y: event.y };
            board.tiles.push(new_tile);
        }
        info!("new board ev");
        // Trigger a redraw event after the board has been modified
        ev_board_modified.send(BoardModifiedEvent);
    }
}

fn trigger_tile_addition(mut ev_add_tile: EventWriter<AddTileEvent>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    // Example: Add a tile when the user presses the spacebar
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("key pressed for adding tile: {:?}", keyboard_input);
        // Generate a random position for the new tile
        let x = rand::thread_rng().gen_range(0.0,(GRID_WIDTH as f32 * TILE_SIZE));
        let y = rand::thread_rng().gen_range(0.0,(GRID_HEIGHT as f32 * TILE_SIZE));

        // Send the event to add a new tile
        ev_add_tile.send(AddTileEvent { x, y });
    }
}



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


fn drawBoard(
    mut commands: Commands, 
    mut ev_board_modified: EventReader<BoardModifiedEvent>,  // EventReader for BoardModifiedEvent
    board_query: Query<&Board>, 
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    for _event in ev_board_modified.read() { 
        // Get the board data
        if let Ok(board) = board_query.get_single() {
            // Clear existing tiles (if necessary) - Optionally add logic to remove existing tiles before redrawing.
            
            for tile in &board.tiles {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(tile.x, tile.y, 0.0),
                            ..default()
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
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
