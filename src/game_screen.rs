use std::f32::consts::PI;

use bevy::prelude::*;

use crate::components::*;
use crate::game_settings;
use crate::chess_engine;
use bevy_interact_2d::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(game_settings::LogicalGameState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(game_settings::LogicalGameState::Game)
                .with_system(interaction_system)
                .with_system(draw_board) // this must come before update_board (otherwise causes fun race condition)
                .with_system(update_board)
                .with_system(update_fen_text)
                .with_system(update_holding_text)
                .with_system(update_turn_text)
            );
    }
}


fn update_board(
    game_object: Query<&GameState, With<GlobalThing>>,
    mut cells_structs: Query<&mut Cell, With<Cell>>) {
    let game_state = game_object.single();
    //if game_state.last_state == game_state.board.to_fen()  { return; }  

    for mut cell in cells_structs.iter_mut() {
        cell.occupier = *game_state.board.entity_at(cell.position); 
    }
}

// I think the board updating issue is fixed

fn draw_board(
    mut commands: Commands,
    mut cells_structs: Query<&mut Cell , With<Cell>>,
    pieces: Query<Entity, With<Piece>>,
    mut game_object: Query<&mut GameState, With<GlobalThing>>,
    game_assets: Res<AssetHandler>
) { 

    let mut game_state = game_object.single_mut();
    let x_pos = (CELLSIZE * -4) + (CELLSIZE/2) - (CELLSIZE * 2);
    let y_pos = (CELLSIZE * 4) - (CELLSIZE/2) + CELLSIZE;
    
    
    for entity in pieces.iter() {
        commands.entity(entity).despawn();
    }

    for cell in cells_structs.iter_mut() {
        if cell.occupier.is_none() { continue; }
        let occupier = cell.occupier.unwrap();
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: CELLSIZE as f32,
                    y: CELLSIZE as f32,
                }),
                ..default()
            },
            texture: get_piece_asset(&game_assets, &occupier.entity_type, occupier.team_id),
            transform: Transform {
                translation: Vec3 {
                    x: (x_pos + (CELLSIZE * cell.position.x)) as f32,
                    y: (y_pos - (CELLSIZE * cell.position.y)) as f32,
                    ..default()
                },
                ..default()
            },
            ..default()
        }).insert(Piece {});
    }

    game_state.last_state = game_state.board.to_fen();
    
}

fn network_system (
    commands: Commands, 
    mut global_structs: Query<&mut ClientNative, With<GlobalThing>>,
) {
    /* TODO: Network stuff :> */

}

// this only exists because bevy decides to out of order black (from Queen to left rook)
fn ecs_fix(index: u32) -> u32 {
    match index {
        6 => 2,
        4 => 3,
        5 => 4,
        2 => 5,
        3 => 6,
        _ => index,
    }
}

fn interaction_system(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
    cells_structs: Query<&mut Cell, With<Cell>>,
    mut global_structs: Query<&mut GameState, With<GlobalThing>> 
) {

    let mut game_state = global_structs.single_mut();

    // FIXME (LATER) check teams again
    if !mouse_button_input.just_released(MouseButton::Left) { //} || game_state.player_team != game_state.board.current_turn  {
        return;
    }
    let mut index = 0;

    // no new fen yet
    for (ent, coords) in interaction_state.get_group(Group(0)).iter() {
        index = ecs_fix(ent.index()) - 2;
    }

    let cells: Vec<&Cell>  = cells_structs.iter().collect();

    if game_state.selected.is_some() {
        // DONT reselect just get the new pos and spit out a new FEN
        // Unless we need to select a new piece thats fine too
        if cells[index as usize].occupier.is_none() || cells[index as usize].occupier.unwrap().team_id != game_state.board.current_turn {
            // allow the move (capture)
            
            // error check (make sure not none)
            let cell_original = game_state.original_cell_index;
            chess_engine::move_entity(&mut game_state.board, cells[cell_original as usize].position, cells[index as usize].position);
            // println!("{}", game_state.board.to_string());
            // println!("{}", game_state.board.to_fen());
            game_state.selected = None;
            
            return;
        } else {
            // self selection
            return;
        }
    }
    // select pieces
    if cells[index as usize].occupier.is_some() && cells[index as usize].occupier.unwrap().team_id == game_state.player_team {
        game_state.selected = cells[index as usize].occupier;
        game_state.original_cell_index = index;
    }
    
}

// check if there is a way to join all these
fn update_turn_text(
    global_thing: Query<&GameState, With<GlobalThing>>,
    mut current_turn_query: Query<&mut Text, With<CurrentTurnText>>
    
) {
    let game_state = global_thing.single();

    let mut current_turn_text = current_turn_query.single_mut();
    current_turn_text.sections[1].value = format!("{:?}", game_state.board.current_turn);
    current_turn_text.sections[1].style.color = if game_state.board.current_turn == chess_engine::TeamLoyalty::WHITE {
        WHITE_TEXT
    } else {
        BLACK_TEXT
    };
}

fn update_fen_text(
    global_thing: Query<&GameState, With<GlobalThing>>,
    mut current_fen_query: Query<&mut Text, With<CurrentFenText>>
) {
    let game_state = global_thing.single();
    let mut current_fen_text = current_fen_query.single_mut();
    current_fen_text.sections[1].value = format!("{:?}", game_state.board.to_fen());
}

fn update_holding_text (
    global_thing: Query<&GameState, With<GlobalThing>>,
    mut current_selected_query: Query<&mut Text, With<CurrentPieceText>>
){
    let game_state = global_thing.single();
    let mut current_selected_text = current_selected_query.single_mut();
    current_selected_text.sections[1].value = if game_state.selected.is_some() {
        format!("{:?}", game_state.selected.unwrap().entity_type)
    } else {
        "None".to_owned()
    };
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,) {
            
    let game_state = GameState {
        board: chess_engine::Board::create_board(chess_engine::DEFAULTFEN.into()),
        selected: None,
        last_state: String::new(),
        player_team: chess_engine::TeamLoyalty::BLACK, // eventually have some matchmaking system decide
        original_cell_index: 0,
    };

    // let color1 = Color::hex("363333").unwrap();
    // let color2 = Color::hex("6d6e53").unwrap();
    let color1 = Color::hex("CBC1AD").unwrap();
    let color2 = Color::hex("242721").unwrap();

    let x_pos = (CELLSIZE * -4) + (CELLSIZE/2) - (CELLSIZE * 2);
    let y_pos = (CELLSIZE * 4) - (CELLSIZE/2) + CELLSIZE;

    let mut color_index = 0;

    /* Create the board */
    for row in 0..8 {
        color_index = row;
        for col in 0..8 {
            let clr = if color_index % 2 == 0 { color1 } else { color2 };
            let id = commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: clr,
                    custom_size: Some(Vec2::new(CELLSIZE as f32, CELLSIZE as f32)),
                    ..default()
                },
                transform : Transform {
                    translation : Vec3 {
                        x: (x_pos + (CELLSIZE) * col) as f32,
                        y: (y_pos - ((CELLSIZE) * row)) as f32,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            }).insert(Cell {
                position: chess_engine::Vec2 { x: col, y: row },
                occupier: game_state.board.entity_at(chess_engine::Vec2 { x: col, y: row }).clone()
            }).insert(Interactable {
                groups: vec![Group(0)],
                bounding_box: (Vec2::new(-(CELLSIZE/2) as f32, -(CELLSIZE/2) as f32), Vec2::new((CELLSIZE/2) as f32, (CELLSIZE/2) as f32)),
                ..default()
            }).id();
            color_index += 1;
            let obj = game_state.board.entity_at(chess_engine::Vec2 { x: col, y: row }).clone();

            if obj.is_some() {
                println!("{} => {:?}", id.index(), obj.unwrap());
            }
        }
    }

    // This must be at the end (don't ask me why)
    commands.spawn_empty()
        .insert(GlobalThing {})
        .insert(game_state);
        // NETWORK HERE
        //.insert(ClientNative {
        //    tcp_client: network_handler::Client::create_client("localhost:8000".to_owned(), 128).unwrap()
        //});

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new( 
                "Current Turn: ",
                TextStyle {
                    font: asset_server.load(FONT_FILE),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(FONT_FILE),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ]).with_text_alignment(TextAlignment::BOTTOM_LEFT)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(100.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        CurrentTurnText
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new( 
                "Current Selected: ",
                TextStyle {
                    font: asset_server.load(FONT_FILE),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(FONT_FILE),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]).with_text_alignment(TextAlignment::BOTTOM_LEFT)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(60.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        CurrentPieceText
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new( 
                "Current Fen: ",
                TextStyle {
                    font: asset_server.load(FONT_FILE),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(FONT_FILE),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]).with_text_alignment(TextAlignment::BOTTOM_LEFT)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(20.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        CurrentFenText
    ));

    const HALF_SIZE: f32 = 1.0;

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("2596.glb#Scene0"),
        transform: Transform{
            translation: Vec3 {
                z: -2.5,
                ..default()
            },
            rotation: Quat::from_rotation_y(PI / 2.),
            ..default()
        },
        ..default()
    });
}