#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    ALIVE = 0,
    DEAD =  1,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if *self == Self::ALIVE {
            write!(f, "#")
        }else {
            write!(f, "0")
        }
        
    }
}

struct Board {
    pub width: u32,
    pub height: u32,
    cells: Vec<Cell>,
} 

impl Board {
    fn get(&self, row:u32, column:u32) -> Cell {
        self.cells[(row * self.width + column) as usize].clone()
    }
} 

fn main() {
    let board = Board { width: 50, height: 50, cells: vec![Cell::DEAD; 50*50] };

    for x in 0..board.width {
        for y in 0..board.height {
            print!("{}", board.get(x, y));
        }
        println!();
    }
}
