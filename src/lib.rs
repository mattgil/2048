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

pub fn down(board: &mut[ [i32; 4]; 4]) {
    let mut occupied_spots = [false; 16];
    for i in (0..board.len()).rev() {
        for j in 0..board[i].len()  {
            if board[i][j] != 0 {
                let spot = j + i*4;
                occupied_spots[spot] = true;
                let mut bottom_free_spot = 4*(board.len() - 1) + j;
                while occupied_spots[bottom_free_spot] {
                    bottom_free_spot -=4;
                    if bottom_free_spot <= spot {
                        break;
                    }
                }

                if  bottom_free_spot <= spot {
                    continue;
                }

                let bottom_row = bottom_free_spot/4;

                board[bottom_row][j] = board[i][j];
                board[i][j] = 0;
                occupied_spots[bottom_free_spot] = true;
                occupied_spots[spot] = false;
            }
        }
    }
}

pub fn left(board: &mut[ [i32; 4]; 4]) {
    for i in 0..board.len()  {
        let mut shift = 0;
        for j in 0..board.len() {
            if board[i][j] != 0 {
                if  j == 0 {
                    break;
                }
                board[i][j - shift] = board[i][j];
                board[i][j] = 0;
                if shift > 0{ 
                    shift -= 1;
                }
            }
        shift += 1;
        }
    }
}

pub fn right(board: &mut[ [i32; 4]; 4]) {
    for i in 0..board.len() {
        let mut shift = board[i].len() - 1;
        for j in (0..board[i].len()).rev() {

            if board[i][j] != 0 {
                if j == board[i].len() - 1 {
                    break;
                }
                board[i][shift]  = board[i][j];
                board[i][j] = 0;
                shift -=1;
            }
        }
        
    }
}