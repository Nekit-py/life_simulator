mod field;

use field::*;

fn main() -> Result<(), std::io::Error> {
    let mut field = Field::new(30, 40);
    println!("{}", field);

    let mut entities = field.get_entities();
    field.start_simulation(&mut entities);
    println!("{}", field);

    Ok(())
}
