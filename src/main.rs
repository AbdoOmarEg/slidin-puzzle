use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fmt;
use std::time::Instant;

use rand::seq::SliceRandom;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    cells: Vec<Vec<i32>>,
    parent: Option<Box<Board>>,
    zero: (usize, usize),
    g_cost: i32,
    h_cost: i32,
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.g_cost + self.h_cost)
            .cmp(&(other.g_cost + other.h_cost))
            .reverse()
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
        let zero = find_zero(&cells);
        Board {
            cells,
            parent: None,
            zero,
            g_cost: 0,
            h_cost: 0,
        }
    }

    fn with_parent(cells: Vec<Vec<i32>>, parent: &Board, goal: &Vec<Vec<i32>>) -> Board {
        let zero = find_zero(&cells);
        // let h_cost = calculate_h_cost(&cells, goal);
        let h_cost = 0;
        Board {
            cells,
            parent: Some(Box::new(parent.clone())),
            zero,
            g_cost: parent.g_cost + 1,
            h_cost,
        }
    }
}

fn main() {
    // let board = Board::new(vec![vec![1, 2, 3], vec![4, 5, 0]]);
    // println!("no.1: {}", sliding_puzzle(board));
    //
    // let board = Board::new(vec![vec![1, 2, 3], vec![4, 0, 5]]);
    // println!("no.2: {}", sliding_puzzle(board));
    //
    // // not solvable
    // let board = Board::new(vec![vec![1, 2, 3], vec![5, 4, 0]]);
    // println!("no.3: {}", sliding_puzzle(board));
    //
    // let board = Board::new(vec![vec![4, 1, 2], vec![5, 0, 3]]);
    // println!("no.4: {}", sliding_puzzle(board));

    // 22 moves like the online solver too
    let custom_board = Board::new(vec![vec![1, 8, 3], vec![6, 4, 7], vec![5, 2, 0]]);
    println!("no.5: {:?}", sliding_puzzle_bfs(custom_board.clone()));
    let bfs_start_time = Instant::now();
    println!("BFS Result: {:?}", sliding_puzzle_bfs(custom_board));
    let bfs_duration = bfs_start_time.elapsed();
    println!("BFS Duration: {:?}", bfs_duration);

    // let random_board = random_board(3, 3);
    // println!("random_board={:?}", random_board);
    // println!("{:?}", find_zero(&random_board.cells));
}

fn calculate_h_cost(cells: &Vec<Vec<i32>>, goal: &Vec<Vec<i32>>) -> i32 {
    let (m, n) = (cells.len(), cells[0].len());
    let mut h_cost = 0;
    for i in 0..m {
        for j in 0..n {
            if cells[i][j] != goal[i][j] && cells[i][j] != 0 {
                h_cost += 1;
            }
        }
    }
    h_cost
}

fn find_zero(cells: &Vec<Vec<i32>>) -> (usize, usize) {
    let (m, n) = (cells.len(), cells[0].len());

    for i in 0..m {
        for j in 0..n {
            if cells[i][j] == 0 {
                return (i, j);
            }
        }
    }

    unreachable!()
}

// take m and n
// make a random board
// solve it and store the result in a vec,
// that will be achieved by making a board struct that will have the parent board
//
// add arrows to display where the last state was, using wher was the the zero or the di, dj
// impl display for board to pretty print
// add g_score & h_score
// add place of zero instead of searching for it every time

fn random_board(m: usize, n: usize) -> Board {
    loop {
        let mut rng = rand::thread_rng();
        let mut board = (1..(m as i32 * n as i32)).collect::<Vec<i32>>();
        board.push(0);
        board.shuffle(&mut rng);

        let random_board = Board::new(board.chunks(n).map(|chunk| chunk.to_vec()).collect());
        println!("random_board={:?}", random_board);
        if sliding_puzzle_bfs(random_board).is_some() {
            return Board::new(board.chunks(n).map(|chunk| chunk.to_vec()).collect());
        }
    }
}

// fn random_board(m: usize, n: usize) -> Board {
//     loop {
//         let mut rng = rand::thread_rng();
//         let mut board = (1..(m as i32 * n as i32)).collect::<Vec<i32>>();
//         board.push(0);
//         board.shuffle(&mut rng);
//
//         let random_board = Board::new(board.chunks(n).map(|chunk| chunk.to_vec()).collect());
//
//         if is_solvable(&random_board, m, n) {
//             return random_board;
//         }
//     }
// }
//
// fn is_solvable(board: &Board, m: usize, n: usize) -> bool {
//     let flattened_board: Vec<i32> = board
//         .cells
//         .iter()
//         .flat_map(|row| row.iter())
//         .cloned()
//         .collect();
//     let mut inversion_count = 0;
//     println!("flattened_board={:?}", flattened_board);
//
//     for i in 0..(m * n) {
//         for j in i + 1..(m * n) {
//             if flattened_board[i] != 0
//                 && flattened_board[j] != 0
//                 && flattened_board[i] > flattened_board[j]
//             {
//                 inversion_count += 1;
//             }
//         }
//     }
//
//     println!("inversion_count={:?}", inversion_count);
//
//     // For an odd-sized puzzle, the number of inversions must be even for solvability
//     if m * n % 2 == 1 {
//         return inversion_count % 2 == 0;
//     }
//
//     // For an even-sized puzzle, check the parity of the blank tile row from the bottom
//     let blank_row = board.cells.iter().position(|row| row.contains(&0)).unwrap();
//     inversion_count % 2 == (m - blank_row) % 2
// }
pub fn sliding_puzzle_bfs(start: Board) -> Option<i32> {
    let (m, n) = (start.cells.len(), start.cells[0].len());
    let goal = (1..(m as i32 * n as i32))
        .chain(std::iter::once(0))
        .collect::<Vec<_>>()
        .chunks(n)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    if start.cells == goal {
        return Some(0);
    }

    let mut q = VecDeque::new();
    q.push_back(start.clone());

    let mut visited = HashSet::new();
    visited.insert(start.cells.clone());

    while let Some(cur) = q.pop_front() {
        for neighbor in get_neighbors(&cur, &goal) {
            if neighbor.cells == goal {
                print_path(&neighbor);
                println!("g_cost == lvl? {}", neighbor.g_cost);
                return Some(neighbor.g_cost);
            }

            if visited.insert(neighbor.cells.clone()) {
                q.push_back(neighbor);
            }
        }
    }

    None
}

pub fn sliding_puzzle_a_star(start: Board) -> Option<i32> {
    let (m, n) = (start.cells.len(), start.cells[0].len());
    let goal = (1..(m as i32 * n as i32))
        .chain(std::iter::once(0))
        .collect::<Vec<_>>()
        .chunks(n)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    if start.cells == goal {
        return Some(0);
    }

    let mut pq = BinaryHeap::new();
    pq.push(start.clone());

    let mut visited = HashSet::new();
    visited.insert(start.cells.clone());

    while let Some(cur) = pq.pop() {
        for neighbor in get_neighbors(&cur, &goal) {
            if neighbor.cells == goal {
                print_path(&neighbor);
                println!("g_cost == lvl? {}", neighbor.g_cost);
                return Some(neighbor.g_cost);
            }

            if visited.insert(neighbor.cells.clone()) {
                pq.push(neighbor);
            }
        }
    }

    None
}

fn get_neighbors(board: &Board, goal: &Vec<Vec<i32>>) -> Vec<Board> {
    let (m, n) = (board.cells.len(), board.cells[0].len());
    let (i, j) = board.zero;
    let mut v = Vec::new();

    for &(di, dj) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let ni = (i as i32 + di) as usize;
        let nj = (j as i32 + dj) as usize;

        if ni < m && nj < n {
            let mut new_board = Board::with_parent(board.cells.clone(), board, &goal);
            new_board.cells[i][j] = new_board.cells[ni][nj];
            new_board.cells[ni][nj] = 0;
            new_board.zero = (ni, nj);

            v.push(new_board);
        }
    }

    v
}

fn print_path(mut board: &Board) {
    let mut path = Vec::new();

    while let Some(parent) = &board.parent {
        path.push(board);
        board = parent;
    }

    path.push(board);

    for (index, step) in path.iter().rev().enumerate() {
        println!("h_cost {}", step.h_cost);
        println!("g_cost {}", step.g_cost);
        println!("Step {}:\n{}", index, step);
    }
}
