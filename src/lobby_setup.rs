use crate::game_settings::LogicalGameState;
use crate::network_handler::*;
use crate::components::*;
use bevy::prelude::*;
use std::time::Duration;

pub struct LobbySetup;

#[derive(Resource)]
struct LocalTimer {
    timer: Timer
}

impl Plugin for LobbySetup {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(LogicalGameState::Lobby)
                .with_system(start_lobby_search)
            )
            .add_system_set(SystemSet::on_update(LogicalGameState::Lobby)
                .with_system(during_update)
                // add other update resources
            )
            .insert_resource(LocalTimer {
                timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
            });
    }
}

fn during_update(
    mut timer: ResMut<LocalTimer>,
    mut player_text: Query<&mut Text, With<LobbyPlayerCountText>>,
    mut game_state: ResMut<State<LogicalGameState>>,
    mut cli: ResMut<Client>,
    time: Res<Time>,
) {

    timer.timer.tick(time.delta());

    if ! timer.timer.finished() { return; }

    let mut playerc_text = player_text.single_mut();
    // get the players online

    cli.send("TP".to_string()).unwrap();
    let new_data = cli.recv().unwrap();

    if new_data.contains(&"MATCHES") {
        // we got in a match
        game_state.set(LogicalGameState::Game).unwrap();
        return;
    }

    playerc_text.sections[0].value = new_data;

}


fn start_lobby_search(
    mut commands: Commands,
    mut cli: ResMut<Client>,
    asset_server: Res<AssetServer>,
    game_assets: Res<AssetHandler>,
) {
    cli.send("RMM".to_string()).unwrap();

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new( 
                "Match Making...",
                TextStyle {
                    font: game_assets.global_font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
        ]).with_text_alignment(TextAlignment::CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(100.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        LobbyText
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new( 
                "Getting players...",
                TextStyle {
                    font: game_assets.global_font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
        ]).with_text_alignment(TextAlignment::CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(140.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        LobbyText,
        LobbyPlayerCountText
    ));

}