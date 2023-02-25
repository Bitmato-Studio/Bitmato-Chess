#[allow(unused)]
#[allow(unused_variables)]
mod chess_engine;

fn main() {
    let mut test_board = chess_engine::Board::create_board(chess_engine::DEFAULTFEN.to_owned(), chess_engine::BoardResolution::default());
    
    loop {
        let mut x_str = String::new();
        let mut y_str = String::new();
        print!("\x1B[2J\x1B[1;1H");

        println!("{}", test_board.to_string());
        println!("{}", test_board.to_fen());
        println!("{:?}", test_board.current_turn);
        
        println!("Select Piece X: ");
        std::io::stdin().read_line(&mut x_str).unwrap();
        println!("Select Piece Y: ");
        std::io::stdin().read_line(&mut y_str).unwrap();

        let start_pos = chess_engine::Vec2 {
            x: x_str.trim().parse().expect(&format!("x_str = {:?}", x_str)),
            y: y_str.trim().parse().expect(&format!("y_str = {:?}", y_str))
        };

        x_str = String::new();
        y_str = String::new();

        let ent = test_board.entity_at(start_pos);

        if ent.is_none() {
            println!("Invalid selection");
            continue;
        }

        if ent.clone().unwrap().team_id != test_board.current_turn{
            println!("Invalid selection");
            continue;
        }

        println!("Selected: {:#?}", ent.clone().unwrap());

        println!("Select Move X: ");
        std::io::stdin().read_line(&mut x_str).unwrap();
        println!("Select Move Y: ");
        std::io::stdin().read_line(&mut y_str).unwrap();

        let end_pos = chess_engine::Vec2 {
            x: x_str.trim().parse().expect(&format!("x_str = {:?}", x_str)),
            y: y_str.trim().parse().expect(&format!("y_str = {:?}", y_str))
        };

        chess_engine::move_entity(&mut test_board, start_pos, end_pos);

        if test_board.current_turn == chess_engine::TeamLoyalty::WHITE {
            test_board.current_turn = chess_engine::TeamLoyalty::BLACK;
        } else {
            test_board.current_turn =chess_engine::TeamLoyalty::WHITE;
        }
    } 
}
