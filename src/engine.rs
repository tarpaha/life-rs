use ::board::Board;
use ::board::Coord;


pub struct Engine {
    pub board: Board,
}

impl Engine {

    pub fn new(new_board: Board) -> Self {
        Engine {board: new_board}
    }

    pub fn from_file() {

    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn one_iteration(&mut self) {

        let mut next_gen = Board::new(4, 4);

        for (coords, is_alive) in self.board.into_iter() {

            let col = coords.col;
            let row = coords.row;

            // check game rules against current cell
            let neighbours = self.board.get_vicinity(col, row);

            if is_alive {

                let neighbours_cnt = neighbours.into_iter().filter(|&x| x).count();

                // any live cell with fewer than two live neighbours dies,
                // as if caused by underpopulation.

                // any live cell with more than three live neighbours
                // dies, as if by overpopulation.

                // any live cell with two or three live neighbours
                // lives on to the next generation.
                if neighbours_cnt == 2 || neighbours_cnt == 3 {
                    next_gen.born_at(col, row);
                }

            } else {

                // any dead cell with exactly three live neighbours becomes
                // a live cell, as if by reproduction.
                if neighbours.into_iter().filter(|&x| x).count() == 3 {
                    next_gen.born_at(col, row);
                }

            }

        }

        self.board = next_gen;

    }

    pub fn iterations(&mut self, n: u64) {
        for _ in 0..n {
            self.one_iteration();
        }
    }

}
