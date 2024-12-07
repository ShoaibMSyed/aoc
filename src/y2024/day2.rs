use anyhow::Result;

use crate::util::IntoReader;

pub fn part1(input: String) -> Result<()> {
    let num_safe = input
        .reader()
        .lines(|r| Ok(r.while_ok(|r| r.unsigned())))?
        .into_iter()
        .filter(|nums| is_safe(nums, nums.len()))
        .count();

    println!("{num_safe}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let num_safe = input
        .reader()
        .lines(|r| Ok(r.while_ok(|r| r.unsigned())))?
        .into_iter()
        .filter(|nums| is_safe(nums, nums.len()) || (0..nums.len()).any(|skip| is_safe(nums, skip)))
        .count();

    println!("{num_safe}");

    Ok(())
}

fn is_safe(nums: &[usize], skip: usize) -> bool {
    let mut inc = None;
    let mut last = None;

    let mut safe = true;



    for (i, &num) in nums.iter().enumerate() {
        if i == skip {
            continue;
        }

        if let Some(last) = last {
            let cur_inc = num > last;

            match inc {
                None => inc = Some(cur_inc),
                Some(last_inc) => {
                    if cur_inc != last_inc {
                        safe = false;
                    }
                }
            }

            let diff = usize::abs_diff(num, last);
            if diff == 0 || diff > 3 {
                safe = false;
            }
        }

        last = Some(num);
    }

    safe
}
