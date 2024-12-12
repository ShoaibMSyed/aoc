use std::collections::HashSet;

use anyhow::Result;

use crate::util::{CellIndex, Grid, IntoGrid};

fn fill(grid: &Grid, from: CellIndex, filter: char) -> HashSet<CellIndex> {
    let mut found = HashSet::new();
    let mut searched = HashSet::new();

    let mut stack = vec![from];

    while let Some(cur) = stack.pop() {
        searched.insert(cur);
        let val = grid.get(cur).unwrap();

        if *val != filter {
            continue;
        }

        found.insert(cur);

        for (adj, _) in grid.cardinal(cur) {
            if searched.contains(&adj) { continue; }
            stack.push(adj);
        }
    }

    found
}

fn perimeter(plot: &HashSet<CellIndex>) -> usize {
    plot
        .iter()
        .map(|i| {
            i
                .cardinal()
                .filter(|i| !plot.contains(i))
                .count()
        })
        .sum()
}

fn sides(plot: &HashSet<CellIndex>, grid: &Grid) -> usize {
    const ABOVE: CellIndex = CellIndex { x: 0, y: -1 };
    const BELOW: CellIndex = CellIndex { x: 0, y: 1 };
    const LEFT: CellIndex = CellIndex { x: -1, y: 0 };
    const RIGHT: CellIndex = CellIndex { x: 1, y: 0 };

    let mut total = 0;

    for y in 0..grid.height() {
        let mut fence_above = false;
        let mut fence_below = false;

        for x in 0..grid.width() {
            let cur = CellIndex { x, y };

            if !plot.contains(&cur) {
                if fence_above { total += 1; fence_above = false;}
                if fence_below { total += 1; fence_below = false;}
                continue;
            }

            if !plot.contains(&(cur + ABOVE)) {
                if !fence_above { fence_above = true; }
            } else {
                if fence_above { total += 1; fence_above = false;}
            }

            if !plot.contains(&(cur + BELOW)) {
                if !fence_below { fence_below = true; }
            } else {
                if fence_below { total += 1; fence_below = false;}
            }
        }

        if fence_above {
            total += 1;
        }

        if fence_below {
            total += 1;
        }
    }

    for x in 0..grid.width() {
        let mut fence_left = false;
        let mut fence_right = false;

        for y in 0..grid.height() {
            let cur = CellIndex { x, y };
            
            if !plot.contains(&cur) {
                if fence_left { total += 1; fence_left = false;}
                if fence_right { total += 1; fence_right = false;}
                continue;
            }

            if !plot.contains(&(cur + LEFT)) {
                if !fence_left { fence_left = true; }
            } else {
                if fence_left { total += 1; fence_left = false;}
            }

            if !plot.contains(&(cur + RIGHT)) {
                if !fence_right { fence_right = true; }
            } else {
                if fence_right { total += 1; fence_right = false;}
            }
        }

        if fence_left {
            total += 1;
        }

        if fence_right {
            total += 1;
        }
    }

    total
}

fn area(plot: &HashSet<CellIndex>) -> usize { plot.len() }

pub fn part1(input: String) -> Result<()> {
    let grid = input.grid()?;

    let mut plots = Vec::<HashSet<CellIndex>>::new();

    for (i, ch) in &grid {
        if plots.iter().any(|h| h.contains(&i)) {
            continue;
        }
        
        let plot = fill(&grid, i, *ch);

        plots.push(plot);
    }

    let total: usize = plots
        .iter()
        .map(|plot| area(plot) * perimeter(plot))
        .sum();

    println!("{total}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let grid = input.grid()?;

    let mut plots = Vec::<HashSet<CellIndex>>::new();

    for (i, ch) in &grid {
        if plots.iter().any(|h| h.contains(&i)) {
            continue;
        }
        
        let plot = fill(&grid, i, *ch);

        plots.push(plot);
    }

    let total: usize = plots
        .iter()
        .map(|plot| area(plot) * sides(plot, &grid))
        .sum();

    println!("{total}");

    Ok(())
}