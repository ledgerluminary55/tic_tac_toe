use std::io::stdin;
use std::io::stdout;
use std::io::Write;
#[derive(Debug, Clone, Copy, PartialEq)]
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
        "The board is structured as follows \n 
                "
    );
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

    while board.winner.is_err() {
        print!("Player {:?}, input your play>>", board.current_turn);
        stdout().flush().expect("Could not flush stdout");
        let mut turn = String::new();
        stdin().read_line(&mut turn).expect("Failed to read line");
        //print!("{}", &mut turn);
        let play: Result<usize, _> = turn.trim().parse();
        //turn.clear();

        //match play {
        //  Ok(n) => print!("testing"),
        //Err(_) => print!("checking")
        //}

        let x = play.unwrap() - 1;
        let y = x / 3;
        let z = x % 3;

        if x > 8 || board.grid[y][z].is_ok() {
            continue;
        }

        board.grid[y][z] = Ok(board.current_turn);

        //if board.grid[y][z].is_ok() || x > 8 {
        //  continue;
        //}

        println!("-------------");
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
            println!("|");
            println!("-------------");
        }

        for row in board.grid {
            if row[0] == row[1] && row[1] == row[2] && row[0].is_ok() {
                board.winner = row[0]
            }
        }

        for i in 0..3 {
            if board.grid[0][i] == board.grid[1][i]
                && board.grid[1][i] == board.grid[2][i]
                && board.grid[0][i].is_ok()
            {
                board.winner = board.grid[0][i];
            }
        }

        if board.grid[0][0] == board.grid[1][1]
            && board.grid[1][1] == board.grid[2][2]
            && board.grid[0][0].is_ok()
        {
            board.winner = board.grid[0][0];
        }
        if board.grid[0][2] == board.grid[1][1]
            && board.grid[1][1] == board.grid[2][0]
            && board.grid[0][2].is_ok()
        {
            board.winner == board.grid[0][2];
        }


        match board.winner {
            Ok(Player::X) => {
                println!("X wins!");
                break;
            }
            Ok(Player::O) => {
                println!("O wins!");
                break;
            }
            Err("") => (),
            _ => println!("Draw!"),
        }

        board.current_turn = match board.current_turn {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
