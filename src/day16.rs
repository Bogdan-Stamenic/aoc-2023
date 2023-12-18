use std::{collections::HashMap, usize};
use petgraph::{graph::{DiGraph, NodeIndex}, graphmap::DiGraphMap};

#[derive(Clone,Copy,Debug)]
pub struct OpticsFixture {
    fixture_type: Optics,
    loc: (usize,usize)
}

#[derive(Clone,Copy,Debug)]
enum Optics {
    MirrorSlash,      // /
    MirrorBackslash,  // \
    VertSplitter,     // |
    HorzSplitter      // -
}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
struct MirrorNode {
    coords: (usize,usize),
    beam_from: IncidenceDirection,
}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
enum IncidenceDirection {
    Above,
    Below,
    Left,
    Right,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> DiGraphMap<MirrorNode,usize> {
    let optics_fixtures = input_to_optics_fixtures(input);
    build_mirror_graph(&optics_fixtures)
}

fn input_to_optics_fixtures(input: &str) -> Vec<OpticsFixture> {
    input.lines()
        .enumerate()
        .map(|(i, line): (usize,&str)| {
            line.bytes()
                .enumerate()
                .filter(|(_,val)| *val != b'.')
                .map(|(j, ch)| {
                    let ftype = match ch {
                        b'/' => Optics::MirrorSlash,
                        b'\\' => Optics::MirrorBackslash,
                        b'|' => Optics::VertSplitter,
                        b'-' => Optics::HorzSplitter,
                        _ => unreachable!("Unexpected symbol")
                    };
                    OpticsFixture {fixture_type: ftype, loc: (i,j)}
                })
            .collect::<Vec<_>>()
        })
    .flatten()
        .collect()
}

fn calc_manhattan_dist(x: (usize,usize), y: (usize,usize)) -> usize {
    let ans = (x.0 as i32 - y.0 as i32).abs() + (x.1 as i32 - y.1 as i32).abs();
    ans as usize
}

fn build_mirror_graph(input: &[OpticsFixture]) -> DiGraphMap<MirrorNode,usize> {
    let mut mirror_layout = DiGraph::<MirrorNode,usize>::new();
}

//#[aoc(day16, part1)]
//pub fn solve_part1(input: &str) -> u64 {
//}
//
//#[aoc(day16, part2)]
//pub fn solve_part2(input: &str) -> u64 {
//}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    //#[test]
    //fn test_solve_day16_p1() {
    //}

    //#[test]
    //fn test_solve_day16_p2() {
    //}
}
