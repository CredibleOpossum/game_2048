//TODO remove magic numbers and allow variable board size
//TODO add better input method?

use rand::seq::SliceRandom;
use text_io::read;

fn print_board(board: &[[i32; 4]; 4]) {
    for array in board {
        println!();
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
    }
    println!();
}

fn score(board: &[[i32; 4]; 4]) -> i32 {
    let mut sum = 0;
    for col in 0..4 {
        for row in 0..4 {
            sum += board[col][row];
        }
    }
    return sum;
}

fn merge(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    let mut new_board = *board;
    for col in 0..4 {
        let mut last_var = 0;
        let mut last_position = 0;
        for row in 0..4 {
            if new_board[col][row] != 0 {
                if new_board[col][row] == last_var {
                    new_board[col][row] += last_var;
                    new_board[col][last_position] = 0;

                    last_var = 0;
                    last_position = 0;
                } else {
                    last_var = new_board[col][row];
                    last_position = row;
                }
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

fn move_left(board: &[[i32; 4]; 4]) -> [[i32; 4]; 4] {
    let mut new_board = merge(board);
    for _ in 0..4 {
        for col in 0..4 {
            for row in 0..3 {
                if new_board[col][row + 1] != 0 {
                    if new_board[col][row] == 0 {
                        new_board[col][row] = new_board[col][row + 1];
                        new_board[col][row + 1] = 0;
                    }
                }
            }
        }
    }
    return new_board;
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

fn add_two(board: &mut [[i32; 4]; 4]) {
    let mut free_spaces = Vec::new();
    for col in 0..4 {
        for row in 0..4 {
            if board[col][row] == 0 {
                free_spaces.push([col, row]);
            }
        }
    }

    if free_spaces.len() != 0 {
        let random_position = free_spaces.choose(&mut rand::thread_rng()).unwrap();
        board[random_position[0]][random_position[1]] = 2;
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

//? board[(index / 4) as usize][(index % 4) as usize] = num;
fn main() {
    let mut board = [[0_i32; 4]; 4];
    add_two(&mut board);
    add_two(&mut board);
    loop {
        print_board(&board);
        println!("{}", score(&board));
        let input: String = read!("{}\n");
        let temp_board = match input.as_str() {
            "w" => move_up(&board),
            "a" => move_left(&board),
            "s" => move_down(&board),
            "d" => move_right(&board),
            _ => board,
        };
        if temp_board != board {
            board = temp_board;
            add_two(&mut board);
        } else {
            if check_end(board) {
                println!("No more possible moves, game has ended.");
                println!("Final score: {}", score(&board));
                break;
            }
        }
    }
}
