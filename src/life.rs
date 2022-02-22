
pub struct Board
{
    width: usize,
    height: usize,
    pub cells: Vec<Vec<u8>>,
    generation: u64
}

impl Board
{
    pub fn new(custom_width: usize, custom_height: usize) -> Board
    {
        Board
        {
            width: custom_width, //x
            height: custom_height, //y
            cells: vec![vec![0; custom_height]; custom_width],
            generation: 0
        }
    }

    fn get_cell_state(&mut self, cell_x: usize, cell_y: usize) -> u8
    {
        let wrap_x = cell_x % self.width;
        let wrap_y = cell_y % self.height;

        return self.cells[wrap_x][wrap_y];
    }

    fn count_cell_neighbours(&mut self, cell_x: usize, cell_y: usize) -> u8
    {
        let mut neighbour_count: u8 = 0;
        for offset_x in [self.width-1,0,1].iter().cloned()
        {
            for offset_y in [self.height-1,0,1].iter().cloned()
            {
                if offset_x == 0 && offset_y == 0
                {
                    continue;
                }

                neighbour_count += self.get_cell_state(offset_x + cell_x, offset_y + cell_y);
            }
        }
        return neighbour_count;
    }

    pub fn next(&mut self)
    {
        let mut next_cells = self.cells.clone();
        for cell_x in 0..self.width
        {
            for cell_y in 0..self.height
            {
                let current_cell: u8 = self.get_cell_state(cell_x, cell_y);
                let neighbour_count: u8 = self.count_cell_neighbours(cell_x, cell_y);

                let new_cell: u8 = match (current_cell, neighbour_count)
                {
                    (1, alive) if alive < 2 => 0,
                    (1, 2) | (1, 3) => 1,
                    (1, alive) if alive > 3 => 0,
                    (0, 3) => 1,
                    (default, _) => default,
                };

                next_cells[cell_x][cell_y] = new_cell;

            }
        }
        self.cells = next_cells;
        self.generation += 1;
    }

    pub fn display(&mut self)
    {
        println!("Generation {}", self.generation);
        self.cells.iter().for_each(|x|
        {
            println!("{:?}", x);
        })
    }

}