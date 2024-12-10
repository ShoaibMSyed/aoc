use std::collections::HashSet;

use anyhow::Result;

use crate::util::{CellIndex, Grid, IntoGrid};

fn generate_path(grid: &Grid, start: CellIndex) -> (HashSet<CellIndex>, usize) {
    let mut path = HashSet::new();

    let mut stack = vec![(start, '0')];

    let mut nine_reached = 0;

    while let Some((cur, search_val)) = stack.pop() {
        let Some(&cur_val) = grid.get(cur)
        else { continue };

        if cur_val != search_val {
            continue;
        }

        if search_val == '9' {
            nine_reached += 1;
        }

        path.insert(cur);

        for (adj, _) in grid.cardinal(cur) {
            stack.push((adj, (search_val as u8 + 1) as char));
        }
    }

    (path, nine_reached)
}

pub fn part1(input: String) -> Result<()> {
    let grid = input.grid()?;

    let zeros = grid
        .iter()
        .filter(|(_, cell)| **cell == '0')
        .map(|(i, _)| i);

    let mut sum = 0;
    
    for zero in zeros {
        let (path, _) = generate_path(&grid, zero);

        let nines = path
            .iter()
            .map(|i| grid.get(*i).unwrap())
            .filter(|c| **c == '9')
            .count();
        sum += nines;
    }

    println!("{sum}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let grid = input.grid()?;

    let zeros = grid
        .iter()
        .filter(|(_, cell)| **cell == '0')
        .map(|(i, _)| i);

    let mut sum = 0;
    
    for zero in zeros {
        let (_, reached) = generate_path(&grid, zero);

        sum += reached;
    }

    println!("{sum}");

    Ok(())
}
