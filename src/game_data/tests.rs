use super::{Color, Piece, Player, COLS, ROWS, SHAPES};
use std::thread;

#[test]
fn rotation() {
    let mut player = Player::new(0);
    player.current_piece = Piece::new(SHAPES[2], Color::Color1, [2, 2]);
    assert_eq!(
        [[1, 2], [2, 2], [2, 1], [3, 2]],
        player.current_piece.pos_on_board()
    );
    player.current_piece.rotate(true);
    assert_eq!(
        [[2, 3], [2, 2], [1, 2], [2, 1]],
        player.current_piece.pos_on_board()
    );
    player.current_piece.rotate(true);
    assert_eq!(
        [[3, 2], [2, 2], [2, 3], [1, 2]],
        player.current_piece.pos_on_board()
    );
    player.current_piece.rotate(true);
    assert_eq!(
        [[2, 1], [2, 2], [3, 2], [2, 3]],
        player.current_piece.pos_on_board()
    );
}

#[test]
fn line_clear() {
    let mut player = Player::new(0);
    for i in 0..COLS {
        player.current_piece = Piece::new(SHAPES[0], Color::Color1, [i as i32, 1]);
        player.rotate_current(true);
        let mut loop_var = 0;
        println!("-------------------------------------------");
        for line in &player.get_board_visual() {
            loop_var += 1;
            if loop_var > (ROWS - 4) {
                break;
            }
            print!("|");
            for point in line {
                print!(" ");
                if *point == 0 {
                    print!(" ");
                } else {
                    print!("*");
                }
            }
            println!("|");
        }
        println!("-------------------------------------------");
        player.move_tick();
    }
    let mut is_cleared = true;
    for line in &player.board {
        for block in line {
            if block != &0 {
                is_cleared = false;
            }
        }
    }
    let mut loop_var = 0;
    println!("-------------------------------------------");
    for line in &player.board {
        loop_var += 1;
        if loop_var > (ROWS - 4) {
            break;
        }
        print!("|");
        for point in line {
            print!(" ");
            if *point == 0 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!("|");
    }
    println!("-------------------------------------------");
    assert!(is_cleared);
    //print!("{}[2J", 27 as char);
}

#[test] //not real test!
fn console_debug() {
    let mut player = Player::new(0);
    loop {
        print!("{}[2J", 27 as char);
        let mut loop_var = 0;
        println!("-------------------------------------------");
        for line in &player.get_board_visual() {
            loop_var += 1;
            if loop_var > (ROWS - 4) {
                break;
            }
            print!("|");
            for point in line {
                print!(" ");
                if *point == 0 {
                    print!(" ");
                } else {
                    print!("*");
                }
            }
            println!("|");
        }
        println!("-------------------------------------------");
        if player.lost {
            break;
        }
        thread::sleep_ms(100);
        player.rotate_current(true);
        player.move_tick();
    }
    println!("game lost!");
    assert_eq!(true, player.lost);
}
