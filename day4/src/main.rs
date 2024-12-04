const PUZZLE: &[u8] = include_bytes!("puzzle");
const DIM: usize = 140;
const XMAS: &[u8] = b"XMAS";
const SAMX: &[u8] = b"SAMX";

fn count_occurrences<T: AsRef<[u8]>>(line: T) -> usize {
    line.as_ref()
        .windows(4)
        .filter(|&w| w == XMAS || w == SAMX)
        .count()
}

fn part1() -> usize {
    let rows = PUZZLE
        .split(|&b| b == b'\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<&[u8]>>();
    let dim = rows.len();
    let cols = {
        let mut cols = Vec::new();
        for i in 0..dim {
            let mut col = Vec::new();
            for j in 0..dim {
                col.push(rows[j][i])
            }
            cols.push(col);
        }
        cols
    };

    let netosw = {
        let mut netosw = Vec::new();
        for i in 0..2 * dim - 1 {
            let mut diag = Vec::new();
            for j in 0..dim {
                if let Some(row) = rows.get(j) {
                    if i >= j {
                        if let Some(&v) = row.get(i - j) {
                            diag.push(v);
                        }
                    }
                }
            }
            netosw.push(diag);
        }
        netosw
    };
    let nwtose = {
        let mut nwtose = Vec::new();
        for i in 0..2 * dim - 1 {
            let mut diag = Vec::new();
            for j in 0..dim {
                if let Some(row) = rows.get(j) {
                    if dim + j >= i + 1 {
                        if let Some(&v) = row.get(dim + j - 1 - i) {
                            diag.push(v);
                        }
                    }
                }
            }
            nwtose.push(diag);
        }
        nwtose
    };

    rows.iter().map(count_occurrences).sum::<usize>()
        + cols.iter().map(count_occurrences).sum::<usize>()
        + netosw.iter().map(count_occurrences).sum::<usize>()
        + nwtose.iter().map(count_occurrences).sum::<usize>()
}

const MAS: &[u8] = b"MAS";
const SAM: &[u8] = b"SAM";

macro_rules! get {
    ( $row:expr,$col:expr ) => {
        match PUZZLE.get($col + (DIM + 1) * $row) {
            Some(x) => *x,
            None => return false,
        }
    };
}

fn is_xmas(row: usize, col: usize) -> bool {
    if row == 0 || col == 0 {
        false
    } else {
        let nwtose = &[
            get!(row - 1, col - 1),
            get!(row, col),
            get!(row + 1, col + 1),
        ];
        let netosw = &[
            get!(row - 1, col + 1),
            get!(row, col),
            get!(row + 1, col - 1),
        ];
        (nwtose == MAS || nwtose == SAM) && (netosw == MAS || netosw == SAM)
    }
}

fn part2() -> usize {
    let mut agg = 0;
    for row in 0..DIM {
        for col in 0..DIM {
            if is_xmas(row, col) {
                agg += 1;
            }
        }
    }
    agg
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
