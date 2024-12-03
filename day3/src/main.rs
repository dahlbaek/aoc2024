const PUZZLE: &str = include_str!("puzzle");

#[derive(Debug)]
struct Mul {
    l: u64,
    r: u64,
}

enum Instruction {
    Mul(Mul),
    Do,
    Dont,
}

fn parse_int(s: &str) -> Result<(u64, &str), ()> {
    let len = s.bytes().take_while(|b| b.is_ascii_digit()).count();
    let i = s[..len].parse::<u64>().map_err(|_| ())?;
    Ok((i, &s[len..]))
}

fn assert_byte(s: &str, b: u8) -> Result<&str, ()> {
    let head = s.as_bytes().get(0).ok_or(())?;
    if *head == b {
        Ok(&s[1..])
    } else {
        Err(())
    }
}

fn parse_mul(s: &str) -> Result<(Mul, &str), ()> {
    let (l, s) = parse_int(s)?;
    let s = assert_byte(s, b',')?;
    let (r, s) = parse_int(s)?;
    let s = assert_byte(s, b')')?;
    Ok((Mul { l, r }, s))
}

fn next_part1(s: &str) -> Result<Option<(Mul, &str)>, &str> {
    match s.find("mul(").map(|idx| idx + 4) {
        None => Ok(None),
        Some(idx) => {
            let s = &s[idx..];
            let (mul, s) = parse_mul(s).or(Err(s))?;
            Ok(Some((mul, s)))
        }
    }
}

fn next_part2(s: &str) -> Result<Option<(Instruction, &str)>, &str> {
    let mul_idx = s.find("mul(");
    let dont_idx = s.find("don't()");
    let do_idx = s.find("do()");
    match vec![mul_idx, do_idx, dont_idx].into_iter().flatten().min() {
        None => Ok(None),
        Some(min_idx) => {
            if mul_idx.is_some_and(|idx| idx == min_idx) {
                let (mul, s) = parse_mul(&s[mul_idx.unwrap() + 4..]).or(Err(&s[1..]))?;
                Ok(Some((Instruction::Mul(mul), s)))
            } else if dont_idx.is_some_and(|idx| idx == min_idx) {
                Ok(Some((Instruction::Dont, &s[dont_idx.unwrap() + 7..])))
            } else if do_idx.is_some_and(|idx| idx == min_idx) {
                Ok(Some((Instruction::Do, &s[do_idx.unwrap() + 4..])))
            } else {
                Ok(None)
            }
        }
    }
}

fn main() {
    let mut agg = 0;
    let mut s = PUZZLE;
    loop {
        match next_part1(s) {
            Ok(Some((m, n))) => {
                s = n;
                agg += m.l * m.r;
            }
            Ok(None) => break,
            Err(n) => s = n,
        }
    }

    println!("Part 1: {}", agg);

    agg = 0;
    s = PUZZLE;
    let mut enabled = true;
    loop {
        s = match next_part2(s) {
            Ok(Some((Instruction::Mul(m), n))) => {
                if enabled {
                    agg += m.l * m.r;
                }
                n
            }
            Ok(Some((Instruction::Do, n))) => {
                enabled = true;
                n
            }
            Ok(Some((Instruction::Dont, n))) => {
                enabled = false;
                n
            }
            Ok(None) => break,
            Err(n) => n,
        }
    }

    println!("Part 2: {}", agg);
}
