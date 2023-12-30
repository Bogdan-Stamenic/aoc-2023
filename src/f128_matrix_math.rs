use ndarray::{prelude::*,Zip};
use f128::f128;
use num::Float;

#[derive(Debug)]
pub struct DetFail {}

#[derive(Debug,PartialEq)]
pub struct GaussFail {}

/* with rule of sarrus */
/* with rule of sarrus */
#[allow(non_snake_case)]
fn f128_3x3_mat_det(A: Array2<f128>) -> Result<f128,DetFail> {
    if A.shape() != [3,3] {
        return Err(DetFail {  });
    }
    let det =
        A[[0,0]]*A[[1,1]]*A[[2,2]]// aei
        + A[[0,1]]*A[[1,2]]*A[[2,0]]// bfg
        + A[[0,2]]*A[[1,0]]*A[[2,1]]// cdh
        - A[[0,2]]*A[[1,1]]*A[[2,0]]// ceg
        - A[[0,1]]*A[[1,0]]*A[[2,2]]// bdi
        - A[[0,0]]*A[[1,2]]*A[[2,1]]// afh
        ;
    if det < f128::from(1e-9) {
        return Err(DetFail {  });
    }
    Ok(det)
}

/* Seems okay? I remember this from Uni, but I can't find a source anywhere online */
#[allow(non_snake_case)]
pub fn f128_mat_det(A: &Array2<f128>, B: &Array2<f128>, C: &Array2<f128>, D: &Array2<f128>)
    -> Result<f128, DetFail> {
    /* | A  B |
     * | C  D |
     * */
    let det_A = f128_3x3_mat_det(A.clone()).unwrap_or(f128::from(0));
    let det_B = f128_3x3_mat_det(B.clone()).unwrap_or(f128::from(0));
    let det_C = f128_3x3_mat_det(C.clone()).unwrap_or(f128::from(0));
    let det_D = f128_3x3_mat_det(D.clone()).unwrap_or(f128::from(0));
    let out = det_A * det_D - det_C * det_B;
    if out < f128::from(1e-9) {
        return Err(DetFail {  });
    }
    Ok(out)
}

#[allow(non_snake_case)]
pub fn f128_gauss_elim(A: &mut Array2<f128>, b: &mut Array1<f128>) -> Result<(), GaussFail>{
    let Delta: f128 = f128::from(1e-9);
    let mut row_pivot = 0;
    let mut col_pivot = 0;
    let foo = A.shape().to_vec();
    let outer_max = foo[0];
    let inner_max = foo[1];
    if b.len() != outer_max {
        return Err(GaussFail{});
    }
    loop {
        if (row_pivot == outer_max-1) && (col_pivot == inner_max-1) {
            break;
        }
        let i_max: usize = A
            .slice(s![row_pivot..outer_max,col_pivot])
            .iter()
            .enumerate()
            .max_by(|x,y| {
                let foo = y.1.abs();
                x.1.abs().partial_cmp(&foo).unwrap()
            })
            .unwrap()// (usize,&f128)
            .0 + row_pivot;
        if A[[i_max,col_pivot]].abs() < Delta {
            /* no pivot in this column, move to next column */
            col_pivot += 1;
        } else {
            /* swap rows i_max and row_pivot */
            ndarray2_swap_rows(A, row_pivot, i_max);
            ndarray1_swap_rows(b, row_pivot, i_max);
            for i in (row_pivot+1)..outer_max {
                let factor = A[[i,col_pivot]] / A[[row_pivot,col_pivot]];
                A[[i,col_pivot]] = f128::from(0);
                b[i] = b[i] - b[row_pivot] * factor;
                for j in (col_pivot+1)..inner_max {
                    A[[i,j]] = A[[i,j]] - A[[row_pivot,j]] * factor;
                }
            }
            row_pivot += 1;
            col_pivot += 1;
        }
    }
    Ok(())
}

#[allow(non_snake_case)]
pub fn f128_back_substitution(A: &mut Array2<f128>, b: &mut Array1<f128>) -> Result<(), GaussFail> {
    let [outer_max,_] = A.shape() else {unreachable!()};
    if b.len() != *outer_max {
        return Err(GaussFail{});
    }
    for i in (0..*outer_max).rev() {
        b[i] = b[i] / A[[i,i]];
        A[[i,i]] = f128::from(1);
        for j in 0..i {
            b[j] = b[j] - A[[j,i]] * b[i];
            A[[j,i]] = f128::from(0);
        }
    }
    Ok(())
}

fn ndarray2_swap_rows(matrix: &mut Array2<f128>, i: usize, j: usize) {
    if i == j {
        return;
    }
    let mut it = matrix.axis_iter_mut(Axis(0));
    Zip::from(it.nth(i).unwrap())
        .and(it.nth(j-(i+1)).unwrap())
        .for_each(|x, y| std::mem::swap(x, y));
}

fn ndarray1_swap_rows(matrix: &mut Array1<f128>, i: usize, j: usize) {
    if i == j {
        return;
    }
    let mut it = matrix.axis_iter_mut(Axis(0));
    Zip::from(it.nth(i).unwrap())
        .and(it.nth(j-(i+1)).unwrap())
        .for_each(|x, y| std::mem::swap(x, y));
}


#[cfg(test)]
mod test {
    use num::Float;
    use super::*;

    #[test]
    fn f128_det_6x6_eye() {
        let mat_a = Array2::eye(3);
        let mat_b = Array2::zeros([3,3]);
        let mat_d = Array2::eye(3);
        let ans = f128_mat_det(&mat_a, &mat_b, &mat_b, &mat_d).unwrap();
        assert!((ans - f128::from(1)).abs() < f128::from(1e-9))
    }

    #[test]
    fn f128_det_diag() {
        let mat_a = Array2::from_shape_vec([3,3],
            vec![
            f128::from(1),f128::from(0),f128::from(0),
            f128::from(0),f128::from(2),f128::from(0),
            f128::from(0),f128::from(0),f128::from(3),
        ]).unwrap();
        let mat_b = Array2::zeros([3,3]);
        let mat_d = Array2::from_shape_vec([3,3],
            vec![
            f128::from(1),f128::from(0),f128::from(0),
            f128::from(0),f128::from(2),f128::from(0),
            f128::from(0),f128::from(0),f128::from(3),
        ]).unwrap();
        let ans = f128_mat_det(&mat_a, &mat_b, &mat_b, &mat_d).unwrap();
        assert!((ans - f128::from(36)).abs() < f128::from(1e-9))
    }

    #[ignore = "only pretty prints"]
    #[test]
    fn f128_swap_rows() {
        let mut matrix = Array2::from_shape_fn((3,3), |(i,j)| f128::from(i+j));
        println!("{}", matrix);
        ndarray2_swap_rows(&mut matrix, 1, 2);
        println!("{}", matrix);
    }

    #[test]
    fn f128_gauss_elim_test() {
        let mut mat_a = Array2::from_shape_vec((3,3),
        vec![
        f128::from(2),f128::from(1),f128::from(-1),
        f128::from(-3),f128::from(-1),f128::from(2),
        f128::from(-2),f128::from(1),f128::from(2),
        ]).unwrap();
        let mut vec_b = Array1::from_shape_vec(3,
            vec![
            f128::from(8),f128::from(-11),f128::from(-3)
            ]).unwrap();
        let ans = f128_gauss_elim(&mut mat_a, &mut vec_b);
        println!("A :\n{} \n\nb :\n{}",mat_a,vec_b);
        assert_eq!(ans, Ok(()));
    }

    #[test]
    fn f128_back_substitution_test() {
        let mut mat_a = Array2::from_shape_vec((3,3),
        vec![
        f128::from(-3),f128::from(-1),f128::from(2),
        f128::from(0),f128::from(1.66667),f128::from(0.666667),
        f128::from(0),f128::from(0),f128::from(0.2),
        ]).unwrap();
        let mut vec_b = Array1::from_shape_vec(3,
            vec![
            f128::from(-11),f128::from(4.33333),f128::from(-0.2),
            ]).unwrap();
        let ans = f128_back_substitution(&mut mat_a, &mut vec_b);
        println!("A :\n{}\nb :\n {}", mat_a, vec_b);
        println!("{}", (vec_b[0] - f128::from(2)).abs());
        println!("{}", (vec_b[1] - f128::from(3)).abs());
        println!("{}", (vec_b[2] - f128::from(-1)).abs());
        assert_eq!(ans, Ok(()));
        assert!((vec_b[0] - f128::from(2)).abs() < f128::from(1e-6));
        assert!((vec_b[1] - f128::from(3)).abs() < f128::from(1e-6));
        assert!((vec_b[2] - f128::from(-1)).abs() < f128::from(1e-6));
    }
}
