use anyhow::Result;

use crate::util::IntoReader;

pub fn part1(input: String) -> Result<()> {
    let mut r = input.reader();
    
    let lines = r.lines(|r| Ok((r.unsigned()?, r.unsigned()?)))?;
    let (mut a, mut b): (Vec<usize>, Vec<usize>) = lines.into_iter().collect();
    a.sort();
    b.sort();

    let sum: usize = a
        .into_iter()
        .zip(b)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("{sum}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let mut r = input.reader();
    
    let lines = r.lines(|r| Ok((r.unsigned()?, r.unsigned()?)))?;
    let (a, b): (Vec<usize>, Vec<usize>) = lines.into_iter().collect();

    let mut score = 0;

    for num in a {
        let occurences = b.iter().filter(|n| **n == num).count();

        score += num * occurences;
    }

    println!("{score}");

    Ok(())
}