use std::time::Duration;

use bevy::prelude::*;
use bevy::window::WindowDescriptor;
use crate::{game_settings::LogicalGameState, components::{is_loaded, AssetHandler}};
pub struct SplashScreen;

const SPLASH_IMAGE: &'static str = "branding/matoface_logo.png";


#[derive(Resource)]
struct LocalTimer {
    timer: Timer,
}

#[derive(Component)]
struct SplashComponent; // so we can clean up later (destroy it all >:) ) 

impl Plugin for SplashScreen {
    fn build(&self, app: &mut App) {
        /* TODO: Splash screen stuff :> */
        app
            .add_system_set(SystemSet::on_enter(LogicalGameState::Splash).with_system(spawn_spash_screen))
            .add_system_set(SystemSet::on_update(LogicalGameState::Splash)
                .with_system(on_tick)
            )
            .insert_resource( LocalTimer {
                timer : Timer::new(Duration::from_secs(5), TimerMode::Once), 
            });
    }
}

fn spawn_spash_screen(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let texture_handle = asset_server.load(SPLASH_IMAGE);
    
    let window = windows.get_primary().unwrap();
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: window.width(),
                y: window.height(),
            }),
            ..default()
        },
        ..default()
    }).insert(SplashComponent);
}

fn on_tick(
    mut commands: Commands,
    time: Res<Time>,
    splash_items: Query<Entity, With<SplashComponent>>,
    mut loc_timer: ResMut<LocalTimer>,
    mut game_state: ResMut<State<LogicalGameState>>,
    asset_server: Res<AssetServer>,
    loading: Res<AssetHandler>,
) {
    loc_timer.timer.tick(time.delta());

    if loc_timer.timer.finished() && is_loaded(asset_server, loading) == 0 {
        game_state.set(LogicalGameState::Menu).unwrap();
        
        for splash in splash_items.iter() {
            commands.entity(splash).despawn();
        }
    
    }
}