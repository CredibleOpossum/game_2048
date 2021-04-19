//TODO remove magic numbers and allow variable board size
//TODO add better input method?

use rand::seq::SliceRandom;
use text_io::read;

fn main() {
    let mut board = [[0_i32; 4]; 4];
    random_two(&mut board, 2);
    loop {
        print_board(&board);
        println!("{}", score(&board));
        let input: String = read!("{}\n");
        let future_board = match input.as_str() {
            "w" => move_up(&board),
            "a" => move_left(&board),
            "s" => move_down(&board),
            "d" => move_right(&board),
            _ => board,
        };
        if future_board != board {
            board = future_board;
            random_two(&mut board, 1);
            if check_end(board) {
                println!("No more possible moves, game has ended.");
                println!("Final score: {}", score(&board));
                break;
            }
        }
    }
}

fn check_end(board: [[i32; 4]; 4]) -> bool {
    match board {
        _ if move_left(&board) != board => return false,
        _ if move_right(&board) != board => return false,
        _ if move_up(&board) != board => return false,
        _ if move_down(&board) != board => return false,
        _ => return true,
    }
}

fn move_left(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    return compress(&merge(&compress(&board)));
}

fn move_right(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    return reverse(&move_left(&reverse(board)));
}

fn move_up(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    return transpose(&move_left(&transpose(board)));
}

fn move_down(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    return transpose(&move_right(&transpose(board)));
}

fn score(board: &[[i32; 4]; 4]) -> i32 {
    let mut sum = 0;
    for col in board {
        for value in col {
            sum += value;
        }
    }
    return sum;
}

fn print_board(board: &[[i32; 4]; 4]) {
    for array in board {
        for value in array {
            if value != &0 {
                print!(
                    "{:?}{}",
                    value,
                    " ".repeat(6 - value.to_string().chars().count())
                );
            } else {
                print!("_{}", " ".repeat(6 - value.to_string().chars().count()))
            }
        }
        println!();
    }
}

fn compress(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    let mut new_board = [[0_i32; 4]; 4];
    for col in 0..4 {
        let mut position = 0;
        for row in 0..4 {
            if board[col][row] != 0 {
                new_board[col][position] = board[col][row];
                position += 1;
            }
        }
    }
    return new_board;
}

fn merge(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    let mut new_board = *board;
    for col in 0..4 {
        for row in 0..3 {
            if new_board[col][row] == new_board[col][row + 1] && new_board[col][row + 1] != 0 {
                new_board[col][row] *= 2;
                new_board[col][row + 1] = 0;
            }
        }
    }
    return new_board;
}

fn reverse(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    let mut new_board = *board;
    for col in 0..4 {
        new_board[col].reverse();
    }
    return new_board;
}

fn transpose(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    let mut new_board = *board;
    for col in 0..4 {
        for row in 0..4 {
            new_board[row][col] = board[col][row];
        }
    }
    return new_board;
}

fn random_two(board: &mut [[i32; 4]; 4], amount: i32) {
    let mut free_spaces = Vec::new();
    for col in 0..4 {
        for row in 0..4 {
            if board[col][row] == 0 {
                free_spaces.push([col, row]);
            }
        }
    }
    for _ in 0..amount {
        if free_spaces.len() != 0 {
            let random_position = free_spaces.choose(&mut rand::thread_rng()).unwrap();
            board[random_position[0]][random_position[1]] = 2;
        }
    }
}
