#![allow(unused)]

use std::io;
use std::io::Read;
use std::fs::File;

// matchでエラーを呼び出し元のコードに返す関数(エラーの委譲)
fn read_username_from_file_v1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?演算子でエラーを呼び出し元に返す関数
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    println!("{}", read_username_from_file_v3().unwrap());
}
