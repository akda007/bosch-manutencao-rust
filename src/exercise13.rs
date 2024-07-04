#![allow(unused)]

use std::{borrow::Borrow, fmt, io::{self, Write}};

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

fn initialize_vec(w:i32, h:i32, mtrx: &mut Vec<Vec<i32>>) {
    for i in 0..h {
        mtrx.push(Vec::new());

        for j in 0..w {
            mtrx[i as usize].push(0);
        }
    }
}

fn read_vec(w:i32, h:i32, mtrx: &mut Vec<Vec<i32>>) {
    for i in 0..h {
        for j in 0..w {
            print!("Matrix {}, {}: ", i, j);
            io::stdout().flush();
            
            read!(value as i32);

            mtrx[i as usize][j as usize] = value;
        }
    }
}

fn print_matrix(w:i32, h:i32, mtrx: Vec<Vec<i32>>) {
    for i in 0..h {
        for j in 0..w {
            print!("{:?} ", mtrx[i as usize][j as usize]);
        }
        println!();
    }
}

pub fn matriz() {
    let (w, h): (i32, i32) = (5, 3);
    
    let mut mtrx_a: Vec<Vec<i32>> = Vec::new();
    let mut mtrx_b: Vec<Vec<i32>> = Vec::new();
    let mut mtrx_r: Vec<Vec<i32>> = Vec::new();

    initialize_vec(w, h, &mut mtrx_a);
    initialize_vec(w, h, &mut mtrx_b);
    initialize_vec(w, h, &mut mtrx_r);

    read_vec(w, h, &mut mtrx_a);
    read_vec(w, h, &mut mtrx_b);
    
    for i in 0..h {
        for j in 0..w {
            mtrx_r[i as usize][j as usize] = mtrx_a[i as usize][j as usize] + mtrx_b[i as usize][j as usize];
        }
    }

    print_matrix(w, h, mtrx_r);
}