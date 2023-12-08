use core::panic;
use std::collections::VecDeque;
use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::all_consuming,
    bytes::complete::{tag, take_while1},
    multi::{separated_list1, many_till, many1}, sequence::{separated_pair, preceded, tuple},
};

#[derive(Clone,Debug)]
#[allow(dead_code)]
pub struct Almanac {
    seeds: Vec<i64>,
    almanac_maps: Vec<AlmanacMap>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[allow(dead_code)]
pub struct AlmanacMap {
    source_name: String,
    destination_name: String,
    almanac_nums: Vec<(i64,i64,i64)>,
}

#[derive(Debug,Clone,PartialEq)]
struct SeedsFormatError;

#[derive(Debug,PartialEq)]
enum SplitStatus {
    Split(((i64,i64), VecDeque<(i64,i64)>)),
    NoSplit,
    DoNothing,
}

impl AlmanacMap {
    fn apply_map_for_part1(&self, input: i64) -> i64 {
        match self.almanac_nums.iter()
            .map(|(d,s,l)| (*d, *s, *l))
            .filter(|(_,s,l)| {
                ((input - s) >= 0) && ((s + l - input) > 0)
            })
        .map(|(d,s,_)| input + (d - s))
            .next() {
                Some(var) => var,
                None => input
            }
    }

    fn apply_map_for_part2(&self, intervals: Vec<(i64,i64)>) -> Vec<(i64,i64)>{
        let mut out = Vec::<(i64,i64)>::new();
        for foo in intervals {
            out.append(&mut self.split_and_map(foo))
        }
        out
    }

    fn split_and_map(&self, (a,b): (i64,i64)) -> Vec<(i64,i64)> {
        let intersecting_mappings = self.almanac_nums.iter()
            .filter(|(_,src,len)| {
                (a < *src + *len) && (b > *src)
            })
            .collect::<Vec<_>>();
        if intersecting_mappings.is_empty() {
            vec![(a,b)]
        } else {
            let mut queue = VecDeque::from([(a, b)]);
            let mut out = Vec::<(i64,i64)>::new();
            loop {
                let next_interval = match queue.pop_front() {//handle queue being empty
                    Some(val) => val,
                    None => break,
                };
                for (dest,src,len) in &intersecting_mappings {
                    match AlmanacMap::split_interval_for_mapping(next_interval, (*src,*src + *len)) {
                        SplitStatus::Split(((e,f),mut v_app)) => {
                            queue.append(&mut v_app);
                            let diff = dest - src;
                            out.push((e + diff,f + diff));
                        },
                        SplitStatus::NoSplit => {
                            let (e,f) = next_interval;
                            let diff = dest - src;
                            out.push((e + diff, f + diff));
                        },
                        SplitStatus::DoNothing => {},
                    }
                }
            }
            while !queue.is_empty() {//Empty queue
                out.push(queue.pop_front().unwrap());
            }
            out
        }
    }

    /* Case 1: (c > a) ^ (d < b) => split in three
     * Case 2: (c > a) ^ (d >= b) ^ (c < b) => split in two
     * Case 3: (c <= a) ^ (d < b) ^ (d > a) => split in two
     * Case 4: (c <= a) ^ (d >= b) => do nothing
     * */
    fn split_interval_for_mapping(interval_in: (i64,i64), mapping_src_interval: (i64,i64)) -> SplitStatus {
        let (a, b) = interval_in;
        let (c, d) = mapping_src_interval;
        if (a < d) && (b > c) {//check if they overlap
            if c > a {
                if d < b {
                    return SplitStatus::Split((mapping_src_interval, VecDeque::from([(a,c),(d,b)])));
                } else {
                    return SplitStatus::Split(((c,b), VecDeque::from([(a,c)])));
                }
            } else {
                if d < b {
                    return SplitStatus::Split(((a,d), VecDeque::from([(d,b)])));
                } else {
                    return SplitStatus::NoSplit;
                }
            }
        } else {
            return SplitStatus::DoNothing;
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanac {
    match all_consuming(tuple((parse_seeds, parse_almanac_maps)))
        .map(|(sds, maps)| Almanac{seeds: sds, almanac_maps: maps})
        .parse(input)
    {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    many_till(parse_seeds_line, tag("\n\n"))
        .map(|(vec,_)| vec.into_iter().flatten().collect())
        .parse(input)
}

fn parse_seeds_line(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(alt((tag("seeds: "),tag("\n"))), separated_list1(tag(" "), parse_num))
        .parse(input)
}

fn parse_num(input: &str) -> IResult<&str, i64> {
    let (out,num) = take_while1(char::is_numeric)(input)?;
    let num = num.parse::<i64>().unwrap();
    Ok((out,num))
}

fn parse_almanac_maps(input: &str) -> IResult<&str, Vec<AlmanacMap>> {
    many1(parse_one_almanac_map)(input)
}

fn parse_one_almanac_map(input: &str) -> IResult<&str, AlmanacMap> {
    separated_pair(parse_map_source_destin, tag(":\n"), parse_mappings)
        .map(|((src,dest),mappings)| AlmanacMap{source_name: src, destination_name: dest, almanac_nums: mappings})
        .parse(input)
}

fn parse_map_source_destin(input: &str) -> IResult<&str, (String,String)> {
    tuple((
            take_while1(char::is_alphabetic),
            tag("-to-"),
            take_while1(char::is_alphabetic),
            tag(" map")
    ))
        .map(|(source,_,destin,_): (&str,_,&str,_)| {
            (source.to_string(), destin.to_string())
        })
    .parse(input)
}

fn parse_mappings(input: &str) -> IResult<&str, Vec<(i64,i64,i64)>> {
    alt((
            many_till(parse_alm_mapping_line, tag("\n\n"))
            .map(|(v,_)| v),
            many1(parse_alm_mapping_line)
    ))
        .parse(input)
}

fn parse_alm_mapping_line(input: &str) -> IResult<&str, (i64,i64,i64)> {
    tuple((
            parse_mapping_num,
            parse_mapping_num,
            parse_mapping_num
            ))
        .parse(input)
}

fn parse_mapping_num(input: &str) -> IResult<&str, i64> {
    alt((
            parse_num,
            preceded(tag("\n"), parse_num),
            preceded(tag(" "), parse_num),
    ))
        .parse(input)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Almanac) -> i64 {
    let Almanac{seeds: sds, almanac_maps: alm_maps} = input.clone();
    match sds.into_iter()
        .map(|seed| {
            let mut out = seed;
            for map in alm_maps.iter() {
                out = AlmanacMap::apply_map_for_part1(map,out as i64);
            }
            out
        })
    .min() {
        Some(val) => val,
        None => 0,
    }
}

fn seed_ranges_for_part2(seeds_ranges: Vec<i64>) -> Result<Vec<(i64, i64)>, SeedsFormatError> {
    if seeds_ranges.len() % 2 == 1 {return Err(SeedsFormatError)}
    let val = seeds_ranges.chunks_exact(2)
        .map(|chunk| {
            let mut it = chunk.into_iter();
            let start = *it.next().unwrap();
            let len = *it.next().unwrap();
            (start, start + len)
        })
    .collect();
    Ok(val)
}

fn map_seed_range_to_smallest_location_p2(seed_range: &[(i64,i64)], alm_maps: &[AlmanacMap]) -> i64 {
    let mut ranges = seed_range.to_vec();
    for amap in alm_maps {
        ranges = amap.apply_map_for_part2(ranges);
    }
    match ranges.iter()
        .map(|(a,_)| a)
        .min() {
            Some(val) => *val,
            None => 1 << 62,
        }
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Almanac) -> i64 {
    let Almanac{seeds: sds, almanac_maps: alm_maps} = input.clone();
    let sds = match seed_ranges_for_part2(sds) {
        Ok(val) => val,
        Err(_) => panic!("Something went wrong"),
    };
    map_seed_range_to_smallest_location_p2(&sds, &alm_maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        const INPUT: &str = "seeds: 79 14 55 13
42 69 5

seed-to-soil map:";
        let ans = parse_seeds(INPUT);
        assert_eq!(ans, Ok(("seed-to-soil map:", vec![79,14,55,13,42,69,5])))
    }

    #[test]
    fn test_parse_one_almanac_map_1() {
        const INPUT: &str = "seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:";
        let ans = parse_one_almanac_map(INPUT);
        assert_eq!(ans,
            Ok(("soil-to-fertilizer map:",
                    AlmanacMap {
                        source_name: "seed".to_string(),
                        destination_name: "soil".to_string(),
                        almanac_nums: vec![(50,98,2), (52,50,48)]
                    }
            ))
        )
    }

    #[test]
    fn test_parse_one_almanac_map_2() {
        const INPUT: &str = "seed-to-soil map:
50 98 2
52 50 48";
        let ans = parse_one_almanac_map(INPUT);
        assert_eq!(ans,
            Ok(("",
                    AlmanacMap {
                        source_name: "seed".to_string(),
                        destination_name: "soil".to_string(),
                        almanac_nums: vec![(50,98,2), (52,50,48)]
                    }
            ))
        )
    }

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_day5_parser() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.seeds, vec![79,14,55,13]);
        assert_eq!(input.almanac_maps.len(), 7);
        assert_eq!(input.almanac_maps[5],
            AlmanacMap{
                source_name: "temperature".to_string(),
                destination_name: "humidity".to_string(),
                almanac_nums: vec![(0,69,1), (1,0,69)]
            });
    }

    #[test]
    fn test_solve_day5p1_1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 35)
    }

    //#[test]
    //fn test_solve_day5_p2() {
    //    let input = input_generator(TEST_INPUT);
    //    let ans = solve_part2(&input);
    //    assert_eq!(ans, 46)
    //}

    #[test]
    fn test_interval_splitting() {
        let ans1 = AlmanacMap::split_interval_for_mapping((2,42), (16,25));
        let ans2 = AlmanacMap::split_interval_for_mapping((10,20), (18,25));
        let ans3 = AlmanacMap::split_interval_for_mapping((46,57), (56,93));
        assert_eq!(ans1, SplitStatus::Split(((16,25), VecDeque::from([(2,16),(25,42)]))));
        assert_eq!(ans2, SplitStatus::Split(((18,20), VecDeque::from([(10,18)]))));
        assert_eq!(ans3, SplitStatus::Split(((56,57), VecDeque::from([(46,56)]))));
    }
}
