use game2048::up;


#[test]
fn up_it_not_modify_empty_board () {
    let mut board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];


    let originalBoard = board;
    up(&board);

    assert_eq!(originalBoard, board);
}

#[test]
fn up_it_moves_elements_up () {
    let mut board = [
        [0, 0, 0, 0],
        [2, 2, 2, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];

    up(&board);

    assert_eq!(
        [
        [2, 2, 2, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ], board
    )
}