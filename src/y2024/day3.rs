use anyhow::Result;

use crate::util::{Either, IntoReader, Reader};

fn mul(r: &mut Reader) -> Result<(usize, usize)> {
    r.text("mul(")?;
    let a = r.unsigned()?;
    r.text(",")?;
    let b = r.unsigned()?;
    r.text(")")?;
    Ok((a, b))
}

pub fn part1(input: String) -> Result<()> {
    let mut sum = 0;

    for (a, b) in input.reader().keep_whitespace().get_matches(mul) {
        sum += a * b;
    }

    println!("{sum}");

    Ok(())
}

pub fn part2(input: String) -> Result<()> {
    let mut enabled = true;
    let mut sum = 0;

    let mut r = input.reader().keep_whitespace();

    while !r.is_empty() {
        let alt: for<'a, 'b> fn(&'a mut Reader<'b>) -> Result<&'b str> = match enabled {
            true => |r| r.text("don't()"),
            false => |r| r.text("do()"),
        };

        let Ok(either) = r.race(mul, alt)
        else { break };

        match either {
            Either::Left((a, b)) => {
                if enabled {
                    sum += a * b;
                }
            }
            Either::Right(_) => enabled = !enabled,
        }
    }

    println!("{sum}");

    Ok(())
}