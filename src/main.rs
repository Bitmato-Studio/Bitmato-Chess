
#[allow(unused)]
#[allow(unused_variables)]

/* Bevy & Third party includes */
// use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::{prelude::*, winit::WinitWindows, window::WindowId};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_interact_2d::*;
use winit::window::Icon;
use image;

/* Local Includes */
mod network_handler;
mod config_handler;
mod splash_screen;
mod game_settings;
mod chess_engine;
mod game_screen;
mod components;
mod menu;

use components::*;

fn set_window_icon (
    windows: NonSend<WinitWindows>
){
    let primary = windows.get_window(WindowId::primary()).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let img = image::open("assets/branding/icon_win_bitmato_chess.png")
                    .expect("Failed to open icon path")
                    .into_rgba8();
        let (width, height) = img.dimensions();
        let rgba = img.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}

#[derive(Debug)]
struct Login {
    pub username: String,
    pub password: String,
}

fn run_login() -> Login{
    //extern crate rpassword;
    //use rpassword::read_password; 

    let mut login = Login{ username: String::new(), password: String::new() };

    print!("Enter DISCORD Username: ");
    std::io::stdin().read_line(&mut login.username).unwrap();

    // TODO: Make this better lol
    print!("Enter a Password (NOT FOR DISCORD): ");
    std::io::stdin().read_line(&mut login.password).unwrap();

    login
}

fn user_login() -> String {
    let mut user_uuid: String = String::new();
    let login = run_login();
    println!("{:?}", login);
    user_uuid
}

fn main() { 

    /* TODO: First time dialog checks */
    let user_uuid = user_login();

    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bitmatoes Chess".into(),
                width: ((CELLSIZE * 8) + 128 + CELLSIZE * 6) as f32,
                height: ((CELLSIZE * 8) + 128 + (CELLSIZE / 2))  as f32,
                resizable: false,
                ..default()
            },
            ..default()
        })
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        )
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InteractionPlugin)
        .add_state(game_settings::LogicalGameState::Splash)
        
        .add_startup_system(spawn_camera)
        .add_startup_system(set_window_icon)

        // local plugins
        .add_plugin(splash_screen::SplashScreen)
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(game_screen::GameplayPlugin)
        .add_startup_system_to_stage(StartupStage::PostStartup, asset_loading)

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
        global_font: assets.load(FONT_FILE), 
        menu_logo: assets.load("branding/logo_bitmato_chess_light_1200.png"),
        test_scene: assets.load("yellow_frame1.glb#Scene0"),
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            //clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::None,
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
        transform: Transform::from_xyz(10.0, -5., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        camera_3d: Camera3d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::None,
            ..default()
        },
        camera: Camera {
            priority: 2,
            ..default()
        },
        ..default()
    });

}