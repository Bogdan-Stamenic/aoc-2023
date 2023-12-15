use ndarray::*;
use std::collections::HashMap;
use std::fmt::{self, Display};
use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::{all_consuming, value},
    bytes::complete::tag,
    multi::{separated_list1,many1},
};

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub enum RockType {
    Empty,
    Fixed,
    Rollable,
}

impl Display for RockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RockType::Rollable => write!(f, "O"),
            RockType::Empty => write!(f, "."),
            RockType::Fixed => write!(f, "#"),
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Array2<RockType> {
    let parse_vec = match all_consuming(separated_list1(tag("\n"), parse_platform_line))
        .parse(input)
    {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    };
    let shape = (parse_vec.len(), parse_vec[0].dim());
    let flat = parse_vec.iter().flatten().cloned().collect();
    let input = Array2::from_shape_vec(shape, flat).expect("Dimensions didn't line up");
    input
}

fn parse_platform_line(input: &str) -> IResult<&str, Array1<RockType>> {
    many1(parse_platform_entry)
        .map(|el| Array1::from_shape_vec(el.len(), el).unwrap())
        .parse(input)
}

fn parse_platform_entry(input: &str) -> IResult<&str, RockType>{
    alt((
            value(RockType::Empty, tag(".")),
            value(RockType::Fixed, tag("#")),
            value(RockType::Rollable, tag("O")),
    ))
        .parse(input)
}

#[allow(dead_code)]
fn pretty_print(input: Array2<RockType>) {
    for row in input.outer_iter() {
        for el in row.iter() {
            print!("{}", el);
        }
        println!();
    }
}

#[allow(dead_code)]
fn tilt_north(input: &Array2<RockType>, out: &mut Array2<RockType>) {
    let (row_max, col_max) = input.dim();
    for col_idx in 0..col_max {
        let mut next = 0;
        for row_idx in 0..row_max {
            match input[[row_idx,col_idx]] {
                RockType::Rollable => {
                    out[[next, col_idx]] = RockType::Rollable;
                    if row_idx != next {
                        out[[row_idx, col_idx]] = RockType::Empty;
                    }
                    next += if next <= row_max - 1 {1} else {0};
                },
                RockType::Fixed => {
                    next = row_idx + 1;
                },
                RockType::Empty => {},
            }
        }
    }
}

#[allow(dead_code)]
fn tilt_south(input: &Array2<RockType>, out: &mut Array2<RockType>) {
    let (row_max, col_max) = input.dim();
    for col_idx in 0..col_max {
        let mut next = row_max - 1;
        for row_idx in (0..row_max).rev() {
            match input[[row_idx,col_idx]] {
                RockType::Rollable => {
                    out[[next, col_idx]] = RockType::Rollable;
                    if row_idx != next {
                        out[[row_idx, col_idx]] = RockType::Empty;
                    }
                    next -= if next == 0 {0} else {1};
                },
                RockType::Fixed => {
                    next = if row_idx == 0 {row_max - 1} else {row_idx - 1};
                },
                RockType::Empty => {},
            }
        }
    }
}

#[allow(dead_code)]
fn tilt_west(input: &Array2<RockType>, out: &mut Array2<RockType>) {
    let (row_max, col_max) = input.dim();
    for row_idx in 0..row_max {
        let mut next = 0;
        for col_idx in 0..col_max {
            match input[[row_idx,col_idx]] {
                RockType::Rollable => {
                    out[[row_idx, next]] = RockType::Rollable;
                    if col_idx != next {
                        out[[row_idx, col_idx]] = RockType::Empty;
                    }
                    next += if next <= col_max - 1 {1} else {0};
                },
                RockType::Fixed => {
                    next = col_idx + 1;
                },
                RockType::Empty => {},
            }
        }
    }
}

#[allow(dead_code)]
fn tilt_east(input: &Array2<RockType>, out: &mut Array2<RockType>) {
    let (row_max, col_max) = input.dim();
    for row_idx in 0..row_max {
        let mut next = col_max - 1;
        for col_idx in (0..col_max).rev() {
            match input[[row_idx,col_idx]] {
                RockType::Rollable => {
                    out[[row_idx, next]] = RockType::Rollable;
                    if col_idx != next {
                        out[[row_idx, col_idx]] = RockType::Empty;
                    }
                    next -= if col_idx == 0 {0} else {1};
                },
                RockType::Fixed => {
                    next = if col_idx == 0 {col_max - 1} else {col_idx - 1};
                },
                RockType::Empty => {},
            }
        }
    }
}

fn calc_north_side_load(input: &Array2<RockType>) -> u64 {
    let (outer_max, _) = input.dim();
    let mut out = 0;
    for (i, row) in input.outer_iter().enumerate() {
        out += row.iter()
            .filter(|el| {
                **el == RockType::Rollable
            })
            .map(|_| (outer_max - i) as u64)
            .sum::<u64>();
    }
    out
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Array2<RockType>) -> u64 {
    let mut out = input.clone();
    tilt_north(input, &mut out);
    calc_north_side_load(&out)
}


fn one_cycle_for_p2(input: &mut Array2<RockType>, out: &mut Array2<RockType>) {
    tilt_north(input, out);
    *input = out.clone();
    tilt_west(input, out);
    *input = out.clone();
    tilt_south(input, out);
    *input = out.clone();
    tilt_east(input, out);
    *input = out.clone();
}

fn find_cycle_len(input: &mut Array2<RockType>) -> u64 {
    let mut results = HashMap::<Array2<RockType>,u64>::new();
    let mut counter = 0;
    let mut out = input.clone();
    loop {
        if results.contains_key(&out) {
            break;
        } else if counter > 1000 {
            panic!("Taking too long to find cycle length");
        }
        results.insert(out.clone(), counter);
        one_cycle_for_p2(input, &mut out);
        counter += 1;
    }
    let offset = results.get(&out).unwrap().clone();
    let cycle_len = counter - offset;
    let rem = (1_000_000_000u64 - offset) % cycle_len;
    results.iter()
        .filter(|(_,val)| **val == rem + offset)
        .map(|(key,_)| calc_north_side_load(key))
        .next().unwrap()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Array2<RockType>) -> u64 {
    let mut out = input.clone();
    find_cycle_len(&mut out)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.dim(),(10,10));
    }

    #[test]
    #[ignore = "Only pretty prints"]
    fn test_tilt_north() {
        let input = input_generator(TEST_INPUT);
        let mut out = input.clone();
        tilt_north(&input, &mut out);
        pretty_print(out);
    }

    #[test]
    #[ignore = "Only pretty prints"]
    fn test_tilt_west() {
        let input = input_generator(TEST_INPUT);
        let mut out = input.clone();
        tilt_west(&input, &mut out);
        pretty_print(out);
    }

    #[test]
    #[ignore = "Only pretty prints"]
    fn test_tilt_south() {
        let input = input_generator(TEST_INPUT);
        let mut out = input.clone();
        tilt_south(&input, &mut out);
        pretty_print(out);
    }

    #[test]
    #[ignore = "Only pretty prints"]
    fn test_tilt_east() {
        let input = input_generator(TEST_INPUT);
        let mut out = input.clone();
        tilt_east(&input, &mut out);
        pretty_print(out);
    }

    #[test]
    #[ignore = "Only pretty prints"]
    fn test_one_cycle() {
        let mut input = input_generator(TEST_INPUT);
        let mut out = input.clone();
        one_cycle_for_p2(&mut input, &mut out);
        one_cycle_for_p2(&mut input, &mut out);
        one_cycle_for_p2(&mut input, &mut out);
        pretty_print(out);
    }

    #[test]
    fn test_find_cycle_len() {
        let mut input = input_generator(TEST_INPUT);
        let ans = find_cycle_len(&mut input);
        assert_eq!(ans, 64);
    }

    #[test]
    fn test_solve_day12_p1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 136);
    }

//    #[test]
//    fn test_solve_day12_p2_foo() {
//        let input = input_generator(TEST_INPUT);
//        let ans = solve_part2(&input);
//        assert_eq!(ans, 64);
//    }
}
