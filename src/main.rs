use std::collections::{HashSet, VecDeque};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    cells: Vec<Vec<i32>>,
    parent: Option<Box<Board>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for (i, vec) in self.cells.iter().enumerate() {
            for (j, e) in vec.iter().enumerate() {
                if j == vec.len() - 1 {
                    write!(f, "{}", e)?;
                    break;
                }
                write!(f, "{}  -  ", e)?;
            }
            if i == self.cells.len() - 1 {
                write!(f, "\n")?;
                break;
            }
            write!(f, "\n\n")?;
        }
        Ok(())
    }
}

impl Board {
    fn new(cells: Vec<Vec<i32>>) -> Board {
        Board {
            cells,
            parent: None,
        }
    }

    fn with_parent(cells: Vec<Vec<i32>>, parent: &Board) -> Board {
        Board {
            cells,
            parent: Some(Box::new(parent.clone())),
        }
    }
}

fn main() {
    let board = Board::new(vec![vec![1, 2, 3], vec![4, 5, 0]]);
    println!("no.1: {}", sliding_puzzle(board));

    let board = Board::new(vec![vec![1, 2, 3], vec![4, 0, 5]]);
    println!("no.2: {}", sliding_puzzle(board));

    // not solvable
    let board = Board::new(vec![vec![1, 2, 3], vec![5, 4, 0]]);
    println!("no.3: {}", sliding_puzzle(board));

    let board = Board::new(vec![vec![4, 1, 2], vec![5, 0, 3]]);
    println!("no.4: {}", sliding_puzzle(board));

    // 22 moves like the online solver too
    let custom_board = Board::new(vec![vec![1, 8, 3], vec![6, 4, 7], vec![5, 2, 0]]);
    println!("no.5: {}", sliding_puzzle(custom_board));
}

// take m and n
// make a random board
// solve it and store the result in a vec,
// that will be achieved by making a board struct that will have the parent board
//
// add arrows to display where the last state was, using wher was the the zero or the di, dj
// impl display for board to pretty print
pub fn sliding_puzzle(start: Board) -> i32 {
    let (m, n) = (start.cells.len(), start.cells[0].len());
    // let goal = Board::new(
    //     (1..(m * n - 1))
    //         .chain(std::iter::once(0))
    //         .collect::<Vec<_>>()
    //         .chunks(n)
    //         .map(|chunk| chunk.to_vec())
    //         .collect(),
    // );
    let mut goal = vec![vec![0; n]; m];
    let mut multiplier = 1;
    for i in 0..m {
        for j in 0..n {
            if i == m - 1 && j == n - 1 {
                break;
            }
            //println!("goal={:?}", goal);
            goal[i][j] = (i + j + multiplier) as i32;
        }
        multiplier += 2;
    }
    //println!("goal={:?}", goal);

    let goal = Board::new(goal);

    if start == goal {
        return 0;
    }

    let mut q = VecDeque::new();
    q.push_back(start.clone());

    let mut visited = HashSet::new();
    visited.insert(start.cells.clone());
    let mut lvl = 0;

    // while let Some(cur) = q.pop_front() {
    while !q.is_empty() {
        let size = q.len();
        lvl += 1;
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            //println!("lvl={:?}", lvl);
            for neighbor in get_neighbors(&cur) {
                //println!("neighbor={:?}", neighbor);
                if neighbor.cells == goal.cells {
                    print_path(&neighbor);
                    return lvl;
                }

                if visited.insert(neighbor.cells.clone()) {
                    q.push_back(neighbor);
                }
            }
        }
    }

    -1
}

fn get_neighbors(board: &Board) -> Vec<Board> {
    //println!("hello");
    let (m, n) = (board.cells.len(), board.cells[0].len());
    let mut v = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if board.cells[i][j] == 0 {
                for &(di, dj) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let ni = (i as i32 + di) as usize;
                    let nj = (j as i32 + dj) as usize;

                    if ni < m && nj < n {
                        let mut new_board = Board::with_parent(board.cells.clone(), board);
                        new_board.cells[i][j] = new_board.cells[ni][nj];
                        new_board.cells[ni][nj] = 0;

                        v.push(new_board);
                    }
                }
            }
        }
    }

    v
}

fn print_path(mut board: &Board) {
    let mut path = Vec::new();

    while let Some(parent) = &board.parent {
        path.push(board.clone());
        board = parent;
    }

    path.push(board.clone());

    for (index, step) in path.iter().rev().enumerate() {
        println!("Step {}:\n{}", index, step);
    }
}
