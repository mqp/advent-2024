use std::error::Error;
use std::io;

// convention: bottom left is (0, 0)

struct Grid {
    pub cells: Vec<Vec<char>>,
    pub h: usize,
    pub w: usize,
}

impl Grid {
    fn from_cells(cells: Vec<Vec<char>>) -> Self {
        let h = cells.len();
        let w = if h > 0 { cells[0].len() } else { 0 };
        Self { cells, h, w }
    }
    fn get_unchecked(&self, x: usize, y: usize) -> char {
        self.cells[self.h - y - 1][x]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cells = Vec::<Vec<char>>::new();
    for line in io::stdin().lines() {
        cells.push(line?.chars().collect());
    }
    let grid = Grid::from_cells(cells);
    let mut total: u32 = 0;
    for y in 1..grid.h - 1 {
        for x in 1..grid.w - 1 {
            let center = grid.get_unchecked(x, y);
            let nw = grid.get_unchecked(x - 1, y + 1);
            let ne = grid.get_unchecked(x + 1, y + 1);
            let sw = grid.get_unchecked(x - 1, y - 1);
            let se = grid.get_unchecked(x + 1, y - 1);
            let nw_se_match = (nw == 'M' || nw == 'S') && (se == 'M' || se == 'S') && nw != se;
            let ne_sw_match = (ne == 'M' || ne == 'S') && (sw == 'M' || sw == 'S') && ne != sw;
            if center == 'A' && nw_se_match && ne_sw_match {
                total += 1;
            }
        }
    }

    println!("{}", total);
    Ok(())
}
