use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::successors,
};

const PUZZLE: &str = include_str!("puzzle");

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
struct Vertex(&'static str);

#[derive(Debug)]
struct Edge(Vertex, Vertex);

fn parse() -> (HashSet<Vertex>, Vec<Edge>) {
    let mut vs = HashSet::new();
    let mut es = Vec::new();
    PUZZLE.trim().lines().for_each(|s| {
        let (v1r, v2r) = s.split_once('-').unwrap();
        let v1 = Vertex(v1r);
        let v2 = Vertex(v2r);
        vs.insert(v1);
        vs.insert(v2);
        es.push(Edge(v1, v2));
        es.push(Edge(v2, v1));
    });
    (vs, es)
}

fn neighbours(v: Vertex, es: &[Edge]) -> HashSet<Vertex> {
    es.iter().filter(|e| e.0 == v).map(|e| e.1).collect()
}

fn get_triples(vs: &HashSet<Vertex>, es: &[Edge]) -> HashSet<[Vertex; 3]> {
    let mut triples = HashSet::new();
    for &v in vs {
        let v_neighbours = neighbours(v, es);
        for &n in &v_neighbours {
            let n_neighbours = neighbours(n, es);
            for &n2 in v_neighbours.intersection(&n_neighbours) {
                let mut tmp = [v, n, n2];
                tmp.sort();
                triples.insert(tmp);
            }
        }
    }
    triples
}

fn get_one_more(
    sets: &HashMap<Vec<Vertex>, HashSet<Vertex>>,
    cached_neighbours: &HashMap<Vertex, HashSet<Vertex>>,
) -> Option<HashMap<Vec<Vertex>, HashSet<Vertex>>> {
    let new_sets = sets
        .iter()
        .flat_map(|(s, intersection)| {
            intersection.iter().map(|&inter| {
                let mut new_s = s.clone();
                new_s.push(inter);
                new_s.sort();
                let new_intersection = intersection
                    .intersection(&cached_neighbours[&inter])
                    .cloned()
                    .collect();
                (new_s, new_intersection)
            })
        })
        .collect::<HashMap<_, _>>();
    Some(new_sets).filter(|i| !i.is_empty())
}

fn main() {
    let (vs, es) = parse();

    let part1 = get_triples(&vs, &es)
        .iter()
        .filter(|component| component.iter().any(|computer| computer.0.starts_with('t')))
        .count();
    println!("Part 1: {}", part1);

    let cached_neighbours = vs.iter().map(|&v| (v, neighbours(v, &es))).collect();
    let first = Some(vs.iter().map(|&v| (vec![v], neighbours(v, &es))).collect());
    let part2 = successors(first, |sets| get_one_more(sets, &cached_neighbours))
        .last()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .0
        .iter()
        .map(|v| v.0)
        .collect::<Vec<_>>()
        .join(",");
    println!("Part 2: {}", part2);
}
