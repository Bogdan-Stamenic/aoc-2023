use crate::f128_matrix_math::*;
use std::usize;
use f128::f128;
use ndarray::{prelude::*, concatenate};
use ndarray_linalg::Solve;
use nom::{
    Parser,
    IResult,
    branch::alt,
    bytes::complete::{tag,take_while1,take_while},
    combinator::all_consuming,
    sequence::{separated_pair, tuple, preceded},
    multi::separated_list1,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct HailMovement {
    pos: Array1<i64>,
    velocity: Array1<i64>,
    p_f64: Array1<f64>,
    v_f64: Array1<f64>,
    p_f128: Array1<f128>,
    v_f128: Array1<f128>,
    vdir_norm: Array1<f64>,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<HailMovement> {
    match all_consuming(separated_list1(tag("\n"), parse_hail_movement_line))
        .parse(input) {
            Ok((_,val)) => val,
            Err(e) => panic!("While parsing : {}", e),
        }
}

fn parse_hail_movement_line(input: &str) -> IResult<&str, HailMovement> {
    separated_pair(parse_3d_coords, tag(" @ "), parse_3d_coords)
        .map(|(hpos,hvel)| {
            let pos_f64 = hpos.iter().map(|x| *x as f64).collect();
            let vel_f64: Array1<f64> = hvel.iter().map(|x| *x as f64).collect();
            let pos_f128 = hpos.iter().map(|x| f128::from(*x)).collect();
            let vel_f128: Array1<f128> = hvel.iter().map(|x| f128::from(*x)).collect();
            let vdir = vel_f64.clone() / (hvel.dot(&hvel) as f64).sqrt();
            HailMovement {pos: hpos, velocity: hvel,
            p_f64: pos_f64, v_f64: vel_f64,
            p_f128: pos_f128, v_f128: vel_f128,
            vdir_norm: vdir}
        })
        .parse(input)
}

fn parse_3d_coords(input: &str) -> IResult<&str, Array1<i64>> {
    tuple((
            parse_num_to_i64,
            tag(", "),
            parse_num_to_i64,
            tag(", "),
            parse_num_to_i64,
            ))
        .map(|x| Array1::from_vec(vec![x.0,x.2,x.4]))
        .parse(input)
}

fn parse_num_to_i64(input: &str) -> IResult<&str, i64> {
    preceded(take_while(char::is_whitespace),//remove any extra preceding whitespace
    alt((
            take_while1(char::is_numeric)
            .map(|x: &str| x.parse::<i64>().expect("expected number")),
            preceded(tag("-"), take_while1(char::is_numeric))
            .map(|x: &str| -1 * x.parse::<i64>().expect("expected number")),
    ))
    )
        .parse(input)
}

/* x1 + t*v1 = x2 + s*v2
 *
 *                   | t|
 * (x2-x1) = [v1 v2] |-s|
 * b = Ax
 *                                           | t|
 * v1,v2 in R^2 --> [v1 v2]^{-1} (x2 - x1) = |-s|
 *
 * t > 0 && s < 0 => paths intersect in future
 *
 * t - s = 0 && t > 0  =>  collision!
 * x1 + t*v1 to find where collision happens
 * */
fn do_hailstones_collide_in_test_area_p1(stone1: &HailMovement, stone2: &HailMovement, cmin: f64, cmax: f64) -> bool {
    let mat_a: Array2<f64> = concatenate![Axis(1),
    stone1.v_f64.slice(s![..2]).insert_axis(Axis(1)),
    stone2.v_f64.slice(s![..2]).insert_axis(Axis(1))];
    let vec_b = (stone2.p_f64.clone() - stone1.p_f64.clone()).slice(s![..2]).to_owned();
    let sol = match mat_a.solve_into(vec_b) {
        Ok(ans) => ans,
        Err(_) => return false,
    };
    if sol[0].is_sign_positive() && sol[1].is_sign_negative() {
        /* x1 + t * v1 = collision_point */
        let collision_point = (stone1.p_f64.clone() + sol[0] * stone1.v_f64.clone())
            .slice(s![..2]).to_owned();
        if collision_point.iter().all(|x| *x >= cmin && *x <= cmax) {
            return true;
        }
    }
    false
}

/* Crossproduct as matrix-vector product with this matrix */
fn skew_crossprod_matrix(input: Array1<f128>) -> Array2<f128> {
    let mut out = Array2::from_elem((3,3), f128::from(0.));
    out[[2,1]] = input[[0]];
    out[[1,2]] = -input[[0]];
    out[[0,2]] = input[[1]];
    out[[2,0]] = -input[[1]];
    out[[1,0]] = input[[2]];
    out[[0,1]] = -input[[2]];
    out
}

/* Start with when hailstones collide with rock:
 * p0 + t * v0 = p_i + t * v_i,   for i in 1..NUM_HAILSTONES (0 is rock pos and vel)
 * -> p0 - p_i = -t * (v0 - v_i)
 *
 * Linear dependance implies:
 * -> (p0 - p_i) x (v0 - v_i) = 0  (it's a property of the cross-product)
 *
 * -> (p0 x v0) = (p_i x v0) + (p0 x v_i) + (v_i x p_i)
 * 
 * Take any three points in i (here p1,p2,p3) and equate the left sides for six
 * equations (2 eqs over 3d vectors each):
 * (p1 x v0) + (p0 x v1) + (v1 x p1) = (p2 x v0) + (p0 x v2) + (v2 x p2)
 * (p2 x v0) + (p0 x v2) + (v2 x p2) = (p3 x v0) + (p0 x v3) + (v3 x p3)
 *
 * And solve for matrix equation Ax = b, where x = [p0_x, p0_y, p0_z, v0_x, v0_y, v0_z].
 * The problem posed in the puzzle implies that if the rock collides with several hailstones,
 * then it'll collide with ALL of them. Therefore, we don't need to use all of the hailstones,
 * only enough to solve for x.
 * */
#[allow(non_snake_case)]
fn find_rock_trajectory_p2(input: &[HailMovement]) -> (Array1<f128>,Array1<f128>) {
    let mut x_vec = Array1::from_elem(6, f128::from(0.));
    for win in input.windows(3) {
        let [h0,h1,h2] = win else {unreachable!()};
        /*      | B  C |   | v0-v1  p1-p0 |  <- all submatrices as skew crossprod mats
         *  A = | D  E | = | v1-v2  p2-p1 |
         * */
        let B = skew_crossprod_matrix(h0.v_f128.clone() - h1.v_f128.clone());
        let D = skew_crossprod_matrix(h1.v_f128.clone() - h2.v_f128.clone());
        let C = skew_crossprod_matrix(h1.p_f128.clone() - h0.p_f128.clone());
        let E = skew_crossprod_matrix(h2.p_f128.clone() - h1.p_f128.clone());
        let mut A: Array2<f128> = concatenate![Axis(0),
        concatenate![Axis(1), B, C],
        concatenate![Axis(1), D, E],
        ];
        //let b_vec = concatenate![Axis(0),
        x_vec = concatenate![Axis(0),
        skew_crossprod_matrix(h1.p_f128.clone()).dot(&h1.v_f128) - skew_crossprod_matrix(h0.p_f128.clone()).dot(&h0.v_f128),
        skew_crossprod_matrix(h2.p_f128.clone()).dot(&h2.v_f128) - skew_crossprod_matrix(h1.p_f128.clone()).dot(&h1.v_f128)
        ];
        /* Use Gauß elimination to determine if A has full rank/is solveable (and partly solve) */
        let _ = f128_gauss_elim(&mut A, &mut x_vec);
        /* Not actually det_A, but it's det_A * C, so it'll still tell us if A is solveable */
        let det_a = A.diag().iter().cloned()
            .reduce(|acc,el| acc * el)
            .unwrap();
        if det_a < f128::from(1e-9) {
            continue;
        }
        match f128_back_substitution(&mut A, &mut x_vec) {
            Ok(_) => {},
            Err(_) => unreachable!("Couldn't solve linear system of equations"),
        }
        break;
    }
    let p_vec = Array1::from_shape_fn(3, |i| x_vec[i]);
    let v_vec = Array1::from_shape_fn(3, |i| x_vec[i+3]);
    (p_vec,v_vec)
}

fn count_collisions_p1(input: &[HailMovement], cmin: f64, cmax: f64) -> usize {
    let mut out = 0;
    for (i,foo) in input.iter().enumerate() {
        for bar in input[i+1..].iter() {
            out += if do_hailstones_collide_in_test_area_p1(foo, bar, cmin, cmax) {1} else {0};
        }
    }
    out
}

/* Find if hailstone paths intersect in while ignoring z-params in test area */
#[aoc(day24,part1)]
pub fn solve_day24_p1(input: &[HailMovement]) -> usize {
    count_collisions_p1(input, 200_000_000_000_000f64, 400_000_000_000_000f64)
}


#[aoc(day24,part2)]
pub fn solve_day24_p2(input: &[HailMovement]) -> u128 {
    let (p, v) = find_rock_trajectory_p2(input);
    println!("p : {}\nv : {}\n", p, v);
    p.iter().sum::<f128>().into()
}

#[cfg(test)]
mod test {
    use super::*;
    // Part 1 : px py pz @ vx vy vz
    const TEST_INPUT: &str =
"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn day24_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.len(), 5);
        assert_eq!(input[0].velocity[0], -2);
    }

    #[test]
    fn day24_solve_p1_1() {
        let input = input_generator(TEST_INPUT);
        let ans = count_collisions_p1(&input, 7f64, 27f64);
        assert_eq!(ans, 2);
    }

    #[test]
    fn day24_solve_p1_2() {
        const INPUT: &str =
"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2";
        let input = input_generator(INPUT);
        let ans = count_collisions_p1(&input, 7f64, 27f64);
        assert_eq!(ans, 1);
    }

    #[test]
    fn day24_solve_p1_3() {
        const INPUT: &str =
"19, 13, 30 @ -2,  1, -2
20, 25, 34 @ -2, -2, -4";
        let input = input_generator(INPUT);
        let ans = count_collisions_p1(&input, 7f64, 27f64);
        assert_eq!(ans, 1);
    }

    #[test]
    fn day24_solve_p2() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_day24_p2(&input);
        println!("ans : {}", ans);
        assert_eq!(ans, 47);
    }

}
