pub fn up(board: &mut[ [i32; 4]; 4]) {
    let mut occupied_spots = [0; 16];
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] != 0 {
                let spot = j + i * 4 ;
                occupied_spots[spot] = 1;
                let mut top_free_spot = j;
                while occupied_spots[top_free_spot] == 1 {
                    
                    top_free_spot += 4;
                    if top_free_spot >= spot {
                        break;
                    }
                }
            
                if top_free_spot >= spot {
                    occupied_spots[spot] = 1;
                    continue;
                }
                let top_row = top_free_spot / 4;
                board[top_row][j] = board[i][j];
                board[i][j] = 0;
                occupied_spots[spot] = 0;
                occupied_spots[top_free_spot] = 1;
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