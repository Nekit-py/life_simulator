mod field;

use field::*;

fn main() -> Result<(), std::io::Error> {
    let mut field = Field::new(30, 40);
    println!("{}", field);

    let mut entities = field.to_entities();
    field.from_entities(&mut entities);
    println!("{}", field);

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
