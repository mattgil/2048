use game2048::up;


#[test]
fn up_it_not_modify_empty_board () {
    let mut board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];


    up(&mut board);

    assert_eq!(
        [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        , board);
}

#[test]
fn up_it_moves_elements_up () {
    let mut board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 2, 2, 2],
    ];

    up(&mut board);

    assert_eq!(
        [
        [2, 2, 2, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ], board
    )
}



#[test]
fn move_multiple_rows_up() {
    let mut board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 2, 2, 2],
        [4, 4, 2, 4],
    ];

    up(&mut board);

    assert_eq!(
        [
            [2, 2, 2, 2],
            [4, 4, 2, 4],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
    ], board
    )
}

#[test]
fn move_non_regular_rows_up() {
    let mut board = [
        [0, 0, 0, 0],
        [0, 0, 2, 0],
        [2, 0, 2, 2],
        [4, 4, 8, 4],
    ];

    up(&mut board);

    assert_eq!(
        [
            [2, 4, 2, 2],
            [4, 0, 2, 4],
            [0, 0, 8, 0],
            [0, 0, 0, 0],
    ], board
    )
}

#[test]
fn move_when_column_is_full_up() {
    let mut board = [
        [2, 0, 0, 0],
        [2, 0, 2, 0],
        [2, 0, 2, 2],
        [4, 4, 8, 4],
    ];

    up(&mut board);

    assert_eq!(
        [
            [2, 4, 2, 2],
            [2, 0, 2, 4],
            [2, 0, 8, 0],
            [4, 0, 0, 0],
    ], board
    )
}