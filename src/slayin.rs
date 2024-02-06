use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fmt;

use rand::seq::SliceRandom;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    cells: Vec<i32>,
    parent: Option<Box<Board>>,
    zero: usize,
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
        for (i, cell) in self.cells.iter().enumerate() {
            // hardcoded for 3x3 board
            if i == 3 || i == 6 {
                write!(f, "\n")?;
            }
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn find_zero(cells: &Vec<i32>) -> usize {
        for i in 0..cells.len() {
            if cells[i] == 0 {
                return i;
            }
        }

        unreachable!()
    }
    pub fn new(cells: Vec<i32>) -> Board {
        let zero = Self::find_zero(&cells);
        Board {
            cells,
            parent: None,
            zero,
            g_cost: 0,
            h_cost: 0,
        }
    }

    fn with_parent(cells: Vec<i32>, parent: &Board) -> Board {
        let zero = Self::find_zero(&cells);
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

    // two fns to calculate_h_cost
    fn manhatan_dist(&self, goal: &Vec<i32>) -> i32 {
        let mut h_cost = 0;
        for i in 0..self.cells.len() {
            if self.cells[i] != goal[i] && self.cells[i] != 0 {
                let ni = i as i32 / 3;
                let nj = i as i32 % 3;
                let goal_i = (self.cells[i] - 1) / 3;
                let goal_j = (self.cells[i] - 1) % 3;

                h_cost += (nj - goal_j).abs() + (ni - goal_i).abs();
            }
        }
        h_cost
    }
    // doesn't work for now as it accumilates the h_cost
    fn _manhatan_dist_not_first_time(&self, ay_ya_eedy: usize) -> i32 {
        let mut h_cost = self.parent.as_ref().unwrap().h_cost;
        // we use the i no ni in fn get_neighbors
        let ni = ay_ya_eedy as i32 / 3;
        let nj = ay_ya_eedy as i32 % 3;
        let goal_i = (self.cells[ay_ya_eedy] - 1) / 3;
        let goal_j = (self.cells[ay_ya_eedy] - 1) % 3;

        h_cost += (nj - goal_j).abs() + (ni - goal_i).abs();
        h_cost
    }
    fn _hamming_dist(cells: &Vec<i32>, goal: &Vec<i32>) -> i32 {
        let mut h_cost = 0;
        for i in 0..cells.len() {
            if cells[i] != goal[i] && cells[i] != 0 {
                h_cost += 1;
            }
        }
        h_cost
    }

    // I think it works-ish, it gives you a solvable board but the inversion count is wrong
    pub fn is_solvable(cells: &Vec<i32>) -> bool {
        let new_vec = cells.iter().filter(|&x| *x != 0).map(|&x| x).collect();
        // println!("new_vec={:?}", new_vec);
        let inversions_count = Self::merge_sort(&new_vec);
        // println!("inversions_count={:?}", inversions_count);
        if inversions_count % 2 == 0 {
            return true;
        }
        false
    }

    fn merge_sort(vec: &Vec<i32>) -> i32 {
        let mut tmp = vec![-1; 8];
        let mut inversions_count = 0;
        Self::_merge_sort(
            &mut vec.clone(),
            0,
            tmp.len() - 1,
            &mut tmp,
            &mut inversions_count,
        );
        // println!("tmp={:#?}", tmp);
        inversions_count
    }

    fn _merge_sort(
        vec: &mut Vec<i32>,
        st: usize,
        end: usize,
        tmp: &mut Vec<i32>,
        inversions_count: &mut i32,
    ) {
        if st == end {
            return;
        }

        let mid = st + (end - st) / 2;

        Self::_merge_sort(vec, st, mid, tmp, inversions_count);
        Self::_merge_sort(vec, mid + 1, end, tmp, inversions_count);

        Self::_join_sorted_arrays(vec, st, mid, end, tmp, inversions_count);
    }

    fn _join_sorted_arrays(
        vec: &mut Vec<i32>,
        st: usize,
        mid: usize,
        end: usize,
        tmp: &mut Vec<i32>,
        inversions_count: &mut i32,
    ) {
        let mut k = st;
        let mut i = st;
        let mut j = mid + 1;

        while k <= end {
            if i > mid {
                tmp[k] = vec[j];
                j += 1;
            } else if j > end {
                tmp[k] = vec[i];
                i += 1;
            } else if vec[j] > vec[i] {
                tmp[k] = vec[i];
                i += 1;
            } else {
                tmp[k] = vec[j];
                j += 1;
                *inversions_count += mid as i32 - i as i32 + 1;
            }
            k += 1;
        }

        // println!("{:?}", &tmp[st..=end]);
        vec[st..=end].copy_from_slice(&tmp[st..=end]);
    }

    pub fn random_board() -> Board {
        loop {
            let mut rng = rand::thread_rng();
            let mut random_board = (0..9).collect::<Vec<i32>>();
            random_board.shuffle(&mut rng);
            // println!("random_board={:?}", random_board);

            if !Self::is_solvable(&random_board) {
                continue;
            }
            return Board::new(random_board);
        }
    }

    pub fn sliding_puzzle_a_star(start: Board) -> Option<Vec<Vec<i32>>> {
        let goal = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        // println!("goal={:?}", goal);

        if start.cells == goal {
            return Some(Vec::new());
        }

        // start.h_cost = start.manhatan_dist(&goal);

        let mut pq = BinaryHeap::new();
        pq.push(start.clone());

        let mut visited = HashSet::new();
        visited.insert(start.cells);

        let mut count = 0;
        while let Some(cur) = pq.pop() {
            // println!("cur={:?}", cur);
            // println!("pq={:?}", pq);
            for neighbor in cur.get_neighbors(&goal) {
                // println!("{count}");
                // println!("neighbor={:?}", neighbor);
                count += 1;
                // if count >= 5 {
                //     break;
                // }
                if neighbor.cells == goal {
                    // println!("neighbor={:?}", neighbor);
                    Self::print_path(&neighbor);
                    // println!("g_cost == lvl? {}", neighbor.g_cost);
                    // return Some(neighbor.g_cost);
                    return Some(Self::return_path(&neighbor));
                }

                if visited.insert(neighbor.cells.clone()) {
                    pq.push(neighbor);
                }
            }
        }

        // println!("hello");
        None
    }

    fn get_neighbors(&self, goal: &Vec<i32>) -> Vec<Board> {
        let og_i = self.zero;
        let mut v = Vec::new();
        // println!("og_i={:#?}", og_i);

        let i = og_i / 3;
        // println!("i={:#?}", i);
        let j = og_i % 3;
        // println!("j={:#?}", j);

        for &(di, dj) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let ni = i as i32 + di;
            // println!("ni dir={:#?}", ni);
            let nj = j as i32 + dj;
            // println!("nj dir={:#?}", nj);

            if ni < 3 && ni >= 0 && nj < 3 && nj >= 0 {
                let mut new_board = Board::with_parent(self.cells.clone(), self);
                let ni_new = ni * 3 + nj;
                // println!("ni new={:#?}", ni_new);
                // let f = new_board.manhatan_dist_not_first_time(&goal, i);
                let f = new_board.manhatan_dist(&goal);
                // println!("f={:#?}", f);
                new_board.cells[og_i] = new_board.cells[ni_new as usize];
                // println!("new_board={:?}", new_board);
                new_board.cells[ni_new as usize] = 0;
                // println!("new_board={:?}", new_board.cells);
                // println!("new_board={:?}", new_board);
                new_board.zero = ni_new as usize;
                new_board.h_cost = f;
                new_board.g_cost = self.g_cost + 1;

                v.push(new_board);
            }
        }

        // println!("v={:?}", v);
        v
    }
    // return path as a vec for each step
    pub fn return_path(mut board: &Board) -> Vec<Vec<i32>> {
        let mut path = Vec::new();

        while let Some(parent) = &board.parent {
            path.push(board.cells.clone());
            board = parent;
        }

        path.push(board.cells.clone());

        // for (index, step) in path.iter().rev().enumerate() {
        //     println!("h_cost {}", step.h_cost);
        //     println!("g_cost {}", step.g_cost);
        //     println!("Step {}:\n{}", index, step);
        // }
        // println!("path from fn={:?}", path);
        path
    }

    fn print_path(mut board: &Board) {
        let mut path = Vec::new();

        while let Some(parent) = &board.parent {
            path.push(board);
            board = parent;
        }

        path.push(board);

        for (index, step) in path.iter().rev().enumerate() {
            // println!("h_cost {}", step.h_cost);
            // println!("g_cost {}", step.g_cost);
            // println!("Step {}:\n{}", index, step);
        }
    }
}

// fn main() {
//     let start_cells = vec![1, 8, 3, 6, 4, 7, 5, 2, 0];
//     let start_board = Board::new(start_cells.clone());
//     let goal = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
//     // println!("{:?}", Board::get_neighbors(&start_board, &goal));
//     println!("{:?}", Board::sliding_puzzle_a_star(start_board));
//     // let random_board = Board::random_board();
//     // println!("{:?}", Board::sliding_puzzle_a_star(random_board));
// }
