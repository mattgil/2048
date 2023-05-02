use game2048::right;

#[test]
fn it_not_modify_empty_board() {
    let mut board = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];


    right(&mut board);

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
fn move_one_column() {
    let mut board = [
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [2, 0, 0, 0],
        [2, 0, 0, 0],
    ];


    right(&mut board);

    assert_eq!(
        [
            [0, 0, 0, 2],
            [0, 0, 0, 2],
            [0, 0, 0, 2],
            [0, 0, 0, 2],
        ]
        , board);
}

#[test]
fn move_multiple_rows() {
    let mut board = [
        [2, 4, 0, 0],
        [2, 4, 0, 0],
        [2, 4, 0, 0],
        [2, 4, 0, 0],
    ];


    right(&mut board);

    assert_eq!(
        [
            [0, 0, 2, 4],
            [0, 0, 2, 4],
            [0, 0, 2, 4],
            [0, 0, 2, 4],
        ]
        , board);
}

#[test]
fn move_non_regular_rows() {
    let mut board = [
        [2, 0, 0, 0],
        [2, 4, 0, 0],
        [2, 0, 0, 0],
        [2, 4, 8, 0],
    ];


    right(&mut board);

    assert_eq!(
        [
            [0, 0, 0, 2],
            [0, 0, 2, 4],
            [0, 0, 0, 2],
            [0, 2, 4, 8],
        ]
        , board);
}

#[test]
fn move_when_column_is_full() {
    let mut board = [
        [2, 0, 0, 0],
        [2, 4, 4, 8],
        [2, 0, 0, 0],
        [2, 4, 8, 0],
    ];


    right(&mut board);

    assert_eq!(
        [
            [0, 0, 0, 2],
            [2, 4, 4, 8],
            [0, 0, 0, 2],
            [0, 2, 4, 8],
        ]
        , board);
}