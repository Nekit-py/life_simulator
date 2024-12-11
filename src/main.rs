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
    let mut field = Field::new(2, 2);
    // let mut field = Field::new(1, 3);
    field.fill(&mut rng);
    println!("{}", field);
    field.start_new_life();
    println!("{}", field);
    // println!("{:#?}", field);

    // println!("{:#?}", field);

    Ok(())
}

////Работотающий пример
// use std::fmt;

// #[derive(Debug, Clone, Copy)]
// struct Foo{
//     val: i32
// }

// impl fmt::Display for Foo {
//     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
//       write!(f, "{}", self.val)?;
//       Ok(())
//     }
// }

// fn main() {
//     let foo = Foo{val:1};
//     let foo2 = Foo{val:2};
//     let mut baz = vec![foo];
//     println!("{:?}", baz);
//     for v in baz.iter_mut() {
//         *v = foo2;
//     }
//     println!("{:?}", baz);
//     println!("{}", baz[0].val);
// }
