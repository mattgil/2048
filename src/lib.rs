pub fn up(board: &mut[ [i32; 4]; 4]) {
    let mut occupied_spots = [0; 16];
    for i in 0..board.len() {
        for j in 0..board[i].len() {

            
            if board[i][j] != 0 {
                let mut free_spot = j;
                while occupied_spots[free_spot] != 0 {
                    println!("gg");
                    free_spot += 4;
                }
                occupied_spots[free_spot] = 1;
                free_spot /= 4;
                println!("{}", free_spot);
                board[free_spot][j] = board[i][j];
                board[i][j] = 0;
                occupied_spots[ j + i * 4] = 0;
               
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