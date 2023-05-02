use game2048::down;

#[test]
fn it_not_modify_empty_board() {
    let mut board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];


    down(&mut board);

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
fn move_one_row() {
    let mut board = [
        [2, 2, 2, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];


    down(&mut board);

    assert_eq!(
        [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [2, 2, 2, 2],
        ]
        , board);
}

#[test]
fn move_multiple_rows() {
    let mut board = [
        [2, 2, 2, 2],
        [4, 4, 4, 4],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];


    down(&mut board);

    assert_eq!(
        [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [2, 2, 2, 2],
            [4, 4, 4, 4],
        ]
        , board);
}

#[test]
fn move_non_regular_rows() {
    let mut board = [
        [2, 2, 2, 2],
        [4, 0, 4, 0],
        [0, 0, 8, 0],
        [0, 0, 0, 0],
    ];


    down(&mut board);

    assert_eq!(
        [
            [0, 0, 0, 0],
            [0, 0, 2, 0],
            [2, 0, 4, 0],
            [4, 2, 8, 2],
        ]
        , board);
}

#[test]
fn move_when_column_is_full() {
    let mut board = [
        [2, 2, 2, 2],
        [4, 4, 0, 4],
        [4, 0, 0, 0],
        [8, 0, 0, 0],
    ];


    down(&mut board);

    assert_eq!(
        [
            [2, 0, 0, 0],
            [4, 0, 0, 0],
            [4, 2, 0, 2],
            [8, 4, 2, 4],
        ]
        , board);
}