fn main() {
    let mut gameboard = vec![vec![false; 5]; 5];

    gameboard[2][2] = true;

    print_field(&gameboard);
}

fn print_field(board: &Vec<Vec<bool>>) {
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if board[x][y] {
                print!("#");
            } else {
                print!("0")
            }
        }
        println!()
    }
    println!()
}
