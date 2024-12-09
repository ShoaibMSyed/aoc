use anyhow::Result;

use crate::util::IntoReader;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum Entry {
        Empty,
        File(u32),
    }

pub fn part1(input: String) -> Result<()> {
    let mut r = input.reader();

    let mut entries = Vec::new();

    let mut i = 0;
    let mut file_id = 0;

    r.while_ok(|r| {
        let num = r.digit()?;

        let entry = if i % 2 == 0 { file_id += 1; Entry::File(file_id - 1) } else { Entry::Empty };

        for _ in 0..num {
            entries.push(entry);
        }

        i += 1;

        Ok(())
    });

    let mut i = 0;
    let mut j = entries.len() - 1;

    loop {
        while i < j {
            if entries[i] != Entry::Empty {
                i += 1;
            } else {
                break;
            }
        }

        while i < j {
            if entries[j] == Entry::Empty {
                j -= 1;
            } else {
                break;
            }
        }

        if i >= j {
            break;
        }

        entries.swap(i, j);
    }

    let sum: usize = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| match entry {
            Entry::Empty => 0,
            Entry::File(id) => i * *id as usize,
        })
        .sum();

    println!("{sum}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let mut r = input.reader();

    let mut entries = Vec::new();

    let mut i = 0;
    let mut file_id = 0;
    r.while_ok(|r| {
        let size = r.digit()? as u8;

        let entry = if i % 2 == 0 { file_id += 1; Entry::File(file_id - 1) } else { Entry::Empty };

        entries.push((entry, size));

        i += 1;

        Ok(())
    });

    let mut file_index = entries.len() - 1;
    loop {
        if entries[file_index].0 == Entry::Empty { file_index -= 1; continue; }

        let file = entries[file_index].0;
        let size = entries[file_index].1;

        for i in 0..file_index {
            if entries[i].0 != Entry::Empty || entries[i].1 < size {
                continue;
            }

            let extra_space = entries[i].1 - size;

            entries[i].0 = file;
            entries[file_index].0 = Entry::Empty;
            
            entries[i].1 -= extra_space;
            entries.insert(i + 1, (Entry::Empty, extra_space));
            file_index += 1;
            break;
        }

        if file_index == 0 {
            break;
        }

        file_index -= 1;
    }

    let mut sum = 0;

    let mut i = 0;
    for (entry, size) in entries {
        for _ in 0..size {
            sum += i * match entry {
                Entry::Empty => 0,
                Entry::File(id) => id as usize,
            };
            i += 1;
        }
    }

    println!("{sum}");

    Ok(())
}