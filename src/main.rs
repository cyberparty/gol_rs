mod life;

fn main() 
{
    let mut board: life::Board = life::Board::new(10, 11);
    board.cells[4][5] = 1;
    board.cells[5][5] = 1;
    board.cells[6][5] = 1;

    for _ in 0..20
    {
        board.display();
        board.next();
    }
}
