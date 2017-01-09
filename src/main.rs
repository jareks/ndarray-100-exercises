#[macro_use(s)]
extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;

use ndarray::*;
use ndarray_rand::RandomExt;

fn main() {
    println!("Please run `cargo test` to run examples");
}

/*
 * 100 Exercises for Numpy:
 * http://www.labri.fr/perso/nrougier/teaching/numpy.100/
 */

#[test]
fn exercise3() {
    let arr = Array1::<f64>::zeros((10));
    assert_eq!(arr.len(), 10);
    assert_eq!(arr.dim(), (10));
}

#[test]
fn exercise5() {
    let mut arr = Array1::<f64>::zeros((10));
    arr[4] = 1.0;
    assert_eq!(arr[4], 1.0);
}

#[test]
fn exercise6() {
    let arr = Array::range(10.0, 50.0, 1.0);
    assert_eq!(arr.len(), 40);
    assert_eq!(arr[0], 10.0);
    assert_eq!(arr[39], 49.0);
}

// TODO: add more solutions for exercise 7
#[test]
fn exercise7() {
    let arr: Array1<f64> = ndarray::Array::range(0.0, 50.0, 1.0);
    let reverse = arr.slice(s!(..;-1));
    assert_eq!(reverse.len(), 50);
    assert_eq!(reverse[0], 49.0);
    assert_eq!(reverse[49], 0.0);
}
#[test]
fn exercise7b() {
    let arr: Array1<f64> = Array::linspace(49.0, 0.0, 50);
    assert_eq!(arr.len(), 50);
    assert_eq!(arr[0], 49.0);
    assert_eq!(arr[49], 0.0);
}

#[test]
fn exercise8() {
    // TODO: can we do otherwise?
    let matrix: Array2<f64> = Array::range(0.0, 9.0, 1.0).into_shape((3,3)).unwrap();
    assert_eq!(matrix.dim(), (3,3));
    assert_eq!(matrix[(0,0)], 0.0);
    assert_eq!(matrix[(1,0)], 3.0);
    assert_eq!(matrix[(2,2)], 8.0);
}

#[test]
fn exercise9() {
 let arr = Array::from_vec(vec![1,2,0,0,4,0]);
 let indices: Vec<usize> = arr.indexed_iter().filter(|&(_,x)| { *x != 0 }).map(|(i,_)| i).collect();
 assert_eq!(indices, vec![0,1,4]);
}

#[test]
fn exercise9b() {
 let arr = Array::from_vec(vec![1,2,0,0,4,0]);
 let indices: Vec<usize> = arr.indexed_iter().filter_map(|(i,x)| {
     if *x != 0 { return Some(i); } else { return None }
 }).collect();
 assert_eq!(indices, vec![0,1,4]);
}

#[test]
fn exercise10() {
    let arr: Array2<f64> = Array::eye(3);
    assert_eq!(arr[(0,0)], 1.0);
    assert_eq!(arr[(1,1)], 1.0);
    assert_eq!(arr[(2,2)], 1.0);
    assert_eq!(arr[(1,0)], 0.0);
}

#[test]
fn exercise11() {
    // Watch out: if you get error:
    // "trait `rand::distributions::Normal: rand::distributions::IndependentSample<_>` not satisfied"
    // make sure your Cargo.toml includes crate rand=0.3
    let arr = Array::random((3, 3, 3 as usize),
                            rand::distributions::normal::Normal::new(0.0, 1.0));

    assert_eq!(arr.len(), 27);
    assert_ne!(arr[(0,0,0)], arr[(2,2,2)]);
}

#[test]
fn exercise12() {
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


