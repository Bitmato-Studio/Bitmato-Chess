/*
    TODO: Check for check(mate)
    TODO: Check for game over condition
 */

use std::{cmp::Ordering, ops::Range};

use crate::components::create_range_vector;

pub const DEFAULTFEN: &str = "rnbkqbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w";

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TeamLoyalty {
    NONE,
    WHITE,
    BLACK,
}

impl Default for TeamLoyalty {
    fn default() -> Self {
        TeamLoyalty::WHITE
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EntityType {
    NOTSET,
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING
}

impl Default for EntityType{
    fn default() -> Self {
        Self::NOTSET
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// This is for the true screen
// X and Y positions of the game object
// This will get used to also handle clicks
// and placing and drawing.
pub type Position = Vec2;

#[derive(Default, Debug, Clone, Copy,PartialEq)]
pub struct GameEntity {
    pub entity_type: EntityType,
    pub team_id: TeamLoyalty,
    pub first_move: bool, 
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub is_occupied : bool,
    pub occupier: Option<GameEntity>,
    pub cell_fen_repr: char, // just a single character
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            is_occupied: false,
            occupier: None,
            cell_fen_repr: '1',
        }
    }
}

impl Cell {
    pub fn make_empty(&mut self) -> Option<GameEntity> {
        self.cell_fen_repr = '1';
        self.is_occupied = false;
        let last_occupier = self.occupier.clone();
        self.occupier = None;
        return last_occupier;
    }

    pub fn update(&mut self, new_entity: GameEntity) {
        let moved_entity = new_entity.clone();
        let mut fen_char = get_entity_fen(&moved_entity.entity_type).to_string();

        if new_entity.team_id == TeamLoyalty::WHITE {
            fen_char = fen_char.to_uppercase();
        }

        self.cell_fen_repr = fen_char.chars().next().unwrap();
        self.is_occupied = true;
        self.occupier = Some(moved_entity);
    }
}

#[derive(Default, Debug, Clone)]
pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub current_turn: TeamLoyalty,
    pub is_check: bool,
    pub is_checkmate: bool,
    pub who_in_check: TeamLoyalty,
}

// just lowercase rnqkbp make upper if needed
pub fn get_entity_fen(ent_type: &EntityType) -> char {
    match ent_type {
        EntityType::PAWN => 'p',
        EntityType::ROOK => 'r',
        EntityType::KNIGHT => 'n',
        EntityType::BISHOP => 'b',
        EntityType::QUEEN => 'q',
        EntityType::KING => 'k',
        _ => 0 as char,
    }
}

pub fn get_entity_type(fen_equiv:char) -> EntityType {
    match fen_equiv {
        'p'|'P' => EntityType::PAWN,
        'r'|'R' => EntityType::ROOK,
        'n'|'N' => EntityType::KNIGHT,
        'b'|'B' => EntityType::BISHOP,
        'q'|'Q' => EntityType::QUEEN,
        'k'|'K' => EntityType::KING,
        _ => EntityType::NOTSET,
    }
}

pub fn make_entity(fen_equiv:char, team: TeamLoyalty) -> GameEntity {
    GameEntity { 
        entity_type: get_entity_type(fen_equiv),
        team_id: team,
        first_move: true,
    }
}

pub fn create_cell(fen_equiv:char) -> Cell {

    // start looking for numbers
    // 9 in ascii is 57 if so we can
    // start checking for numbers (only using 1 if cause lazy)
    // 
    let game_entity:Option<GameEntity> = if fen_equiv < 57 as char {
        None
    } else {
        let loyalty = if fen_equiv < 97 as char {
            TeamLoyalty::WHITE
        } else { 
            TeamLoyalty::BLACK 
        };
        Some(make_entity(fen_equiv, loyalty))
    };

    Cell {
        is_occupied: true,
        occupier: game_entity,
        cell_fen_repr: fen_equiv,
    }
}

fn d(from: Vec2, to: Vec2) -> (usize, usize) {
    let dx = match from.x.cmp(&to.x) {
        Ordering::Less => to.x - from.x,
        Ordering::Greater => from.x - to.x,
        Ordering::Equal => 0,
    };

    let dy = match from.y.cmp(&to.y) {
        Ordering::Less => to.y - from.y,
        Ordering::Greater => from.y - to.y,
        Ordering::Equal => 0,
    };

    (dx as usize, dy as usize)
}

pub fn validate(board: &Board, from: Vec2, to:Vec2, piece: GameEntity) -> bool {
    let (dx, dy) = d(from, to);

    match piece.entity_type {
        EntityType::PAWN => {
            if piece.first_move { 
                return (dx == 0 && (dy == 1 || dy == 2) && board.entity_at(to).is_none()) || (dx == 1 && dy == 1 && board.entity_at(to).is_some());
            } else {
                return (dx == 0 && dy == 1 && board.entity_at(to).is_none()) || (dx == 1 && dy == 1 && board.entity_at(to).is_some());    
            }
        }
        EntityType::ROOK => {
            return (dx == 0 && dy != 0) || (dx != 0 && dy == 0);
        },
        EntityType::BISHOP => {
            return dx == dy && dx != 0;
        },
        EntityType::KNIGHT => {
            return (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
        },
        EntityType::QUEEN => {
            return (dx == dy && dx != 0) || (dx == 0 && dy != 0) || (dx == 0 && dy != 0);
        },
        EntityType::KING => {
            return dx == 1 || dy == 1;
        },
        _ => false
    }
}

fn validate_horizontal(board: &Board, start: Vec2, end: Vec2, piece: GameEntity) -> bool {
    if start.x - end.x == 0 { return true; } // didn't move horizonally
    // Check X
    for x in start.x..end.x {
        let new_pos = Vec2 {
            x,
            y: start.y,
        };
        let ent_at = board.entity_at(new_pos);
        if ent_at.is_some() && ent_at.unwrap() != piece{
            println!("Horizontal: Colliding with {:?} at {:?}", ent_at, new_pos);
            return ent_at.unwrap().team_id != piece.team_id;
        }
    }
    true
}

fn validate_vertical(board: &Board, start: Vec2, end: Vec2, piece: GameEntity) -> bool {
    // Check Y
    if start.y - end.y == 0 { return true; } // didn't move vertically
    for y in start.y..end.y {
        let new_pos = Vec2 {
            x: start.x,
            y,
        };
        let ent_at = board.entity_at(new_pos);
        if ent_at.is_some() && ent_at.unwrap() != piece{
            println!("Vertical: Colliding with {:?} at {:?}", ent_at, new_pos);
            return ent_at.unwrap().team_id != piece.team_id;
        }
    }
    true
}

fn validate_diagonal(board: &Board, start: Vec2, end: Vec2, piece: GameEntity) -> bool {
    // triangles
    if start.y - end.y == 0 || start.x - end.x == 0 { return false; } // return false if we don't move diagonally
    
    // issue when going down and left... yay { hopefully this fixes it}
    let x_iter = create_range_vector(start.x, end.x);
    let y_iter = create_range_vector(start.y, end.y);

    for y in y_iter {
        /* FIXME: Issue with moving downwards diagonal (Repeatable)
            thread 'Compute Task Pool (1)' panicked at 'Not enough X', src\chess_engine.rs:255:31
            note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
            thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', C:\Users\Cross\.cargo\registry\src\github.com-1ecc6299db9ec823\bevy_tasks-0.9.1\src\task_pool.rs:273:45
        */
        let tx = x_iter.get((start.y-y) as usize);
    
        if tx.is_none() {
            return true;
        }

        let x = *tx.unwrap();

        let new_pos = Vec2 {
            y,
            x
        };

        let ent_at = board.entity_at(new_pos);
        println!("{:?}", ent_at);
        if ent_at.is_some() && ent_at.unwrap() != piece{
            println!("Diagonal: Colliding with {:?} at {:?}", ent_at, new_pos);

            return ent_at.unwrap().team_id != piece.team_id;
        }
    }
    true
}

pub fn move_entity(board: &mut Board, original:Position, new_pos:Position) {
    let tmp_ent = board.cells[original.y as usize][original.x as usize].occupier.clone();

    if tmp_ent.is_none() { return; }
    let mut ent = tmp_ent.unwrap();
    
    let mut is_legal =   validate(board, original, new_pos, ent);

    if ent.entity_type != EntityType::KNIGHT && is_legal {
        is_legal = if validate_diagonal(board, original, new_pos, ent) {
            true
        } else { 
            if !validate_horizontal(board, original, new_pos, ent) {
                println!("Horizontal False");
                false
            } else if !validate_vertical(board, original, new_pos, ent) {
                println!("Vertical False");
                false
            } else {
                true
            }
        };
    }

    if !is_legal {
        println!("Illegal Move!");
        return;
    }

    if ent.first_move {
        ent.first_move = false;
    }

    board.cells[new_pos.y as usize][new_pos.x as usize].update(ent);
    board.cells[original.y as usize][original.x as usize].make_empty();

    board.current_turn = if board.current_turn == TeamLoyalty::WHITE {
        TeamLoyalty::BLACK
    } else {
        TeamLoyalty::WHITE
    };
}

impl Board {

    /* For Debugging  */
    pub fn to_string(&self) -> String {
        let mut out = "- 0 1 2 3 4 5 6 7\n0".to_owned();
        let mut row_i = 1;
        for row in &self.cells {
            for column in row {
                out += &(" ".to_string() + &column.cell_fen_repr.to_string());
            }
            out += &("\n".to_owned() + &row_i.to_string());
            row_i += 1;
        }
        return out;
    }

    pub fn at(&self, pos: Position) -> &Cell {
        return &self.cells[pos.y as usize][pos.x as usize];
    }

    pub fn entity_at(&self, pos:Position) -> &Option<GameEntity> {
        return &self.at(pos).occupier;
    }

    pub fn to_fen(&self) -> String {
        let mut out: String = "".to_owned();
        
        for row in &self.cells {
            let mut acc = 0;
            for cell in row {
                if cell.occupier.is_none() {
                    acc += 1;
                    continue;
                }

                if acc > 0 {
                    out += &acc.to_string();
                    acc = 0;
                }

                out += &cell.cell_fen_repr.to_string();
            }
            if acc > 0 {
                out += &acc.to_string();
            }
            out += &"/";
        }

        out.pop();

        let turn = if self.current_turn == TeamLoyalty::WHITE { "w" } else { "b" };
        out += " ";
        out += turn;
        

        return out;
    }

    pub fn update_turn(&mut self, fen:String) {
        self.current_turn = if fen.chars().last().unwrap() == 'w' { TeamLoyalty::WHITE } else { TeamLoyalty::BLACK };
    }

    pub fn create_board(fen: String) -> Self {
        let mut cells: Vec<Vec<Cell>> = Vec::new();
        let current_turn: TeamLoyalty = if fen.chars().last().unwrap() == 'w' { TeamLoyalty::WHITE } else { TeamLoyalty::BLACK };

        // the first
        cells.push(Vec::new());

        for fen_char in fen.chars() {
            if fen_char == '/' {
                cells.push(Vec::new());
                continue;
            }

            if fen_char == ' ' {
                break;
            }
            if fen_char < (57 as char) && fen_char > (47 as char)  {
                let count = (fen_char as i32) - 47;
                for _ in 0..count-1 {
                    cells.last_mut().unwrap().push(Cell::default());
                }
            } else {
                let data = create_cell(fen_char);
                cells.last_mut().unwrap().push(data);
            }
        }
        Self {
            cells,
            current_turn,
            is_check: false,
            is_checkmate: false,
            who_in_check: TeamLoyalty::NONE,
        }
    }

}