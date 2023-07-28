

trait Collapsable<T> {
    fn try_collapse(&self) -> Option<T>;
}

#[derive(Copy, Clone, Default, Debug)]
struct Sp {
    pub states: [bool; 9],
}

impl Sp {
    pub fn display(&self) -> String {
        let trues: Vec<usize> = self.states
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if *b {Some(i+1)} else {None})
            .collect();
        if trues.len() == 9 {
            return format!("any");
        } else {
            return trues
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
    }
}

impl Collapsable<u8> for Sp {
    fn try_collapse(&self) -> Option<u8>{
        let mut trues_cnt = 0;
        let mut val = 0;
        for i in 0..self.states.len() {
            if self.states[i] {
                trues_cnt+=1;
                val = i;
            }
        }
        if trues_cnt == 1 {
            return Some(val as u8);
        } else {
            return None;
        }
    }
}

type Grid = [[u8; 9]; 9];

type GridSp = [[Sp; 9]; 9];

trait Printable {
    fn print(&self);
}

impl Printable for GridSp {
    fn print(&self) {
        println!();
        for i in 0..9 {
            println!("{:?}", self[i].map(|sp| sp.display()).join(" , "));
        }
    }
}

fn get_square<T>(arr: &[[T; 9]; 9], row: usize, col: usize) -> [T; 8] 
    where T: Copy + Default
{
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    let mut output = [T::default(); 8];

    let mut ctr = 0;
    for i in 0..3 {
        for j in 0..3 {
            if (start_row+j) != row || (start_col+i) != col {
                output[ctr] = arr[start_row + i][start_col + j];
                ctr+=1;
            }
        }
    }

    return output;
}

fn get_row<T>(arr: &[[T; 9]; 9], row: usize, col: usize) -> [T; 8] 
    where T: Copy + Default
{
    let mut output = [T::default(); 8];
    let mut ctr = 0;
    for i in 0..9 {
        if col != i {
            output[ctr] = arr[row][i];
            ctr+=1;
        }
    }

    return output;
}

fn get_col<T>(arr: &[[T; 9]; 9], row: usize, col: usize) -> [T; 8]
    where T: Copy + Default
{
    let mut output = [T::default(); 8];

    let mut ctr = 0;
    for i in 0..9 {
        if row != i {
            output[ctr] = arr[i][col];
            ctr+=1;
        }
    }

    return output;
}

fn build_sp(grid: &Grid) -> GridSp {
    let mut grid_sp = [[Sp::default(); 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j] == 0 {
                grid_sp[i][j] = Sp{states: [true; 9]};
            } else {
                grid_sp[i][j].states = [false; 9];
                grid_sp[i][j].states[grid[i][j] as usize - 1] = true;
            }
        }
    }

    return grid_sp;
}

fn reduce_sp(sp: &GridSp) -> GridSp {
    let mut output = sp.clone();
    for i in 0..9 {
        for j in 0..9 {
            let row = get_row(&output, i, j);
            for val in row {
                match val.try_collapse() {
                    Some(x) => output[i][j].states[x as usize] = false,
                    _ => ()
                }
            }
            let col = get_col(&output, i, j);
            for val in col {
                match val.try_collapse() {
                    Some(x) => output[i][j].states[x as usize] = false,
                    None => (),
                }
            }
            let square = get_square(&output, i, j);
            for val in square {
                match val.try_collapse() {
                    Some(x) => output[i][j].states[x as usize] = false,
                    None => (),
                }
            }
        }
    }

    return output;
}

fn solve(initial_grid: &mut Grid) {
    let sp = build_sp(&initial_grid);
    sp.print();
    let mut new_sp = sp.clone();

    for i in 0..10 {
        new_sp = reduce_sp(&new_sp);
    }

    new_sp.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRID: [[u8; 9]; 9] = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [10,11,12,13,14,15,16,17,18],
        [19,20,21,22,23,24,25,26,27],
        [28,29,30,31,32,33,34,35,36],
        [37,38,39,40,41,42,43,44,45],
        [46,47,48,49,50,51,52,53,54],
        [55,56,57,58,59,60,61,62,63],
        [64,65,66,67,68,69,70,71,72],
        [73,74,75,76,77,78,79,80,81],
    ];

    const TEST_SUDOKU: [[u8; 9]; 9] = [
        [0, 6, 0, 0, 2, 7, 4, 0, 0],
        [0, 0, 0, 8, 0, 4, 0, 0, 0],
        [4, 0, 0, 3, 9, 0, 0, 7, 0],
        [8, 7, 9, 0, 6, 0, 0, 0, 3],
        [5, 0, 0, 0, 0, 8, 0, 0, 7],
        [0, 2, 6, 7, 0, 5, 0, 4, 0],
        [0, 0, 1, 5, 0, 0, 7, 0, 0],
        [0, 0, 3, 0, 7, 0, 8, 9, 4],
        [0, 8, 0, 2, 0, 3, 0, 6, 1],
    ];

    #[test]
    fn test_get_row() {
        assert_eq!(super::get_row(&TEST_GRID, 0, 0), [2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_get_col() {
        assert_eq!(super::get_col(&TEST_GRID, 0, 0), [10, 19, 28, 37, 46, 55, 64, 73]);
    }

    #[test]
    fn test_get_square() {
        assert_eq!(super::get_square(&TEST_GRID, 5, 5), [31, 32, 33, 40, 41, 42, 49, 50]);
    }

    #[test]
    fn test_build_grid_sp() {
        let grid_sp = super::build_sp(&TEST_SUDOKU);
        grid_sp.print();
    }

    #[test]
    fn test_solve() {
        let mut initial_grid = TEST_SUDOKU.clone();
        super::solve(&mut initial_grid);
    }

}

fn main() {
    println!("Hello, world!");
}
