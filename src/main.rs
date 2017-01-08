#[macro_use]
extern crate ndarray;
use ndarray::*;

fn main() {
    println!("Please run `cargo test` to run examples");
}

/*
 * 100 Excersises for Numpy:
 * http://www.labri.fr/perso/nrougier/teaching/numpy.100/
 */

#[test]
fn excersise3() {
    let arr = Array1::<f64>::zeros((10));
    assert_eq!(arr.len(), 10);
    assert_eq!(arr.dim(), (10));
}

#[test]
fn excersise5() {
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

#[test]
fn exercise7() {
    // TODO: add more solutions
    let arr: Array1<f64> = Array::range(0.0, 50.0, 1.0);
    let reverse = arr.slice(s!(..;-1));
    assert_eq!(reverse.len(), 50);
    assert_eq!(reverse[0], 49.0);
    assert_eq!(reverse[49], 0.0);
}

#[test]
fn exercise7() {
    // TODO: add more solutions
    let arr: Array1<f64> = Array::range(0.0, 50.0, 1.0);
    let reverse = arr.slice(s!(..;-1));
    assert_eq!(reverse.len(), 50);
    assert_eq!(reverse[0], 49.0);
    assert_eq!(reverse[49], 0.0);
}
