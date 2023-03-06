

#[allow(unused)]
#[allow(unused_variables)]

/* Bevy & Third party includes */
// use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::{prelude::*, winit::WinitWindows, window::WindowId};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_interact_2d::*;
use winit::window::Icon;
use std::io::{Write, Read};
use image;

/* Local Includes */
mod network_handler;
mod config_handler;
mod splash_screen;
mod game_settings;
mod chess_engine;
mod game_screen;
mod lobby_setup;
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

/// It asks for a username and password, then returns a string with the username and password separated
/// by a character
/// 
/// Returns:
/// 
/// A String
fn run_login() -> String{
    extern crate rpassword;
    use rpassword::read_password; 

    let mut login = Login{ username: String::new(), password: String::new() };

    print!("Enter DISCORD Username: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut login.username).unwrap();

    login.username = login.username.trim().to_string();
    // TODO: Check if it follows discord name scheme

    print!("Enter a Password (NOT FOR DISCORD): ");
    std::io::stdout().flush().unwrap();
    login.password = read_password().unwrap();

    login.username + SPLIT_CHAR + &login.password
}

fn get_server_ip() -> String {
    // TEMP function for debug etc etc
    let mut server_ip_addr = String::new();

    print!("Enter Server IP/URL: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut server_ip_addr).unwrap();

    server_ip_addr = server_ip_addr.trim().to_string();
    server_ip_addr
}
/// The main function of the program. It is the first function that is called when the program is run.
fn main() { 
    println!(r#" 
     _______  __   __                             __                      ______  __                                 
    /       \/  | /  |                           /  |                    /      \/  |                                
    $$$$$$$  $$/ _$$ |_   _____  ____   ______  _$$ |_    ______        /$$$$$$  $$ |____   ______   _______ _______ 
    $$ |__$$ /  / $$   | /     \/    \ /      \/ $$   |  /      \       $$ |  $$/$$      \ /      \ /       /       |
    $$    $$<$$ $$$$$$/  $$$$$$ $$$$  |$$$$$$  $$$$$$/  /$$$$$$  |      $$ |     $$$$$$$  /$$$$$$  /$$$$$$$/$$$$$$$/ 
    $$$$$$$  $$ | $$ | __$$ | $$ | $$ |/    $$ | $$ | __$$ |  $$ |      $$ |   __$$ |  $$ $$    $$ $$      $$      \ 
    $$ |__$$ $$ | $$ |/  $$ | $$ | $$ /$$$$$$$ | $$ |/  $$ \__$$ |      $$ \__/  $$ |  $$ $$$$$$$$/ $$$$$$  $$$$$$  |
    $$    $$/$$ | $$  $$/$$ | $$ | $$ $$    $$ | $$  $$/$$    $$/       $$    $$/$$ |  $$ $$       /     $$/     $$/ 
    $$$$$$$/ $$/   $$$$/ $$/  $$/  $$/ $$$$$$$/   $$$$/  $$$$$$/         $$$$$$/ $$/   $$/ $$$$$$$/$$$$$$$/$$$$$$$/  
    
    Lead Programmer   : May Draskovics (MayD524#8008)
    Lead Art/Designer : Roscoe Lamontagne (scoe#2222)

    Project Github: https://github.com/Bitmato-Studio/Bitmato-Chess
    
    "#);
    /* TODO: First time dialog checks */
    let login_data = run_login();
    let server_ip = get_server_ip();

    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bitmatoes Chess".into(),
                width: ((CELLSIZE * 8) + 128 + CELLSIZE * 6) as f32,
                height: ((CELLSIZE * 8) + 128 + (CELLSIZE / 2))  as f32,
                // 672
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
        
        .insert_resource(network_handler::Client::create_client(server_ip, components::BUFFER_SIZE, login_data).unwrap())

        .add_startup_system(spawn_camera)
        .add_startup_system(set_window_icon)

        // local plugins
        .add_plugin(splash_screen::SplashScreen)
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(game_screen::GameplayPlugin)
        .add_plugin(lobby_setup::LobbySetup)
        .add_startup_system(init_networking)
        .add_startup_system_to_stage(StartupStage::PostStartup, asset_loading)

        .run();
}

fn init_networking(mut cli: ResMut<network_handler::Client>) {
    cli.recv().unwrap(); // void the first msg
    
    cli.login().unwrap();
    println!("{}", cli.to_string());

    cli.send_cmd("PING".to_string(), "hello world".to_string()).unwrap();
    let data = cli.recv().unwrap();

    println!("Data: {}", data);
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