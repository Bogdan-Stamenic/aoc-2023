use ndarray::*;
use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::{all_consuming, value},
    bytes::complete::tag,
    multi::{separated_list1, many1},
};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Space {
    Empty,
    Galaxy
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Array2<Space> {
    let parse_vec = match all_consuming(separated_list1(tag("\n"), parse_galaxy_line))
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

fn parse_galaxy_line(input: &str) -> IResult<&str, Array1<Space>> {
    many1(parse_one_space_unit)
        .map(|v| Array1::from_shape_vec(v.len(), v).unwrap())
        .parse(input)
}

fn parse_one_space_unit(input: &str) -> IResult<&str, Space> {
    alt((
            value(Space::Galaxy, tag("#")),
            value(Space::Empty, tag(".")),
            ))
        .parse(input)
}

fn find_empty_rows(input: &Array2<Space>) -> Vec<usize> {
    let mut empty_row_lst = Vec::<usize>::new();
    for (idx, row) in input.outer_iter().enumerate() {
        if row.iter().all(|el| *el != Space::Galaxy) {
            empty_row_lst.push(idx);
        }
    }
    empty_row_lst
}

fn find_empty_columns(input: &Array2<Space>) -> Vec<usize> {
    let mut empty_column_lst = Vec::<usize>::new();
    for (idx, column) in input.axis_iter(Axis(1)).enumerate() {
        if column.iter().all(|el| *el != Space::Galaxy) {
            empty_column_lst.push(idx);
        }
    }
    empty_column_lst
}

fn expand_input_p1(input: &Array2<Space>) -> Array2<Space> {
    let empty_row_lst = find_empty_rows(input);
    let empty_column_lst = find_empty_columns(input);
    let mut out = input.clone();
    let mut counter = 0;
    for idx in empty_row_lst.into_iter() {
        let i = idx + counter;
        out = concatenate![
            Axis(0), out.slice(s![0..i+1, ..]),
            out.slice(s![i..i+1, ..]),
            out.slice(s![i+1.., ..])
        ];
        counter += 1;
    }
    counter = 0;
    for idx in empty_column_lst.into_iter() {
        let i = idx + counter;
        out = concatenate![
            Axis(1), out.slice(s![.., 0..i+1]),
            out.slice(s![.., i..i+1]),
            out.slice(s![.., i+1..])
        ];
        counter += 1;
    }
    out
}

fn input_to_coord_list(input: &Array2<Space>) -> Vec<(usize,usize)> {
    input.indexed_iter()
        .filter(|(_,el)| **el == Space::Galaxy)
        .map(|(crds,_)| crds)
        .collect()
}

fn manhatten_dist(x1: (usize,usize), x2: (usize,usize)) -> u64 {
    let ans: i64 = (x1.0 as i64 - x2.0 as i64).abs() + (x1.1 as i64 - x2.1 as i64).abs();
    ans as u64
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Array2<Space>) -> u64 {
    let expanded_input = expand_input_p1(input);
    let mut galaxy_coords = input_to_coord_list(&expanded_input);
    let mut ans: u64 = 0;
    loop {
        match galaxy_coords.pop() {
            Some(x0) => {
                let inc = galaxy_coords.iter()
                    .map(|crd| manhatten_dist(x0, *crd))
                    .sum::<u64>();
                ans += inc;
            },
            None => break,
        }
    }
    ans
}

#[inline]
fn calc_galaxy_dist(x0: (usize,usize), x1: (usize,usize),
    empty_rows_lst: &[usize], empty_columns_lst: &[usize],
    empty_space_size: usize) -> u64 {
    let expanded_row_count = empty_rows_lst.iter()
        .filter(|idx| {
            (**idx > x0.0 && **idx < x1.0)
                || (**idx < x0.0 && **idx > x1.0)
        })
    .count();
    let expanded_column_count = empty_columns_lst.iter()
        .filter(|idx| {
            (**idx > x0.1 && **idx < x1.1)
                || (**idx < x0.1 && **idx > x1.1)
        })
    .count();
    let expanded_space = expanded_column_count + expanded_row_count;
    manhatten_dist(x0, x1) + (expanded_space * (empty_space_size - 1)) as u64
}

fn calc_sum_of_galaxy_dists(input: &Array2<Space>, factor: usize) -> u64 {
    let empty_rows_lst = find_empty_rows(input);
    let empty_columns_lst = find_empty_columns(input);
    let mut galaxy_coords = input_to_coord_list(input);
    let mut ans: u64 = 0;
    loop {
        match galaxy_coords.pop() {
            Some(x0) => {
                let inc = galaxy_coords.iter()
                    .map(|crd| calc_galaxy_dist(x0, *crd, &empty_rows_lst, &empty_columns_lst, factor))
                    .sum::<u64>();
                ans += inc;
            },
            None => break,
        }
    }
    ans
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Array2<Space>) -> u64 {
    calc_sum_of_galaxy_dists(input, 1_000_000usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

      #[test]
      fn day11_find_empty_rows() {
          let input = input_generator(TEST_INPUT);
          let ans = find_empty_rows(&input);
          assert_eq!(ans, vec![3,7]);
      }
      
      #[test]
      fn day11_find_empty_columns() {
          let input = input_generator(TEST_INPUT);
          let ans = find_empty_columns(&input);
          assert_eq!(ans, vec![2,5,8]);
      }

      #[test]
      fn day11_expand_space_p1() {
          let input = input_generator(TEST_INPUT);
          let ans = expand_input_p1(&input);
          assert_eq!(ans.dim(), (12,13));
          assert_eq!(ans[[10,9]], Space::Galaxy);
      }

      #[test]
      fn day11_solve_day11_p1() {
          let input = input_generator(TEST_INPUT);
          let ans = solve_part1(&input);
          assert_eq!(ans, 374);
      }

      #[test]
      fn test_calc_galaxy_dist() {
          let empty_rows_lst = vec![3,7];
          let empty_columns_lst = vec![2,5,8];
          let ans1 = calc_galaxy_dist((9,0), (9,4), &empty_rows_lst, &empty_columns_lst, 2);
          let ans2 = calc_galaxy_dist((9,0), (9,4), &empty_rows_lst, &empty_columns_lst, 3);
          let ans3 = calc_galaxy_dist((5,1), (9,4), &empty_rows_lst, &empty_columns_lst, 2);
          assert_eq!(ans1, 5);
          assert_eq!(ans2, 6);
          assert_eq!(ans3, 9);
      }

      #[test]
      fn day11_calc_sum_of_galaxy_dists() {
          let input = input_generator(TEST_INPUT);
          let ans1 = calc_sum_of_galaxy_dists(&input, 10);
          let ans2 = calc_sum_of_galaxy_dists(&input, 100);
          let ans3 = calc_sum_of_galaxy_dists(&input, 2);
          assert_eq!(ans1, 1030);
          assert_eq!(ans2, 8410);
          assert_eq!(ans3, 374);
      }

//    #[test]
//    fn test_solve_day11_p2() {
//}
}
