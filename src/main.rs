
#[allow(unused)]
#[allow(unused_variables)]

/* Bevy & Third party includes */
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use bevy_interact_2d::*;
use bevy::prelude::*;

/* Local Includes */
mod network_handler;
mod game_settings;
mod chess_engine;
mod game_screen;
mod components;
mod menu;

use components::*;


fn main() { 
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bitmatoes Chess".into(),
                width: ((CELLSIZE * 8) + 128 + CELLSIZE * 3) as f32,
                height: ((CELLSIZE * 8) + 128 + CELLSIZE)  as f32,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(InteractionPlugin)
        .add_plugin(drag::DragPlugin)
        
        .insert_resource(game_settings::DisplayQuality::Medium)
        .add_state(game_settings::LogicalGameState::Menu)
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(game_screen::GameplayPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    let black = "b".to_owned();
    let white = "w".to_owned();

    commands.insert_resource(AssetHandler { 
        pawn: vec![assets.load(white.to_owned() + &PAWN_FILENAME), assets.load(black.to_owned() + &PAWN_FILENAME)],
        rook: vec![assets.load(white.to_owned() + &ROOK_FILENAME), assets.load(black.to_owned() + &ROOK_FILENAME)],
        bishop: vec![assets.load(white.to_owned() + &BISHOP_FILENAME), assets.load(black.to_owned() + &BISHOP_FILENAME)],
        knight: vec![assets.load(white.to_owned() + &KNIGHT_FILENAME), assets.load(black.to_owned() + &KNIGHT_FILENAME)],
        queen: vec![assets.load(white.to_owned() + &QUEEN_FILENAME), assets.load(black.to_owned() + &QUEEN_FILENAME)],
        king: vec![assets.load(white.to_owned() + &KING_FILENAME), assets.load(black.to_owned() + &KING_FILENAME)],
        global_font: assets.load(FONT_FILE) 
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::None,
            ..default()
        },
        camera: Camera {
            priority: 1,
            ..default()
        },
        ..default()
    })
            .insert( InteractionSource {
                groups: vec![Group(0), Group(1)],
                ..default()
            });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        camera_3d: Camera3d {
            //clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::None,
            ..default()
        },
        ..default()
    });

}