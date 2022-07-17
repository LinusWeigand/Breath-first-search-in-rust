fn main() {
    println!("Hello, world!");
    let mut grid = Grid::new();
    println!("{}", grid.get(0, 0));
    //create Walls
    let mut walls = Vec::from(vec![
        Wall::new(0, 1, Alignment::Horizontal),
        Wall::new(2, 1, Alignment::Horizontal),
        Wall::new(4, 1, Alignment::Horizontal),
        Wall::new(5, 2, Alignment::Horizontal),
        Wall::new(4, 2, Alignment::Vertical),
        Wall::new(0, 5, Alignment::Vertical),
    ]);


}
//A breath first search on a 7 x 7 grid
//7 x 7 grid
struct Grid {
    grid: [[(i32, i32); 7]; 7],
}
impl Grid {
    fn new() -> Grid {
        Grid {
            grid: [[(0, 0); 7]; 7],
        }
    }
    fn get(&self, x: i32, y: i32) -> i32 {
        self.grid[x as usize][y as usize].0
    }
    fn set(&mut self, x: i32, y: i32, value: i32) {
        self.grid[x as usize][y as usize].0 = value;
    }
}

struct Wall {
    x: i32,
    y: i32,
    a: Alignment,
}
impl Wall {
    fn new(x: i32, y: i32, a: Alignment) -> Wall {
        Wall {
            x,
            y,
            a,
        }
    }
}

enum Alignment {
    Horizontal,
    Vertical
}


