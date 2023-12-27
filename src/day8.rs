use core::panic;
use std::collections::HashMap;
use num::integer::lcm;
use nom::{
    Parser,
    IResult,
    branch::alt,
    character::complete::one_of,
    combinator::{all_consuming, value},
    bytes::complete::tag,
    multi::{count,separated_list1, many1}, sequence::{separated_pair,delimited},
};

#[derive(Clone,Copy,Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug,Hash,PartialEq,Eq)]
struct Location {
    coords: (u8,u8,u8),
}

#[allow(dead_code)]
pub struct DesertMap {
    dirs: Vec<Direction>,
    map: HashMap<Location,(Location,Location)>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> DesertMap {
    match all_consuming(separated_pair(parse_all_directions, tag("\n\n"), parse_map))
        .map(|(x,y)| DesertMap { dirs: x, map: y })
        .parse(input)
    {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    }
}

fn parse_all_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(input)
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
    ))
        .parse(input)
}

fn parse_map(input: &str) -> IResult<&str,HashMap<Location,(Location,Location)>> {
    separated_list1(tag("\n"), parse_location_line)
        .map(|vec| {
            vec.into_iter().collect::<HashMap<Location,(Location,Location)>>()
        })
    .parse(input)
}

fn parse_location_line(input: &str) -> IResult<&str, (Location, (Location,Location))> {
    separated_pair(
        parse_location, tag(" = "),
        delimited(
            tag("("),
            separated_pair(parse_location, tag(", "), parse_location),
            tag(")"),
        ))
        .parse(input)
}

fn parse_location(input: &str) -> IResult<&str,Location> {
    count(parse_location_letter, 3)
        .map(|vec| Location {coords: (vec[0], vec[1],vec[2])})
        .parse(input)
}

#[inline]
fn parse_location_letter<'a>(input: &'a str) -> IResult<&'a str, u8> {
    alt((
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            one_of("12")
    ))
        .map(|el: char| el.to_string().bytes().next().unwrap())
        .parse(input)
}

fn find_cycle_len_p1(input: &DesertMap, start: &Location) -> u64 {
    let mut counter: u64 = 0;
    let mut current_loc = start;
    let my_dir_it = input.dirs.iter().cycle();
    for direction in my_dir_it {
        counter += 1;
        current_loc = match direction {
            Direction::Left => &input.map.get(current_loc).unwrap().0,
            Direction::Right => &input.map.get(current_loc).unwrap().1,
        };
        if current_loc.coords == (b'Z',b'Z',b'Z') {
            break;
        }
    }
    counter
}

#[inline]
fn find_cycle_len_p2(input: &DesertMap, start: &Location) -> u64 {
    let mut counter: u64 = 0;
    let mut current_loc = start;
    let my_dir_it = input.dirs.iter().cycle();
    for direction in my_dir_it {
        counter += 1;
        current_loc = match direction {
            Direction::Left => &input.map.get(current_loc).unwrap().0,
            Direction::Right => &input.map.get(current_loc).unwrap().1,
        };
        if current_loc.coords.2 == b'Z' {
            break;
        }
    }
    counter
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &DesertMap) -> u64 {
    let start = &Location {coords: (b'A',b'A',b'A')};
    let ans = find_cycle_len_p1(input, start);
    ans
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &DesertMap) -> u64 {
    let starting_points = input.map.iter()
        .map(|(x,_)| x)
        .filter(|x| x.coords.2 == b'A')
        .collect::<Vec<&Location>>();
    starting_points.iter()
        .map(|start| find_cycle_len_p2(input, start))
        .reduce(|acc,el| lcm(acc, el)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn day8_parser() {
        let input = input_generator(TEST_INPUT1);
        assert_eq!(input.dirs.len(), 2);
        assert_eq!(input.map.len(), 7);
        assert_eq!(input.map.contains_key(&Location {coords: (b'A',b'A',b'A')}), true);
    }
    
    #[test]
    fn day8_solve_p1_1() {
        let input = input_generator(TEST_INPUT1);
        let ans = solve_part1(&input);
        assert_eq!(ans, 2);
    }

const TEST_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn day8_solve_p1_2() {
        let input = input_generator(TEST_INPUT2);
        let ans = solve_part1(&input);
        assert_eq!(ans, 6);
    }

    const TEST_INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    #[test]
    fn day8_solve_p2() {
        let input = input_generator(TEST_INPUT3);
        let ans = solve_part2(&input);
        assert_eq!(ans, 6);
    }

    #[test]
    fn day8_lcm() {
        assert_eq!(lcm(3, 7), 21);
        assert_eq!(lcm(3, 6), 6);
    }

}
