use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::util::IntoReader;

fn obeys_rule(rule_map: &HashMap<usize, HashSet<usize>>, update: &[usize], i_a: usize) -> Result<(), usize> {
    let a = update[i_a];
    let Some(bs) = rule_map.get(&a)
    else { return Ok(()) };

    for (i_b, b) in update.into_iter().enumerate() {
        if bs.contains(b) {
            if i_b < i_a {
                return Err(i_b);
            }
        }
    }

    Ok(())
}

pub fn part1(input: String) -> Result<()> {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules = rules.reader().lines(|r| {
        let a = r.unsigned()?;
        r.text("|")?;
        let b = r.unsigned()?;
        Ok((a, b))
    })?;

    let mut rule_map = HashMap::new();

    for (a, b) in rules {
        rule_map.entry(a).or_insert(HashSet::new()).insert(b);
    }

    let updates = pages.reader().lines(|r| {
        r.list(",", |r| r.unsigned())
    })?;

    let mut sum = 0;

    for update in updates {
        let mut obeys = true;
        for i in 0..update.len() {
            if obeys_rule(&rule_map, &update, i).is_err() {
                obeys = false;
            }
        }
        if obeys {
            let mid = update.len() / 2;
            sum += update[mid];
        }
    }

    println!("{sum}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules = rules.reader().lines(|r| {
        let a = r.unsigned()?;
        r.text("|")?;
        let b = r.unsigned()?;
        Ok((a, b))
    })?;

    let mut rule_map = HashMap::new();

    for (a, b) in rules {
        rule_map.entry(a).or_insert(HashSet::new()).insert(b);
    }

    let updates = pages.reader().lines(|r| {
        r.list(",", |r| r.unsigned())
    })?;

    let mut sum = 0;

    for mut update in updates {
        let mut erred = false;

        {
            let mut i = 0;
            while i < update.len() {
                if let Err(i_b) = obeys_rule(&rule_map, &update, i) {
                    erred = true;
                    update.swap(i, i_b);
                    i = 0;
                } else {
                    i += 1;
                }
            }
        }

        if erred {
            let mid = update.len() / 2;
            sum += update[mid];
        }
    }

    println!("{sum}");

    Ok(())
}