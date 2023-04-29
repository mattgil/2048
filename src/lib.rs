pub fn up(board: &mut[ [i32; 4]; 4]) {
    for i in 1..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] != 0 {
                board[0][j] = board[i][j];
                board[i][j] = 0;
            }
        }
    }
}

pub fn down() {

}

pub fn left() {

}

pub fn right() {

}