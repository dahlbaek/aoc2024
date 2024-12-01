use std::collections::HashMap;

const PUZZLE: &str = include_str!("puzzle");

fn main() {
    let (mut list1, mut list2): (Vec<u64>, Vec<u64>) = PUZZLE
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(str::split_whitespace)
        .map(|mut it| {
            (
                it.next().unwrap().parse::<u64>().unwrap(),
                it.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .unzip();

    list1.sort_unstable();
    list2.sort_unstable();

    let part1 = list1
        .iter()
        .zip(list2.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .reduce(|x, y| x + y)
        .unwrap();

    println!("Part 1: {}", part1);

    let list2_occurrences = list2
        .iter()
        .fold(HashMap::<u64, u64>::new(), |mut agg, &elem| {
            *agg.entry(elem).or_default() += 1;
            agg
        });

    let part2 = list1
        .iter()
        .map(|&elem| {
            elem * list2_occurrences
                .get(&elem)
                .map(ToOwned::to_owned)
                .unwrap_or_default()
        })
        .reduce(|x, y| x + y)
        .unwrap();

    println!("Part 2: {}", part2);
}
