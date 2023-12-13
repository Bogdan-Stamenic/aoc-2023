use std::{collections::HashMap, usize};
use ndarray::*;
use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::*,
    multi::*,
    combinator::*,
};

//#[aoc_generator(day13)]
//pub fn input_generator(input: &str) -> Array2<Pipe> {
//    let parse_vec = match all_consuming(separated_list1(tag("\n"), parse_pipes_line))
//        .parse(input)
//    {
//        Ok((_, val)) => val,
//        Err(e) => panic!("{}", e),
//    };
//    let shape = (parse_vec.len(), parse_vec[0].dim());
//    let flat = parse_vec.iter().flatten().cloned().collect();
//    let input = Array2::from_shape_vec(shape, flat).expect("Dimensions didn't line up");
//    input
//}
//
//#[aoc(day13, part1)]
//pub fn solve_part1(input: &Array2<Pipe>) -> usize {
//}
//
//#[aoc(day13, part2)]
//pub fn solve_part2(input: &Array2<Pipe>) -> u64 {
//}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_day11_parser() {
    }

    #[test]
    fn test_solve_day11_part1() {
    }

    #[test]
    fn test_solve_day11_part2() {
    }
}
