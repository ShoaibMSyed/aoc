use anyhow::Result;

use crate::util::{GridMask, IntoGrid};

pub fn part1(input: String) -> Result<()> {
    let masks = [
        "XMAS",
        "SAMX",
        "X\nM\nA\nS",
        "S\nA\nM\nX",
        "X...\n.M..\n..A.\n...S",
        "S...\n.A..\n..M.\n...X",
        "...X\n..M.\n.A..\nS...",
        "...S\n..A.\n.M..\nX...",
    ];
    let masks = masks.map(|mask| GridMask::new_mask(mask, '.').unwrap());

    let grid = input.grid()?;

    let mut sum = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            for mask in &masks {
                if grid.matches_mask_at(mask, [x, y]) {
                    sum += 1;
                }
            }
        }
    }

    println!("{sum}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let masks = [
        "M.S\n.A.\nM.S",
        "M.M\n.A.\nS.S",
        "S.M\n.A.\nS.M",
        "S.S\n.A.\nM.M",
    ];
    let masks = masks.map(|mask| GridMask::new_mask(mask, '.').unwrap());

    let grid = input.grid()?;

    let mut sum = 0;

    for y in 0..grid.height() - 2 {
        for x in 0..grid.width() - 2 {
            for mask in &masks {
                if grid.matches_mask_at(mask, [x, y]) {
                    sum += 1;

                    // Optimization
                    break;
                }
            }
        }
    }

    println!("{sum}");

    Ok(())
}
