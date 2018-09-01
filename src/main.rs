#![feature(test)]

extern crate actix_web;
extern crate time;

use actix_web::{http, server, App, Path, Responder};
use std::collections::HashMap;
#[macro_use] extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref INSTANT: Mutex<time::Timespec> = Mutex::new(time::get_time());
    static ref COUNT: Mutex<u8> = Mutex::new(0);
}


fn instant(shard: u8) -> u64 {
    let now = time::get_time();
    let mut prev = INSTANT.lock().unwrap();
    let mut count = COUNT.lock().unwrap();
    if *prev == now {
      *count += 1;
    } else {
      *count = 0;
    }
    *prev = now;
  
    let val = (now.sec as u64) << 36
        | (now.nsec as u64) << 4
        | (shard as u64) << 4
        | (*count as u64);
    //println!("s {:0>64b} {0}", now.sec);
    //println!("s {:0>64b} {0}", now.sec as u64);
    //println!("s {:0>64b}", now.sec << 32);
    //println!("n {:0>64b}", now.nsec);
    //println!("c {:0>64b}", *count);
    //println!("+ {:0>64b}", shard);
    //println!("n {:0>64b}", now.nsec);
    //println!("c {:0>64b}", *count);
    //println!("+ {:0>64b}", shard);
    //println!("+ {:0>64b}", val);
    val
}

fn foo(_: Path<()>) -> impl Responder {
    format!("{:0>64b}", instant(1))
}

fn index(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() {
    server::new(
        || App::new()
            .route("/", http::Method::GET, foo)
            .route("/{id}/{name}/index.html", http::Method::GET, index))
        .bind("127.0.0.1:8080").unwrap()
        .run();
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn duplicates() {
        let mut counts = HashMap::new();
        let instants: Vec<_> = (0..1000000).map(|_| instant(15)).collect();
        let mut last = 0;
        for i in instants {
            let mut c = counts.entry(i).or_insert(0);
            *c += 1;
            if last > i {
                panic!("{:?} {:?}", last, i);
            }
            last = i;
        }
        for (i, val) in &counts {
            if *val > 1 { panic!("{:?} {:?}", i, val) }
        }
    }
    #[bench]
    fn bench_foo(b: &mut Bencher) {
        b.iter(|| instant(30));
    }
}
