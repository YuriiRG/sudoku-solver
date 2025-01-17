use crate::*;

#[test]
fn easy_puzzle() {
    let mut board = array_to_board([
        [2, 9, 0, 0, 7, 1, 0, 0, 0],
        [0, 8, 0, 3, 0, 9, 0, 0, 6],
        [0, 4, 0, 0, 0, 0, 0, 0, 0],
        [9, 0, 7, 0, 8, 0, 2, 0, 4],
        [0, 0, 0, 9, 0, 0, 6, 0, 0],
        [0, 0, 8, 0, 2, 0, 9, 1, 3],
        [0, 2, 9, 7, 0, 4, 0, 3, 8],
        [8, 0, 5, 1, 0, 0, 0, 7, 9],
        [0, 7, 4, 0, 9, 0, 1, 6, 2],
    ]);

    board.solve().unwrap();

    let result = board_to_array(board);
    let expected = [
        [2, 9, 6, 8, 7, 1, 3, 4, 5],
        [5, 8, 1, 3, 4, 9, 7, 2, 6],
        [7, 4, 3, 2, 5, 6, 8, 9, 1],
        [9, 1, 7, 6, 8, 3, 2, 5, 4],
        [4, 3, 2, 9, 1, 5, 6, 8, 7],
        [6, 5, 8, 4, 2, 7, 9, 1, 3],
        [1, 2, 9, 7, 6, 4, 5, 3, 8],
        [8, 6, 5, 1, 3, 2, 4, 7, 9],
        [3, 7, 4, 5, 9, 8, 1, 6, 2],
    ];
    assert_eq!(result, expected);
}

#[test]
fn difficult_puzzle() {
    let mut board = array_to_board([
        [0, 0, 9, 0, 4, 7, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 6],
        [0, 8, 0, 0, 2, 0, 0, 0, 0],
        [8, 0, 1, 0, 0, 3, 0, 0, 0],
        [0, 7, 3, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 5, 4],
        [0, 0, 0, 2, 0, 0, 0, 0, 1],
        [3, 0, 0, 0, 0, 9, 0, 7, 0],
        [0, 9, 0, 8, 0, 6, 0, 4, 0],
    ]);

    board.solve().unwrap();

    let result = board_to_array(board);
    let expected = [
        [2, 1, 9, 6, 4, 7, 5, 8, 3],
        [7, 3, 4, 9, 8, 5, 1, 2, 6],
        [6, 8, 5, 3, 2, 1, 4, 9, 7],
        [8, 5, 1, 4, 9, 3, 7, 6, 2],
        [4, 7, 3, 5, 6, 2, 8, 1, 9],
        [9, 2, 6, 7, 1, 8, 3, 5, 4],
        [5, 6, 8, 2, 7, 4, 9, 3, 1],
        [3, 4, 2, 1, 5, 9, 6, 7, 8],
        [1, 9, 7, 8, 3, 6, 2, 4, 5],
    ];
    assert_eq!(result, expected);
}

#[test]
fn bruteforce_pathological() {
    let mut board = array_to_board([
        [9, 0, 0, 8, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 5, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 1, 0, 0, 0, 3],
        [0, 1, 0, 0, 0, 0, 0, 6, 0],
        [0, 0, 0, 4, 0, 0, 0, 7, 0],
        [7, 0, 8, 6, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 3, 0, 1, 0, 0],
        [4, 0, 0, 0, 0, 0, 2, 0, 0],
    ]);
    board.solve().unwrap();

    let result = board_to_array(board);
    let expected = [
        [9, 7, 2, 8, 5, 3, 6, 1, 4],
        [1, 4, 6, 2, 7, 9, 5, 3, 8],
        [5, 8, 3, 1, 4, 6, 7, 2, 9],
        [6, 2, 4, 7, 1, 8, 9, 5, 3],
        [8, 1, 7, 3, 9, 5, 4, 6, 2],
        [3, 5, 9, 4, 6, 2, 8, 7, 1],
        [7, 9, 8, 6, 2, 1, 3, 4, 5],
        [2, 6, 5, 9, 3, 4, 1, 8, 7],
        [4, 3, 1, 5, 8, 7, 2, 9, 6],
    ];
    assert_eq!(result, expected);
}

#[test]
fn invalid_puzzle() {
    let mut board = array_to_board([
        [9, 0, 0, 8, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 5, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 2, 0, 0, 0, 3],
        [0, 1, 0, 0, 0, 0, 0, 6, 0],
        [0, 0, 0, 4, 0, 0, 0, 7, 0],
        [7, 0, 8, 6, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 3, 0, 1, 0, 0],
        [4, 0, 0, 0, 0, 0, 2, 0, 0],
    ]);
    board.solve().unwrap_err();
}
fn array_to_board(array: [[u8; 9]; 9]) -> Board {
    Board {
        values: array.map(|row| row.map(|n| if n != 0 { Some(n) } else { None })),
    }
}

fn board_to_array(board: Board) -> [[u8; 9]; 9] {
    board.values.map(|row| row.map(|cell| cell.unwrap_or(0)))
}
