use std::collections::HashMap;

const PUZZLE: &str = include_str!("puzzle");

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Xor,
    Or,
    And,
}

#[derive(Clone, Debug, PartialEq)]
struct Input {
    a: String,
    b: String,
    op: Op,
    out: String,
}

fn parse_op(o: &str) -> Op {
    match o {
        "XOR" => Op::Xor,
        "OR" => Op::Or,
        "AND" => Op::And,
        _ => panic!(),
    }
}

fn parse() -> (HashMap<String, u64>, Vec<Input>) {
    let (start, ops) = PUZZLE.trim().split_once("\n\n").unwrap();
    let start_parsed = start
        .lines()
        .map(|s| {
            let (name, value) = s.split_once(": ").unwrap();
            (name.to_owned(), value.parse::<u64>().unwrap())
        })
        .collect();
    let ops_parsed = ops
        .lines()
        .map(|l| {
            let l_parsed = l.split_whitespace().collect::<Vec<_>>();
            Input {
                a: l_parsed[0].to_owned(),
                b: l_parsed[2].to_owned(),
                op: parse_op(l_parsed[1]),
                out: l_parsed[4].to_owned(),
            }
        })
        .collect();
    (start_parsed, ops_parsed)
}

fn find<'a>(ops: &'a [Input], state: &HashMap<String, u64>) -> Option<&'a Input> {
    ops.iter().find(|inp| {
        !state.contains_key(&inp.out) && state.contains_key(&inp.a) && state.contains_key(&inp.b)
    })
}

fn operate(a: u64, b: u64, op: Op) -> u64 {
    match op {
        Op::Xor => a ^ b,
        Op::Or => a | b,
        Op::And => a & b,
    }
}

fn put_together(prefix: &str, state: &HashMap<String, u64>) -> u64 {
    let mut vec = state
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .collect::<Vec<_>>();
    vec.sort();
    vec.into_iter()
        .enumerate()
        .fold(0u64, |acc, (idx, (_, value))| acc + (value << idx))
}

fn update_state(state: &mut HashMap<String, u64>, ops: &[Input]) {
    while let Some(input) = find(&ops, &state) {
        state.insert(
            input.out.clone(),
            operate(state[&input.a], state[&input.b], input.op),
        );
    }
}

fn part1() -> u64 {
    let (mut state, ops) = parse();
    update_state(&mut state, &ops);
    put_together("z", &state)
}

fn make_name(prefix: &str, index: usize) -> String {
    format!("{}{:02}", prefix, index)
}

fn paths_to(z_name: &str, ops: &[Input]) -> Vec<Vec<Input>> {
    let mut paths = vec![vec![z_name.to_string()]];
    let mut finished_paths = Vec::new();
    while let Some(path) = paths.pop() {
        let nexts = ops
            .iter()
            .filter(|op| op.out == *path.last().unwrap())
            .collect::<Vec<_>>();
        if nexts.is_empty() {
            finished_paths.push(
                path.iter()
                    .filter_map(|out| ops.iter().find(|i| i.out == *out))
                    .cloned()
                    .collect::<Vec<_>>(),
            );
        } else {
            for op in ops {
                if op.out == *path.last().unwrap() {
                    for n in [op.a.clone(), op.b.clone()] {
                        let mut p = path.clone();
                        p.push(n);
                        paths.push(p);
                    }
                }
            }
        }
    }
    finished_paths.sort_by_key(|v| v.len());
    finished_paths.dedup();
    finished_paths
}

fn swappy(ops: &mut [Input], sws: &[(&str, &str)]) {
    for &(sw1, sw2) in sws {
        for op in ops.iter_mut() {
            let tmp = op.clone();
            if op.out == sw1 {
                op.out = sw2.to_string();
            } else if op.out == sw2 {
                op.out = sw1.to_string();
            }
            if *op != tmp {
                println!("op before: {:?}", tmp);
                println!("op after: {:?}", op);
            }
        }
    }
}

fn part2_debug(sws: &[(&str, &str)]) {
    let (_, mut ops) = parse();
    swappy(&mut ops, &sws);
    for index in 0..=45 {
        let z_name = make_name("z", index);
        let finished_paths = paths_to(&z_name, &ops);
        for path in finished_paths.iter().take(5) {
            println!(
                "{} path: {}",
                z_name,
                path.iter()
                    .map(|i| format!("{}[{} {:?} {}]", i.out, i.a, i.op, i.b))
                    .collect::<Vec<_>>()
                    .join(" <- ")
            )
        }
        println!();
        if index >= 3 && index < 45 {
            let head = finished_paths[0].clone();

            //  The shortest input should be x xor y
            assert_eq!(head[1].op, Op::Xor);
            assert!(
                (head[1].a == make_name("x", index) && head[1].b == make_name("y", index))
                    || (head[1].a == make_name("y", index) && head[1].b == make_name("x", index))
            );

            // The output should always be an xor
            assert_eq!(head[0].op, Op::Xor);
        }
    }
}

fn part2() -> String {
    // Pairs found manually by inspecting the output of part2_debug.
    // The rules included as comments in part2_debug could be used to
    // write code that would find these pairs. But meh.
    let sws = &vec![
        ("z06", "fkp"),
        ("z11", "ngr"),
        ("z31", "mfm"),
        ("bpt", "krj"),
    ];
    part2_debug(&sws);
    let mut swapped = sws.iter().flat_map(|p| [p.0, p.1]).collect::<Vec<_>>();
    swapped.sort();
    swapped.join(",")
}

fn main() {
    let p2 = part2();
    println!("Part 1: {}", part1());
    println!("Part 2: {}", p2);
}
