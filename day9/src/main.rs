use std::cmp::min;

const PUZZLE: &[u8] = include_bytes!("puzzle");

#[derive(Debug)]
struct Slot {
    id: u64,
    size: u64,
}

impl Slot {
    fn new(id: u64, size: u64) -> Slot {
        Slot { id, size }
    }
}

fn get(index: usize) -> u64 {
    (PUZZLE[index] - 48).into()
}

fn take_left(left_index: usize, left_id: u64) -> (usize, u64, u64) {
    (left_index + 2, left_id + 1, get(left_index + 1))
}

fn take_right(right_index: usize, right_id: u64) -> (usize, u64, u64) {
    (right_index - 2, right_id - 1, get(right_index))
}

fn part1() -> u64 {
    let mut left_index = 0;
    let mut left_id = 0;
    let mut to_fill = 0;

    let mut right_index = PUZZLE.len() - 1;
    let mut right_id = ((PUZZLE.len() + 1) / 2).try_into().unwrap();
    let mut to_push = 0;

    let mut disk = Vec::new();

    while left_id < right_id {
        if to_fill == 0 {
            disk.push(Slot::new(left_id, get(left_index)));
            (left_index, left_id, to_fill) = take_left(left_index, left_id);
        } else if to_push == 0 {
            (right_index, right_id, to_push) = take_right(right_index, right_id);
        } else {
            let to = min(to_fill, to_push);
            disk.push(Slot::new(right_id, to));
            to_fill -= to;
            to_push -= to;
        }
    }

    if to_push > 0 {
        disk.push(Slot::new(right_id, to_push));
    }

    let mut start_index = 0;
    let mut agg = 0;
    for slot in disk {
        agg += slot.id * (slot.size * start_index + (slot.size - 1) * (slot.size) / 2);
        start_index += slot.size
    }

    agg
}

#[derive(Debug)]
struct Slot2 {
    id: Option<u64>,
    size: u64,
}

impl Slot2 {
    fn new(id: Option<u64>, size: u64) -> Slot2 {
        Slot2 { id, size }
    }
}

fn start_disk() -> Vec<Slot2> {
    let mut disk = Vec::new();
    let mut id = 0;
    let mut index = 0;
    disk.push(Slot2::new(Some(id), get(index)));
    index += 1;
    id += 1;
    while index < PUZZLE.len() - 1 {
        let empty_size = get(index);
        if empty_size > 0 {
            disk.push(Slot2::new(None, empty_size));
        }
        disk.push(Slot2::new(Some(id), get(index + 1)));
        id += 1;
        index += 2;
    }
    disk
}

fn end_disk(mut disk: Vec<Slot2>) -> Vec<Slot2> {
    let mut outcome = Vec::new();
    while let Some(head) = disk.pop() {
        let insert = head.id.and_then(|_| {
            disk.iter()
                .enumerate()
                .find(|(_, slot)| slot.id.is_none() && slot.size >= head.size)
                .map(|(index, slot)| (index, slot.size - head.size))
        });
        match insert {
            None => outcome.push(head),
            Some((index, empty_size)) => {
                disk.push(Slot2::new(None, head.size));
                disk[index] = head;
                if empty_size > 0 {
                    disk.insert(index + 1, Slot2::new(None, empty_size));
                }
            }
        }
    }
    outcome.reverse();
    outcome
}

fn part2() -> u64 {
    let disk = start_disk();
    let outcome = end_disk(disk);

    outcome
        .iter()
        .scan(0, |state, slot| {
            let start_index = *state;
            *state += slot.size;
            Some((start_index, slot))
        })
        .filter_map(|(start_index, slot)| slot.id.map(|id| (start_index, id, slot.size)))
        .fold(0, |agg, (start_index, id, size)| {
            agg + id * (size * start_index + (size - 1) * (size) / 2)
        })
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
