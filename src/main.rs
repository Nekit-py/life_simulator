mod field;
//
use field::*;
use rand::thread_rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
// use std::io::{self, Write};
// use std::time::{Duration, Instant};

fn main() -> Result<(), std::io::Error> {
    let mut rng = thread_rng();
    // // let mut field = Field::new(50, 70);
    let mut field = Field::new(2, 3);
    // let mut field = Field::new(1, 3);
    field.fill(&mut rng);
    println!("{}", field);
    field.start_new_life();
    println!("{}", field);
    // println!("{:#?}", field);

    Ok(())
}
