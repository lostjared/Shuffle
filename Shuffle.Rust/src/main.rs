use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::env;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut v : Vec<String> = Vec::new();
    if args.len() <= 1 {
        for i in std::io::stdin().lock().lines() {
            v.push(i.unwrap());
        }
    } else {
        for i in args.iter().skip(1) {
            let data = std::fs::read_to_string(&i).unwrap();
            for z in data.lines() {
                v.push(z.to_string());
            }
        }
    }
    let mut rng = rand::thread_rng();
    let mut irs = Irs::default();
    irs.shuffle(&mut v, &mut rng).expect("on shuffle");
    for i in &v {
        println!("{}", i);
    }
    Ok(())
}