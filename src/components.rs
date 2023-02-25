use crate::network_handler;
use crate::chess_engine;
use bevy::prelude::*;

/* Lots of constants for us */
pub const FONT_FILE: &'static str = "fonts/CaskaydiaCoveRegular.otf";
pub const CELLSIZE: i32 = 64; // TODO: Make this better (dynamic cell size)
pub const BLACK_TEXT: Color = Color::DARK_GRAY;
pub const WHITE_TEXT: Color = Color::rgb(99., 103., 110.);

#[derive(Component)]
pub struct GlobalThing; // just a struct so we can grab game state everywhere;

#[derive(Component)]
pub struct CurrentTurnText;

#[derive(Component)]
pub struct CurrentPieceText;

#[derive(Component)]
pub struct CurrentFenText;

#[derive(Component, Clone)]
pub struct Piece;


#[derive(Component, Default, Clone, Debug)]
pub struct GameState {
    pub board: chess_engine::Board,
    pub selected: Option<chess_engine::GameEntity>,
    pub last_state: String,
    pub player_team: chess_engine::TeamLoyalty, // someone has to decide eventually
    pub original_cell_index: u32,
}

// TODO: TCP Stuff
#[derive(Component)]
pub struct ClientNative {
    pub tcp_client: network_handler::Client
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Cell {
    pub position: chess_engine::Vec2,
    pub occupier: Option<chess_engine::GameEntity>,
}


/* Main Menu Components */
#[derive(Component)]
pub struct MenuUIroot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

/* For our assets */
pub const PAWN_FILENAME: &'static str = "_pawn_png_shadow_128px.png";
pub const ROOK_FILENAME: &'static str = "_rook_png_shadow_128px.png";
pub const BISHOP_FILENAME: &'static str = "_bishop_png_shadow_128px.png";
pub const KNIGHT_FILENAME: &'static str = "_knight_png_shadow_128px.png";
pub const QUEEN_FILENAME: &'static str = "_queen_png_shadow_128px.png";
pub const KING_FILENAME: &'static str = "_king_png_shadow_128px.png";

#[derive(Resource)]
pub struct AssetHandler {
    pub pawn: Vec<Handle<Image>>,
    pub rook: Vec<Handle<Image>>,
    pub bishop: Vec<Handle<Image>>,
    pub knight: Vec<Handle<Image>>,
    pub queen: Vec<Handle<Image>>,
    pub king: Vec<Handle<Image>>,

    pub global_font: Handle<Font>,
}

pub fn get_piece_asset(asset_handler: &AssetHandler, 
    piece_type: &chess_engine::EntityType, 
    team: chess_engine::TeamLoyalty) -> Handle<Image> 
{  
    let diff = if team == chess_engine::TeamLoyalty::WHITE { 0 } else { 1 };
    match piece_type {
        chess_engine::EntityType::PAWN => asset_handler.pawn[diff].clone(),
        chess_engine::EntityType::ROOK => asset_handler.rook[diff].clone(),
        chess_engine::EntityType::BISHOP => asset_handler.bishop[diff].clone(),
        chess_engine::EntityType::KNIGHT => asset_handler.knight[diff].clone(),
        chess_engine::EntityType::QUEEN => asset_handler.queen[diff].clone(),
        chess_engine::EntityType::KING => asset_handler.king[diff].clone(),
        _ => asset_handler.pawn[0].clone(),
    }
}