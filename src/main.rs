use game2048::up;


fn main() {
    println!("Hello, world!");
    let  board = &mut [
        [0, 0, 0, 0],
        [2, 2, 2, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];
     up(
        board
    );
    println!("{:?}", board);
}
