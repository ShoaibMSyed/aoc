use std::collections::HashSet;

use anyhow::Result;

use crate::util::{CellIndex, IntoGrid};

pub fn part1(input: String) -> Result<()> {
    let mut freqs = Vec::new();
    freqs.extend('a'..='z');
    freqs.extend('A'..='Z');
    freqs.extend('0'..='9');

    let mut antennas = HashSet::new();

    let grid = input.grid()?;

    for freq in freqs {
        let freq_locs: Vec<CellIndex> = grid
            .iter()
            .filter(|(_, ch)| **ch == freq)
            .map(|(i, _)| i)
            .collect();

        for i in 0..freq_locs.len() {
            for j in (i + 1)..freq_locs.len() {
                let a = freq_locs[i];
                let b = freq_locs[j];

                let diff = b - a;
                
                let ant1 = b + diff;
                let ant2 = a - diff;

                if grid.contains(ant1) {
                    antennas.insert(ant1);
                }

                if grid.contains(ant2) {
                    antennas.insert(ant2);
                }
            }
        }
    }

    println!("{}", antennas.len());

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let mut freqs = Vec::new();
    freqs.extend('a'..='z');
    freqs.extend('A'..='Z');
    freqs.extend('0'..='9');

    let mut antennas = HashSet::new();

    let grid = input.grid()?;

    for freq in freqs {
        let freq_locs: Vec<CellIndex> = grid
            .iter()
            .filter(|(_, ch)| **ch == freq)
            .map(|(i, _)| i)
            .collect();

        for i in 0..freq_locs.len() {
            for j in (i + 1)..freq_locs.len() {
                let a = freq_locs[i];
                let b = freq_locs[j];

                antennas.insert(a);
                antennas.insert(b);

                let diff = b - a;
                
                let mut start = b + diff;

                while grid.contains(start) {
                    antennas.insert(start);
                    start += diff;
                }

                start = a - diff;

                while grid.contains(start) {
                    antennas.insert(start);
                    start -= diff;
                }
            }
        }
    }

    println!("{}", antennas.len());

    Ok(())
}