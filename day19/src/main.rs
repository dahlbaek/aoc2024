use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
};

const PUZZLE: &str = include_str!("puzzle");

fn parse() -> (HashSet<&'static str>, Vec<&'static str>) {
    let mut lines = PUZZLE.trim().lines();
    let available = lines.next().unwrap().split(", ").collect();
    assert_eq!(lines.next().unwrap(), "");
    let targets = lines.collect();
    (available, targets)
}

fn is_buildable<'a>(
    cache: &mut Cache<String, bool, str, HashSet<&'a str>>,
    target: &str,
    available: &HashSet<&'a str>,
) -> bool {
    target.is_empty()
        || (1..=target.len())
            .filter(|&j| available.contains(&target[..j]))
            .any(|j| cache.get_or_compute(&target[j..], available))
}

fn count<'a>(
    cache: &mut Cache<String, u64, str, HashSet<&'a str>>,
    target: &str,
    available: &HashSet<&'a str>,
) -> u64 {
    if target.is_empty() {
        1
    } else {
        (1..=target.len())
            .filter(|&j| available.contains(&target[..j]))
            .map(|j| cache.get_or_compute(&target[j..], available))
            .sum::<u64>()
    }
}

struct Cache<K, V, Q: ?Sized, C> {
    inner: HashMap<K, V>,
    f: fn(&mut Self, &Q, &C) -> V,
}

impl<K, V, Q, C> Cache<K, V, Q, C>
where
    Q: ?Sized,
    Q: Eq + Hash,
    K: Eq + Hash,
    Q: ToOwned<Owned = K>,
    K: Borrow<Q>,
    V: Clone,
{
    fn new(f: fn(&mut Self, &Q, &C) -> V) -> Cache<K, V, Q, C> {
        let inner = HashMap::new();
        Cache { inner, f }
    }

    fn get_or_compute(&mut self, q: &Q, c: &C) -> V {
        match self.inner.get(q) {
            Some(v) => v.clone(),
            None => {
                let v = (self.f)(self, q, c);
                self.inner.insert(q.to_owned(), v.clone());
                v
            }
        }
    }
}

fn main() {
    let (available, targets) = parse();

    let part1 = targets
        .iter()
        .filter(|target| Cache::new(is_buildable).get_or_compute(target, &available))
        .count();
    println!("Part 1: {}", part1);

    let part2 = targets
        .iter()
        .map(|target| Cache::new(count).get_or_compute(target, &available))
        .sum::<u64>();
    println!("Part 2: {}", part2);
}