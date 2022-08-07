use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut state = true;
    for (id,row) in board.iter().enumerate() {
        let x: HashSet<&char> =row.iter().filter(|&x| x != &'x').collect();
        let y: Vec<&char> =row.iter().filter(|&x| x != &'x').collect();
        if x.len() != y.len(){
            state = false;
            break
        }
        let x: HashSet<&char> =board[id][0..9].iter().filter(|&x| x != &'x').collect();
        let y: Vec<&char> =board[id][0..9].iter().filter(|&x| x != &'x').collect();
        if x.len() != y.len(){
            state = false;
            break
        }

    }
    state
}
fn main() {
    let sudoku = vec![
                                     vec!['5','3','x','x','7','x','x','x','x']
                                    ,vec!['6','x','x','1','9','5','x','x','x']
                                    ,vec!['x','9','8','x','x','x','x','6','x']
                                    ,vec!['8','x','x','x','6','x','x','x','3']
                                    ,vec!['4','x','x','8','x','3','x','x','1']
                                    ,vec!['7','x','x','x','2','x','x','x','6']
                                    ,vec!['x','6','x','x','x','x','2','8','x']
                                    ,vec!['x','x','x','4','1','9','x','x','5']
                                    ,vec!['1','x','x','x','8','x','x','7','9']];
    // println!("{:?}", is_valid_sudoku(sudoku));
    // let x: Vec<char> =
    // let y = sudoku[0][0]..sudoku[8][0];
    for (row, row_d) in sudoku.iter().enumerate(){
        for (col, col_d) in row_d.iter().enumerate(){
            print!("{}", sudoku[(col%3)][((row%3)+(col%3))+(row%4)]);
        }
        println!("");
    }
    // println!("column {:?}", y);
    // println!('{:?}', &num[1..]);


    // println!('{:?}', two_sum(num, 6));
}