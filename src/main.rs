enum Player {
    X,
    O,
}

struct Board {
    grid: [[Result<Player, &'static str>; 3]; 3],
    current_turn: Player,
    winner: Result<Player, &'static str>,
}

fn main() {
    println!("tic tac toe!");
    println!(
        "------------\n\
        | 1 | 2 | 3 |\n\
        -------------\n\
        | 4 | 5 | 6 |\n\
        -------------\n\
        | 7 | 8 | 9 |\n\
        -------------"
    );

    let mut board = Board {
        grid: [
            [Err(""), Err(""), Err("")],
            [Err(""), Err(""), Err("")],
            [Err(""), Err(""), Err("")],
        ],
        current_turn: Player::X,
        winner: Err(""),
    };

    for row in board.grid {
        for square in row {
            print!("|");
            match square {
                Ok(Player::X) => print!(" X "),
                Ok(Player::O) => print!(" O "),
                Err("") => print!("   "),
                _ => print!("Testing"),
            }
        }
        println!("|<- next line");
        println!("-------------");
    }
}
