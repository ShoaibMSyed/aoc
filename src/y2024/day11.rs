use anyhow::Result;
use cached::proc_macro::cached;

use crate::util::IntoReader;

#[cached]
fn blink(stone: u64, times: usize) -> usize {
    if times == 0 {
        return 1;
    }

    if stone == 0 {
        let sum = blink(1, times - 1);
        return sum;
    }

    let num_digits = stone.checked_ilog10().unwrap_or(0) + 1;

    if num_digits % 2 == 0 {
        let stone_str = stone.to_string();
        let (l, r) = stone_str.split_at(stone_str.len() / 2);
        let l: u64 = l.parse().unwrap();
        let r: u64 = r.parse().unwrap();
        let l_sum = blink(l, times - 1);
        let r_sum = blink(r, times - 1);
        let sum = l_sum + r_sum;
        return sum;
    }

    blink(stone * 2024, times - 1)
}

fn run(stones: &[u64], times: usize) -> usize {
    stones
        .iter()
        .map(|stone| blink(*stone, times))
        .sum()
}

pub fn part1(input: String) -> Result<()> {
    let stones = input.reader().while_ok(|r| r.unsigned().map(|u| u as u64));

    let sum = run(&stones, 25);

    println!("{}", sum);

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let stones = input.reader().while_ok(|r| r.unsigned().map(|u| u as u64));

    let sum = run(&stones, 75);

    println!("{}", sum);

    Ok(())
}
