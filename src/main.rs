#[macro_use(s)]
extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;

#[macro_use]
extern crate approx;

use ndarray::*;
use ndarray_rand::RandomExt;

fn main() {
    println!("Please run `cargo test` to run examples");
}

/*
 * 100 Exercises for Numpy:
 * http://www.labri.fr/perso/nrougier/teaching/numpy.100/
 */


// 3. Create a null vector of size 10 (★☆☆)
#[test]
fn exercise3() {
    let arr = Array1::<f64>::zeros((10));
    assert_eq!(arr.len(), 10);
    assert_eq!(arr.dim(), (10));
}

// 6. Create a null vector of size 10 but the fifth value which is 1 (★☆☆)
#[test]
fn exercise6() {
    let mut arr = Array1::<f64>::zeros((10));
    arr[4] = 1.0;
    assert_eq!(arr[4], 1.0);
}

// 7. Create a vector with values ranging from 10 to 49 (★☆☆)
#[test]
fn exercise7() {
    let arr = Array::range(10.0, 50.0, 1.0);
    assert_eq!(arr.len(), 40);
    assert_eq!(arr[0], 10.0);
    assert_eq!(arr[39], 49.0);
}

// 8. Reverse a vector (first element becomes last) (★☆☆)
#[test]
fn exercise8() {
    let arr: Array1<f64> = ndarray::Array::range(0.0, 50.0, 1.0);
    let reverse = arr.slice(s!(..;-1));
    assert_eq!(reverse.len(), 50);
    assert_eq!(reverse[0], 49.0);
    assert_eq!(reverse[49], 0.0);
}
#[test]
fn exercise8b() {
    let arr: Array1<f64> = Array::linspace(49.0, 0.0, 50);
    assert_eq!(arr.len(), 50);
    assert_eq!(arr[0], 49.0);
    assert_eq!(arr[49], 0.0);
}

// 9. Create a 3x3 matrix with values ranging from 0 to 8 (★☆☆)
#[test]
fn exercise9() {
    // TODO: can we do otherwise?
    let matrix: Array2<f64> = Array::range(0.0, 9.0, 1.0).into_shape((3,3)).unwrap();
    assert_eq!(matrix.dim(), (3,3));
    assert_eq!(matrix[(0,0)], 0.0);
    assert_eq!(matrix[(1,0)], 3.0);
    assert_eq!(matrix[(2,2)], 8.0);
}

// 10. Find indices of non-zero elements from [1,2,0,0,4,0] (★☆☆)
#[test]
fn exercise10() {
 let arr = Array::from_vec(vec![1,2,0,0,4,0]);
 let indices: Vec<usize> = arr.indexed_iter().filter(|&(_,x)| { *x != 0 }).map(|(i,_)| i).collect();
 assert_eq!(indices, vec![0,1,4]);
}
#[test]
fn exercise10b() {
 let arr = Array::from_vec(vec![1,2,0,0,4,0]);
 let indices: Vec<usize> = arr.indexed_iter().filter_map(|(i,x)| {
     if *x != 0 { return Some(i); } else { return None }
 }).collect();
 assert_eq!(indices, vec![0,1,4]);
}

// 11. Create a 3x3 identity matrix (★☆☆)
#[test]
fn exercise11() {
    let arr: Array2<f64> = Array::eye(3);
    assert_eq!(arr[(0,0)], 1.0);
    assert_eq!(arr[(1,1)], 1.0);
    assert_eq!(arr[(2,2)], 1.0);
    assert_eq!(arr[(1,0)], 0.0);
}

// 12. Create a 3x3x3 array with random values (★☆☆)
#[test]
fn exercise12() {
    // Watch out: if you get error:
    // "trait `rand::distributions::Normal: rand::distributions::IndependentSample<_>` not satisfied"
    // make sure your Cargo.toml includes crate rand=0.3
    let arr = Array::random((3, 3, 3 as usize),
                            rand::distributions::normal::Normal::new(0.0, 1.0));

    assert_eq!(arr.len(), 27);
    assert_ne!(arr[(0,0,0)], arr[(2,2,2)]);
}

// 13. Create a 10x10 array with random values and find the minimum and maximum values (★☆☆)
#[test]
fn exercise13() {
    // Watch out: if you get error:
    // "trait `rand::distributions::Normal: rand::distributions::IndependentSample<_>` not satisfied"
    // make sure your Cargo.toml includes crate rand=0.3
    let arr = Array::random((10, 10, 3 as usize),
                            rand::distributions::normal::Normal::new(0.0, 5.0));
    let inf = 0.0/0.0;
    let min = arr.fold(inf,  |r, x| { x.min(r) });
    let max = arr.fold(-inf, |r, x| { x.max(r) });
    assert!(min < max);
}

// 14. Create a random vector of size 30 and find the mean value (★☆☆)
#[test]
fn exercise14() {
    let arr: Array1<f64> = Array::random((30), rand::distributions::normal::Normal::new(0.0, 10.0));
    let mean = arr.iter().fold(0.0, |a, x| a + x / arr.len() as f64);
    let sum: f64 = arr.iter().sum();
    relative_eq!(mean, sum / 30.0);
}

