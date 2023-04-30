pub fn up(board: &mut[ [i32; 4]; 4]) {
    let mut occupied_spots = [0; 16];
    for i in 0..board.len() {
        for j in 0..board[i].len() {

            
            if board[i][j] != 0 {
                let spot = j + i * 4 ;
                if i == 0  {
                    occupied_spots[spot] = 1;
                    continue;
                }
                
                let mut top_free_spot = j;

                while occupied_spots[top_free_spot] != 1 {
                    top_free_spot += 4;
                    if top_free_spot > 15 {
                        break;
                    }
                }

                if top_free_spot > 15  {
                    occupied_spots[spot] = 1;
                    continue;
                }

                let top_row = top_free_spot / 4;
                println!("{}", top_row);
                board[top_row][j] = board[i][j];
                board[i][j] = 0;
                occupied_spots[spot] = 0;
               
            }
        }
    }
    println!("{:?}", occupied_spots);
}

pub fn down() {

}

pub fn left() {

}

pub fn right() {

}