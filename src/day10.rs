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
    coords_curr: (usize,usize)) -> Vec<((usize,usize), Pipe)> {
    match current {
        Pipe::Vertical  => {
            vec![
                ((coords_curr.0 + 1, coords_curr.1), input[[coords_curr.0 + 1, coords_curr.1]]),
                ((coords_curr.0 - 1, coords_curr.1), input[[coords_curr.0 - 1, coords_curr.1]])
            ]
        },
        Pipe::Horizontal  => {
            vec![
                ((coords_curr.0, coords_curr.1 - 1), input[[coords_curr.0, coords_curr.1 - 1]]),
                ((coords_curr.0, coords_curr.1 + 1), input[[coords_curr.0, coords_curr.1 + 1]]),
            ]
        },
        Pipe::NEBend => {
            vec![
                ((coords_curr.0, coords_curr.1 + 1), input[[coords_curr.0, coords_curr.1 + 1]]),
                ((coords_curr.0 - 1, coords_curr.1), input[[coords_curr.0 - 1, coords_curr.1]])
            ]
        },
        Pipe::NWBend => {
            vec![
                ((coords_curr.0, coords_curr.1 - 1), input[[coords_curr.0, coords_curr.1 - 1]]),
                ((coords_curr.0 - 1, coords_curr.1), input[[coords_curr.0 - 1, coords_curr.1]])
            ]
        },
        Pipe::SWBend => {
            vec![
                ((coords_curr.0, coords_curr.1 - 1), input[[coords_curr.0, coords_curr.1 - 1]]),
                ((coords_curr.0 + 1, coords_curr.1), input[[coords_curr.0 + 1, coords_curr.1]])
            ]
        },
        Pipe::SEBend => {
            vec![
                ((coords_curr.0, coords_curr.1 + 1), input[[coords_curr.0, coords_curr.1 + 1]]),
                ((coords_curr.0 + 1, coords_curr.1), input[[coords_curr.0 + 1, coords_curr.1]])
            ]
        },
        Pipe::Start => {
            return pipes_connecting_to_start(input, coords_curr);
        },
        Pipe::Ground => panic!("Ground has no pipe connections"),
    }
}

fn pipes_connecting_to_start(input: &Array2<Pipe>, coords_curr: (usize,usize))
    -> Vec<((usize,usize), Pipe)> {
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
            return foo.into_iter()
                .map(|(x,y)| ((x,y), input[[x, y]]))
                .collect::<Vec::<((usize,usize), Pipe)>>();
        }
        unreachable!("Double-check pipes_connecting_to_start()")
}

fn figure_out_which_pipe_start_is(pipes_adj_to_start: &[((usize,usize), Pipe)]) -> Pipe {
    Pipe::Start
}

fn calc_start_loop(input: &Array2<Pipe>) -> HashMap<(usize,usize), Pipe> {
    let mut start_pos = (0usize,0usize);
    for (coord,el) in input.indexed_iter() {
        if *el == Pipe::Start {
            start_pos = coord;
            break;
        }
    }
    let mut visited = HashMap::from([(start_pos, Pipe::Start)]);
    let sconnect = pipes_connecting_to_start(&input, start_pos);
    let mut current_pos = sconnect[0];
    visited.insert(start_pos, figure_out_which_pipe_start_is(&sconnect));
    visited.insert(current_pos.0, current_pos.1);
    loop {
        let positions = find_connecting_pipes_p1(&input,
            input[[current_pos.0.0, current_pos.0.1]], current_pos.0);
        if positions.iter().all(|el| visited.contains_key(&el.0)) {
            break;
        }
        current_pos = *positions.iter().filter(|el| !visited.contains_key(&el.0)).next().unwrap();
        visited.insert(current_pos.0, current_pos.1);
    }
    visited
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Array2<Pipe>) -> usize {
    let start_loop = calc_start_loop(input);
    start_loop.iter().count() / 2
}

/* Decides whether or not to flip fis_inside
 * F-----J -> flip
 * F-----7 -> no flip */
fn handle_defered_pipe_symbol(defered_pipe_symbol: &mut Option<Pipe>, fis_inside: &mut bool, next: Pipe) {
    match defered_pipe_symbol {
        None => {*defered_pipe_symbol = Some(next);},
        Some(val) => {
            match val {
                    Pipe::SEBend => {
                        if next == Pipe::NWBend {
                            *fis_inside = !*fis_inside;
                        }
                    },
                    Pipe::NEBend => {
                        if next == Pipe::SWBend {
                            *fis_inside = !*fis_inside;
                        }
                    },
                    _ => unreachable!("Unexpected symbol while traversing start_loop"),
            };
            *defered_pipe_symbol = None;
        },
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Array2<Pipe>) -> u64 {
    let start_loop = calc_start_loop(input);
    let (outer_max, inner_max) = input.dim();
    let mut defered_pipe_symbol: Option<Pipe> = None;//helps with deciding edge case
    let mut ans = 0;
    for i in 0..outer_max {
        let mut fis_inside = false;
        for j in 0..inner_max {
            if start_loop.contains_key(&(i,j)) {
                println!("({}, {})", i,j);
                match input[[i,j]] {
                    Pipe::Vertical => {fis_inside = !fis_inside;},
                    Pipe::SEBend => {handle_defered_pipe_symbol(&mut defered_pipe_symbol, &mut fis_inside, input[[i,j]])},
                    Pipe::SWBend => {handle_defered_pipe_symbol(&mut defered_pipe_symbol, &mut fis_inside, input[[i,j]])},
                    Pipe::NEBend => {handle_defered_pipe_symbol(&mut defered_pipe_symbol, &mut fis_inside, input[[i,j]])},
                    Pipe::NWBend => {handle_defered_pipe_symbol(&mut defered_pipe_symbol, &mut fis_inside, input[[i,j]])},
                    Pipe::Horizontal => {},
                    _ => unreachable!("ground shouldn't be part of starting loop"),//ground,
                }
            } else {
                if fis_inside {
                    ans += 1;
                }
            }
        }
    }
    ans
}

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
        
        assert_eq!(ans1, vec![((1,2), Pipe::Horizontal),((2,1), Pipe::Vertical)]);
        assert_eq!(ans2, vec![((2,1), Pipe::NWBend),((3,0), Pipe::Vertical,)]);
        assert_eq!(ans3, vec![((4,2), Pipe::NEBend),((3,3), Pipe::Vertical)]);
    }

    #[test]
    fn test_find_connecting_pipes_p1() {
        let input = input_generator(TEST_INPUT3);
        let ans1 = find_connecting_pipes_p1(&input, Pipe::Vertical, (3,2));
        let ans2 = find_connecting_pipes_p1(&input, Pipe::SEBend, (2,2));
        //let ans3 = find_connecting_pipes_p1(&input, Pipe::SWBend, (2,3));
        //let ans4 = find_connecting_pipes_p1(&input, Pipe::NEBend, (4,2));
        assert_eq!(ans1, vec![((4,2), Pipe::NEBend), ((2,2), Pipe::SEBend)]);
        assert_eq!(ans2, vec![((2,3), Pipe::SWBend), ((3,2), Pipe::Vertical)]);
        //assert_eq!(ans3, vec![((2,2), ), ((3,3), )]);
        //assert_eq!(ans4, vec![(4,3), (3,2)]);
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

    const TEST_INPUT4: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn test_solve_day10_part2() {
        let input = input_generator(TEST_INPUT4);
        let ans = solve_part2(&input);
        assert_eq!(ans, 4);
    }
}
