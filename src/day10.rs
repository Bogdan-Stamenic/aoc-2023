use std::collections::HashSet;
use ndarray::*;
use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::*,
    multi::*,
    combinator::*,
};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SEBend,
    SWBend,
    Ground,
    Start,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Array2<Pipe> {
    let parse_vec = match all_consuming(separated_list1(tag("\n"), parse_pipes_line))
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

fn parse_pipes_line(input: &str) -> IResult<&str, Array1<Pipe>> {
    many1(parse_one_pipe)
        .map(|vec| Array1::from_shape_vec(vec.len(), vec).unwrap())
        .parse(input)
}

fn parse_one_pipe(input: &str) -> IResult<&str, Pipe> {
    alt((
            value(Pipe::Vertical, tag("|")),
            value(Pipe::Horizontal, tag("-")),
            value(Pipe::NEBend, tag("L")),
            value(Pipe::NWBend, tag("J")),
            value(Pipe::SWBend, tag("7")),
            value(Pipe::SEBend, tag("F")),
            value(Pipe::Ground, tag(".")),
            value(Pipe::Start, tag("S")),
            ))
        .parse(input)
}

fn find_connecting_pipes_p1(input: &Array2<Pipe>, current: Pipe,
    coords_curr: (usize,usize)) -> Vec<(usize,usize)> {
    match current {
        Pipe::Vertical  => {
            vec![
                (coords_curr.0 + 1, coords_curr.1),
                (coords_curr.0 - 1, coords_curr.1)
            ]
        },
        Pipe::Horizontal  => {
            vec![
                (coords_curr.0, coords_curr.1 - 1),
                (coords_curr.0, coords_curr.1 + 1)
            ]
        },
        Pipe::NEBend => {
            vec![
                (coords_curr.0, coords_curr.1 + 1),
                (coords_curr.0 - 1, coords_curr.1)
            ]
        },
        Pipe::NWBend => {
            vec![
                (coords_curr.0, coords_curr.1 - 1),
                (coords_curr.0 - 1, coords_curr.1)
            ]
        },
        Pipe::SWBend => {
            vec![
                (coords_curr.0, coords_curr.1 - 1),
                (coords_curr.0 + 1, coords_curr.1)
            ]
        },
        Pipe::SEBend => {
            vec![
                (coords_curr.0, coords_curr.1 + 1),
                (coords_curr.0 + 1, coords_curr.1)
            ]
        },
        Pipe::Start => {
            return pipes_connecting_to_start(input, coords_curr);
        },
        Pipe::Ground => panic!("Ground has no pipe connections"),
    }
}

fn pipes_connecting_to_start(input: &Array2<Pipe>, coords_curr: (usize,usize))
    -> Vec<(usize,usize)> {
        let mut foo = Vec::<(usize,usize)>::new();
        let (max_outer, max_inner) = input.dim();
        if coords_curr.1 > 1 {//look left/west
            match input[[coords_curr.0, coords_curr.1 - 1]] {
                Pipe::NEBend => foo.push((coords_curr.0, coords_curr.1 - 1)),
                Pipe::SEBend => foo.push((coords_curr.0, coords_curr.1 - 1)),
                Pipe::Horizontal => foo.push((coords_curr.0, coords_curr.1 - 1)),
                _ => {},
            };
        }
        if coords_curr.1 <  max_inner - 1 {//look right/east
            match input[[coords_curr.0, coords_curr.1 + 1]] {
                Pipe::NWBend => foo.push((coords_curr.0, coords_curr.1 + 1)),
                Pipe::SWBend => foo.push((coords_curr.0, coords_curr.1 + 1)),
                Pipe::Horizontal => foo.push((coords_curr.0, coords_curr.1 + 1)),
                _ => {},
            };
        }
        if coords_curr.0 > 1 {//look up/north
            match input[[coords_curr.0 - 1, coords_curr.1]] {
                Pipe::SEBend => foo.push((coords_curr.0 - 1, coords_curr.1)),
                Pipe::SWBend => foo.push((coords_curr.0 - 1, coords_curr.1)),
                Pipe::Vertical => foo.push((coords_curr.0 - 1, coords_curr.1)),
                _ => {},
            };
        }
        if coords_curr.0 < max_outer - 1 {//look down/south
            match input[[coords_curr.0 + 1, coords_curr.1]] {
                Pipe::NEBend => foo.push((coords_curr.0 + 1, coords_curr.1)),
                Pipe::NWBend => foo.push((coords_curr.0 + 1, coords_curr.1)),
                Pipe::Vertical => foo.push((coords_curr.0 + 1, coords_curr.1)),
                _ => {},
            };
        }
        /* To pipes connecting to start */
        if foo.len() == 2 {
            return foo;
        }
        unreachable!("Double-check pipes_connecting_to_start()")
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Array2<Pipe>) -> usize {
    let mut start_pos = (0,0);
    for (coords,el) in input.indexed_iter() {
        if *el == Pipe::Start {
            start_pos = coords;
            break;
        }
    }
    //let mut visited = HashSet::from([start_pos]);
    let mut visited = HashSet::from([start_pos]);
    let mut current_pos = pipes_connecting_to_start(&input, start_pos)[0];
    visited.insert(current_pos.clone());
    loop {
        let positions = find_connecting_pipes_p1(&input,
            input[[current_pos.0, current_pos.1]], current_pos);
        if positions.iter().all(|el| visited.contains(el)) {
            break;
        }
        current_pos = *positions.iter().filter(|el| !visited.contains(el)).next().unwrap();
        visited.insert(current_pos.clone());
    }
    let ans = visited.iter().count() / 2;
    ans
}

//#[aoc(day10, part2)]
//pub fn solve_part2(input: &[Vec<i64>]) -> i64 {
//}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const TEST_INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const TEST_INPUT3: &str = ".....
.....
..F7.
..||.
..LS.";

    #[test]
    fn test_day10_parser() {
        let input = input_generator(TEST_INPUT1);
        assert_eq!(input.dim(), (5,5));
        assert_eq!(input[[1,1]], Pipe::Start);
    }

    #[test]
    fn test_pipes_connecting_to_start() {
        let input1 = input_generator(TEST_INPUT1);
        let ans1 = pipes_connecting_to_start(&input1, (1,1));
        let input2 = input_generator(TEST_INPUT2);
        let ans2 = pipes_connecting_to_start(&input2, (2,0));
        let input3 = input_generator(TEST_INPUT3);
        let ans3 = pipes_connecting_to_start(&input3, (4,3));
        assert_eq!(ans1, vec![(1,2),(2,1)]);
        assert_eq!(ans2, vec![(2,1),(3,0)]);
        assert_eq!(ans3, vec![(4,2),(3,3)]);
    }

    #[test]
    fn test_find_connecting_pipes_p1() {
        let input = input_generator(TEST_INPUT3);
        let ans1 = find_connecting_pipes_p1(&input, Pipe::Vertical, (3,2));
        let ans2 = find_connecting_pipes_p1(&input, Pipe::SEBend, (2,2));
        let ans3 = find_connecting_pipes_p1(&input, Pipe::SWBend, (2,3));
        let ans4 = find_connecting_pipes_p1(&input, Pipe::NEBend, (4,2));
        assert_eq!(ans1, vec![(4,2), (2,2)]);
        assert_eq!(ans2, vec![(2,3), (3,2)]);
        assert_eq!(ans3, vec![(2,2), (3,3)]);
        assert_eq!(ans4, vec![(4,3), (3,2)]);
    }

    #[test]
    fn test_solve_day10_part1() {
        let input1 = input_generator(TEST_INPUT1);
        let ans1 = solve_part1(&input1);
        assert_eq!(ans1, 4);
        let input2 = input_generator(TEST_INPUT2);
        let ans2 = solve_part1(&input2);
        assert_eq!(ans2, 8);
        let input3 = input_generator(TEST_INPUT3);
        let ans3 = solve_part1(&input3);
        assert_eq!(ans3, 3);
    }

    //#[test]
    //fn test_solve_day10_part2() {
    //}
}
