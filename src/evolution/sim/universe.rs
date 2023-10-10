use ::rand::Rng;

use super::Params;


#[derive(Clone)]
pub struct Cell {
    pub genes: u32,
    pub move_left: bool,
    pub move_right: bool,
    pub move_top: bool,
    pub move_bottom: bool,
}

 #[derive(Clone)]
pub enum Tile {
    Empty,
    Occupied(Cell)
}

pub struct Universe {
    params: Params,
    tiles: Vec<Tile>,
    step: usize,
    generation: usize,
}

impl Cell {
    fn new(genes: u32) -> Self {
        let move_left = genes & 0b0001 > 0;
        let move_right = genes & 0b0010 > 0;
        let move_top = genes & 0b0100 > 0;
        let move_bottom = genes & 0b1000 > 0;
        Self { 
            genes,
            move_left,
            move_right,
            move_top,
            move_bottom,
        }
    }
}

impl Universe {
    pub fn random(params: Params) -> Universe {
        let mut rng = ::rand::thread_rng();

        let mut cells = vec![Tile::Empty; params.width*params.height];
        let distance = params.width*params.height/ params.population - 1;
        for i in 0..params.population {
            let a = i*distance;
            let b = (i+1)*distance;
            let idx = rng.gen_range(a..b);
            cells[idx] = Tile::Occupied(Cell::new(rng.gen()))
        }
        Universe {
            params,
            tiles: cells,
            step: 0,
            generation: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.params.width
    }
    
    pub fn height(&self) -> usize {
        self.params.height
    }
    
    pub fn cell_at(&self, row: usize, col: usize) -> &Tile {
        let idx = self.xy_idx(row, col);
        &self.tiles[idx]
    }

    pub fn tick(&mut self) {
        if self.step < self.params.steps_per_generation {
            self.update_ants();
            self.step += 1;
        } else {
            self.new_generation();
            self.step = 0;
            self.generation += 1;
        }
    }

    fn update_ants(&mut self) {
        let mut next_cells = vec![Tile::Empty; self.params.width*self.params.height];
        
        for row in 0..self.params.height {
            for col in 0..self.params.width {
                let idx = self.xy_idx(row, col);
                if let Tile::Occupied(a) = &self.tiles[idx] {
                    let mut r = row;
                    let mut c = col;
                    if r > 0 && a.move_top { r -= 1}
                    if r < self.params.height-1 && a.move_bottom { r += 1}
                    if c > 0 && a.move_left { c -= 1}
                    if c < self.params.width-1 && a.move_right { c += 1}

                    let new_idx = self.xy_idx(r, c);
                    next_cells[new_idx] = Tile::Occupied(a.clone());
                }
            }
        }
 
        self.tiles = next_cells;
    }

    fn new_generation(&mut self) {
        let mut rng = ::rand::thread_rng();

        let mut cells = vec![Tile::Empty; self.params.width*self.params.height];
        let distance = self.params.width*self.params.height/ self.params.population - 1;
        for i in 0..self.params.population {
            let a = i*distance;
            let b = (i+1)*distance;
            let idx = rng.gen_range(a..b);
            cells[idx] = Tile::Occupied(Cell::new(rng.gen()))
        } 
 
        self.tiles = cells;
    }

    fn xy_idx(&self, row: usize, column: usize) -> usize {
        (row * self.params.width + column) as usize
    }
}
