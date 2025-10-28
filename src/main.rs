// use csv_to_json::{functions::read_csv, models::structs::HousePrice};

use std::{iter, slice::Iter};

fn main() {
    let _v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    // let mut v3 = Vec::new();
    // v3.push(5);
    // v3.push(6);
    // println!("{:?}", v3);

    let i = _v2[1];
    println!("{}", i);
    let i1 = _v2.get(3);
    match i1 {
        Some(i) => println!("{}", i),
        None => println!("No value"),
    }

    let v: Vec<i32> = vec![1,2];
    let mut iter: Iter<'_, i32> = v.iter();
    let n1 = iter.next().unwrap();
    let n2 = iter.next().unwrap();
    let n3 = iter.next();
    println!("{}, {}, {:?}", n1, n2, n3);
}
