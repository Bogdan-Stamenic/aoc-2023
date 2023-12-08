use std::iter::zip;

use nom::{
    Parser,
    IResult,
    combinator::all_consuming,
    bytes::complete::{tag, take_while1},
    multi::many1, sequence::{separated_pair,preceded},
};

#[derive(Debug,PartialEq,Eq)]
#[allow(dead_code)]
pub struct BoatRacePair {
    time: u64,
    distance: u64,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<BoatRacePair> {
    match all_consuming(separated_pair(parse_times, tag("\n"), parse_distances))
        .map(|(times,dists)| {
            zip(times, dists).into_iter()
                .map(|(x,y)| BoatRacePair {time: x, distance: y})
                .collect()
        })
        .parse(input)
    {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    }
}

fn parse_times(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("Time:"), parse_nums)
        .parse(input)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("Distance:"), parse_nums)
        .parse(input)
}

fn parse_nums(input: &str) -> IResult<&str, Vec<u64>> {
    many1(preceded(take_while1(char::is_whitespace), parse_one_num))
        .parse(input)
}

fn parse_one_num(input: &str) -> IResult<&str, u64> {
    take_while1(char::is_numeric)
        .map(|x: &str| x.to_string().parse::<u64>().unwrap())
        .parse(input)
}

fn calc_num_possible_winning_boat_button_push_times(input: &BoatRacePair) -> u64 {
    (0..input.time).into_iter()
        .map(|num| {
            num * (input.time - num)
        })
        .filter(|x| x > &input.distance)
        .count() as u64
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[BoatRacePair]) -> u64 {
    input.iter()
        .map(|el| {
            calc_num_possible_winning_boat_button_push_times(el)
        })
    .reduce(|acc,el| acc * el).unwrap()
}

fn join_nums_for_p2(input: &[BoatRacePair]) -> BoatRacePair {
    let new_time: String = input.iter()
        .map(|pair| pair.time.to_string())
        .reduce(|mut acc,el| {
            acc.push_str(el.as_str());
            acc
        }).unwrap();
    let new_time = new_time.parse::<u64>().unwrap();
    let new_distance = input.iter()
        .map(|pair| pair.distance.to_string())
        .reduce(|mut acc,el| {
            acc.push_str(el.as_str());
            acc
        }).unwrap();
    let new_distance = new_distance.parse::<u64>().unwrap();
    BoatRacePair {
        time: new_time,
        distance: new_distance
    }
}

/* Solve using quadratic formula on f(x) = x^2 - x*t + d
 * All nums between the zeros are answers.
 * x : button push time
 * t : maximum time allowed
 * d : distance to beat
 * */
fn calc_num_possible_winning_boat_button_push_times_p2(input: &BoatRacePair) -> u64 {
    let BoatRacePair {time: t, distance: d} = input;
    let discrimianant = ((t * t - 4 * d) as f64).sqrt();
    let x1 = (-(*t as f64) + discrimianant) / 2.0; 
    let x2 = (-(*t as f64) - discrimianant) / 2.0; 
    if x1 < x2 {
        return (x2.ceil() - x1.ceil()) as u64;
    }
    (x1.ceil() - x2.ceil()) as u64
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[BoatRacePair]) -> u64 {
    let new_input = join_nums_for_p2(input);
    calc_num_possible_winning_boat_button_push_times_p2(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_day6_parser() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.len(),3);
        assert_eq!(input[0].time,7);
        assert_eq!(input[2].distance,200);
    }
    
    #[test]
    fn test_function_with_the_long_name() {
        let input = BoatRacePair {time: 7, distance: 9};
        let ans = calc_num_possible_winning_boat_button_push_times(&input);
        assert_eq!(ans,4);
    }

    #[test]
    fn test_join_nums_for_p2() {
        let input = input_generator(TEST_INPUT);
        let ans = join_nums_for_p2(&input);
        assert_eq!(ans, BoatRacePair {time: 71530, distance: 940200})
    }
    
    //#[test]
    //fn test_solve_day6p1() {
    //}

    #[test]
    fn test_solve_day6p2() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part2(&input);
        assert_eq!(ans,71503);
    }
}
