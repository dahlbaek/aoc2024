const PUZZLE: &[u8] = include_bytes!("puzzle");

fn parse() -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let (mut locks, mut keys) = (Vec::new(), Vec::new());

    let mut s = PUZZLE;
    while s.len() > 0 {
        let position = s
            .windows(2)
            .position(|w| w == &[b'\n', b'\n'])
            .unwrap_or(s.len() - 2);
        let (schematic, end) = s.split_at(position);
        s = &end[2..];
        let mut heights = [0; 5];
        schematic
            .iter()
            .skip(6)
            .take(5 * 6)
            .enumerate()
            .filter(|(_, &b)| b == b'#')
            .map(|(index, _)| index % 6)
            .for_each(|x| heights[x] += 1);
        if schematic[0] == b'#' {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (locks, keys)
}

fn main() {
    let (locks, keys) = parse();
    let part1 = locks
        .iter()
        .flat_map(|l| keys.iter().map(move |k| (l, k)))
        .filter(|(l, k)| l.iter().zip(k.iter()).all(|(lh, kh)| lh + kh <= 5))
        .count();

    println!("Part 1: {}", part1);
}
