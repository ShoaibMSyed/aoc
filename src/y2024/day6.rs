use std::{collections::{HashMap, HashSet}, time::Instant};

use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::util::{CellIndex, Grid, IntoGrid};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    impl Dir {
        fn rotate_right(&mut self) {
            *self = match *self {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
        }

        fn to_offset(self) -> CellIndex {
            match self {
                Dir::Up => (0isize, -1),
                Dir::Down => (0, 1),
                Dir::Left => (-1, 0),
                Dir::Right => (1, 0),
            }
            .into()
        }

        fn in_front_of(self, index: CellIndex) -> CellIndex {
            index + self.to_offset()
        }
    }

pub fn part1(input: String) -> Result<()> {
    let mut grid = input.grid()?;
    let mut guard = grid.iter().find(|(_, ch)| **ch == '^').unwrap().0;
    let mut dir = Dir::Up;

    'outer: while grid.contains(guard) {
        grid.set(guard, 'X')?;

        let mut next;

        loop {
            next = dir.in_front_of(guard);
            let Some(next_value) = grid.get(next) else {
                break 'outer;
            };

            if *next_value == '#' {
                dir.rotate_right();
            } else {
                break;
            }
        }

        guard = next;
    }

    let count = grid.cells().iter().filter(|ch| **ch == 'X').count();

    println!("{count}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    fn is_infinite_loop(grid: &Grid<char>, extra: CellIndex, mut guard: CellIndex) -> Result<bool> {
        let mut visits: HashMap<CellIndex, HashSet<Dir>> = Default::default();

        let mut dir = Dir::Up;

        'outer: while grid.contains(guard) {
            visits.entry(guard).or_default().insert(dir);
    
            let mut next;

            let mut i = 0;
    
            loop {
                i += 1;
                if i > 5 {
                    return Ok(true);
                }

                next = dir.in_front_of(guard);
                let Some(mut next_value) = grid.get(next) else {
                    break 'outer;
                };

                if next == extra {
                    next_value = &'#';
                }
    
                if *next_value == '#' {
                    dir.rotate_right();
                } else {
                    break;
                }
            }
    
            guard = next;

            if visits.entry(guard).or_default().contains(&dir) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    let grid = input.grid()?;
    let guard = grid.iter().find(|(_, ch)| **ch == '^').unwrap().0;

    let start = Instant::now();

    let indexes = Vec::from_iter(CellIndex::all_indexes_for(&grid));

    let count = indexes.into_par_iter()
        .filter(|cell| cell != &guard)
        .filter(|cell| {
            is_infinite_loop(&grid, *cell, guard).unwrap()
        })
        .count();

    let end = Instant::now();

    println!("{count} in {} seconds", (end - start).as_secs());

    Ok(())
}
