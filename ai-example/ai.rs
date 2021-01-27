#[no_mangle]
pub extern "C" fn ai(
    board: *const [[u32; 10]; 24],
    current_piece: *const [[i32; 2]; 4],
    saved_piece: *const [[i32; 2]; 4],
) -> u32 {
    unsafe { calculate_move(*board, *current_piece, *saved_piece) }
}

//A really braindead AI which will move until one block is over the least filled coloumn
fn calculate_move(
    board: [[u32; 10]; 24],
    current_piece: [[i32; 2]; 4],
    saved_piece: [[i32; 2]; 4],
) -> u32 {
    let mut action = 0;
    let mut rows = [0; 10];
    for l in 0..24 {
        for i in 0..10 {
            if board[l][i] != 0 {
                rows[i] = l;
            }
        }
    }
    let mut row = 0;
    let mut min_val = 24;
    for (xx, t) in rows.iter().enumerate() {
        if t < &min_val {
            min_val = *t;
            row = xx;
        }
    }
    for [x, y] in &current_piece {
        if x < &0 || x > &10 {
            //Ignore eventual errors
            continue;
        }
        if *x == row as i32 {
            //if piece is above best row, do nothing
            action = 8;
            break;
        } else if *x < row as i32 {
            //if to the left of best row, move right
            action = 2;
        } else {
            //else move left
            action = 1;
        }
    }
    action
}
