extern crate ndarray;

use ndarray::{Array2, Dim};
use rand::prelude::*;
use std::convert::TryFrom;
use std::mem;

const ALIVE: u8 = 1;
const DEAD: u8 = 0;

#[derive(Debug)]
pub struct World {
    pub present: Array2<u8>,
    pub future: Array2<u8>,
    pub rows: usize,
    pub cols: usize,
}

impl World {
    pub fn new(rows: usize, cols: usize) -> Self {
        //self.present = Array2::<u8>::zeros((rows, cols));
        let d = Dim([rows, cols]);
        Self {
            present: Array2::<u8>::zeros(d),
            future: Array2::<u8>::zeros(d),
            rows: rows,
            cols: cols,
        }
    }

    pub fn print(self: &Self) {
        for ri in 0..self.rows {
            for ci in 0..self.cols {
                print!("{}", self.present[[ri, ci]]);
            }
            println!("");
        }
        println!("");
    }

    pub fn populate_random(self: &mut Self) {
        let mut rng = rand::thread_rng();

        for r in 0..self.rows {
            for c in 0..self.cols {
                if rng.gen() {
                    self.present[[r, c]] = ALIVE;
                } else {
                    self.present[[r, c]] = DEAD;
                }
            }
        }
    }

    fn within_world(self: &Self, row: i32, col: i32) -> bool {
        if col >= 0
            && col < i32::try_from(self.cols).unwrap()
            && row >= 0
            && row < i32::try_from(self.rows).unwrap()
        {
            return true;
        } else {
            return false;
        }
    }

    fn count_alive_neighbors(self: &Self, row: usize, col: usize) -> i32 {
        let mut count = 0;
        for ri in -1..2 {
            for ci in -1..2 {
                // everything but the element itself
                if ci == 0 && ri == 0 {
                    continue;
                }
                let col_candidate = ci + i32::try_from(col).unwrap();
                let row_candidate = ri + i32::try_from(row).unwrap();
                if self.within_world(row_candidate, col_candidate) {
                    // only within the present
                    if self.present[[
                        usize::try_from(row_candidate).unwrap(),
                        usize::try_from(col_candidate).unwrap(),
                    ]] == ALIVE
                    {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn step(self: &mut Self) {
        for ci in 0..self.cols {
            for ri in 0..self.rows {
                let neighbors = self.count_alive_neighbors(ri, ci);
                if self.present[[ri, ci]] == ALIVE {
                    match neighbors {
                        0 | 1 => self.future[[ri, ci]] = DEAD,
                        2 | 3 => self.future[[ri, ci]] = ALIVE,
                        _ => self.future[[ri, ci]] = DEAD,
                    }
                } else {
                    match neighbors {
                        3 => self.future[[ri, ci]] = ALIVE,
                        _ => self.future[[ri, ci]] = DEAD,
                    }
                }
            }
        }
        mem::swap(&mut self.future, &mut self.present);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_world() {
        let world = World::new(3, 3);
        let truth = Array2::<u8>::zeros((3, 3));
        assert_eq!(world.present, truth);
    }

    #[test]
    fn test_populate_random() {
        let mut world = World::new(3, 3);
        world.populate_random();
        // this should not panic
    }

    #[test]
    fn test_count_alive_neighbors_none() {
        let world = World::new(3, 3);
        let n = world.count_alive_neighbors(1, 1);
        assert_eq!(n, 0);
    }

    #[test]
    fn test_count_alive_neighbors_all() {
        let mut world = World::new(3, 3);
        for r in 0..world.rows {
            for c in 0..world.cols {
                world.present[[r, c]] = ALIVE;
            }
        }
        let n = world.count_alive_neighbors(1, 1);
        assert_eq!(n, 8);
    }

    #[test]
    fn test_count_alive_neighbors_diagonal_center() {
        let mut world = World::new(3, 3);
        for i in 0..3 {
            world.present[[i, i]] = ALIVE;
        }
        let n = world.count_alive_neighbors(1, 1);
        assert_eq!(n, 2);
    }

    #[test]
    fn test_count_alive_neighbors_diagonal_top_left() {
        let mut world = World::new(3, 3);
        for i in 0..3 {
            world.present[[i, i]] = ALIVE;
        }
        let n = world.count_alive_neighbors(0, 0);
        assert_eq!(n, 1);
    }

    #[test]
    /*
        000             010
        111     ->      010
        000             010
    */
    fn test_bar_3x3() {
        let mut world = World::new(3, 3);
        for i in 0..3 {
            world.present[[1, i]] = ALIVE; // [row,col]
        }
        world.step();
        for i in 0..3 {
            assert_eq!(world.present[[i, 0]], DEAD);
            assert_eq!(world.present[[i, 1]], ALIVE);
            assert_eq!(world.present[[i, 2]], DEAD);
        }
    }

    #[test]
    /*
        0100             0000
        0010             1010
        1110     ->      0110
        0000             0100
    */
    fn test_glider_4x4() {
        let mut world = World::new(4, 4);
        world.present[[0, 1]] = ALIVE; // [row,col]
        world.present[[1, 2]] = ALIVE;
        world.present[[2, 0]] = ALIVE;
        world.present[[2, 1]] = ALIVE;
        world.present[[2, 2]] = ALIVE;
        world.step();
        // first row
        assert_eq!(world.present[[0, 0]], DEAD);
        assert_eq!(world.present[[0, 1]], DEAD);
        assert_eq!(world.present[[0, 2]], DEAD);
        assert_eq!(world.present[[0, 3]], DEAD);
        // second row
        assert_eq!(world.present[[1, 0]], ALIVE);
        assert_eq!(world.present[[1, 1]], DEAD);
        assert_eq!(world.present[[1, 2]], ALIVE);
        assert_eq!(world.present[[1, 3]], DEAD);
        // third row
        assert_eq!(world.present[[2, 0]], DEAD);
        assert_eq!(world.present[[2, 1]], ALIVE);
        assert_eq!(world.present[[2, 2]], ALIVE);
        assert_eq!(world.present[[2, 3]], DEAD);
        // fourth row
        assert_eq!(world.present[[3, 0]], DEAD);
        assert_eq!(world.present[[3, 1]], ALIVE);
        assert_eq!(world.present[[3, 2]], DEAD);
        assert_eq!(world.present[[3, 3]], DEAD);
    }
}
