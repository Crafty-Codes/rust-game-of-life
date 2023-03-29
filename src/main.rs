use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut gameboard = vec![vec![false; 20]; 20];

    gameboard[0][0] = true;
    gameboard[2][0] = true;
    gameboard[2][1] = true;
    gameboard[1][1] = true;
    gameboard[1][2] = true;

    loop {
        print_field(&gameboard);
        gameboard = ruler(gameboard);

        sleep(Duration::from_millis(1000));
    }
}

fn print_field(board: &Vec<Vec<bool>>) {
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if board[x][y] {
                print!("▮");
            } else {
                print!("▯")
            }
        }
        println!()
    }
    println!()
}

fn ruler(board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut tmpboard = vec![vec![false; board.len()]; board[0].len()];

    for x in 0..board.len() {
        for y in 0..board[0].len() {
            let mut neighbors = 0;

            let mut left = false;
            let mut right = false;
            let mut top = false;
            let mut bottom = false;

            if x > 0 {
                top = true;
            }
            if x < board[0].len() - 1 {
                bottom = true;
            }
            if y > 0 {
                left = true;
            }
            if y < board.len() - 1 {
                right = true;
            }

            if bottom && left && board[x + 1][y - 1] {
                neighbors += 1;
            }
            if left && board[x][y - 1] {
                neighbors += 1;
            }
            if top && left && board[x - 1][y - 1] {
                neighbors += 1;
            }

            if bottom && right && board[x + 1][y + 1] {
                neighbors += 1;
            }
            if right && board[x][y + 1] {
                neighbors += 1;
            }
            if top && right && board[x - 1][y + 1] {
                neighbors += 1;
            }

            if bottom && board[x + 1][y] {
                neighbors += 1;
            }
            if top && board[x - 1][y] {
                neighbors += 1;
            }

            if neighbors == 3 {
                tmpboard[x][y] = true;
            } else if board[x][y] && neighbors == 2 {
                tmpboard[x][y] = true;
            }
        }
    }

    tmpboard
}
