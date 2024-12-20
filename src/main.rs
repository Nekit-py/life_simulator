mod field;

use field::*;

use crate::entities::Entity;
use core::time;
use crossterm::{
    event::KeyCode,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use entities::animals::{Boar, Lion};
use entities::food::Grass;
use entities::other::Wasteland;
use std::io::Write;
use std::thread;
fn test_case1() -> Vec<Vec<Entity>> {
    vec![
        vec![
            Entity::Boar(Boar::new(Point::new(0, 0))),
            Entity::Wasteland(Wasteland::new(Point::new(1, 0))),
            Entity::Wasteland(Wasteland::new(Point::new(2, 0))),
        ],
        vec![
            Entity::Grass(Grass::new(Point::new(0, 1))),
            Entity::Wasteland(Wasteland::new(Point::new(1, 1))),
            Entity::Wasteland(Wasteland::new(Point::new(2, 1))),
        ],
        vec![
            // Entity::Wasteland(Wasteland::new(Point::new(0, 2))),
            Entity::Lion(Lion::new(Point::new(0, 2))),
            Entity::Wasteland(Wasteland::new(Point::new(1, 2))),
            Entity::Grass(Grass::new(Point::new(2, 2))),
        ],
    ]
}

fn run() -> Result<(), std::io::Error> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    // let delay = time::Duration::from_millis(300);

    // let mut field = Field::from_test_case(test_case1());
    let mut field = Field::new(10, 10);
    let mut entities = field.get_entities();

    // Обеспечиваем отключение "сырого режима" при выходе
    let _guard = std::panic::catch_unwind(|| {
        terminal::disable_raw_mode().ok();
    });

    stdout.execute(terminal::Clear(ClearType::All))?;
    while entities.total_animals() > 0 {
        stdout.execute(terminal::Clear(ClearType::All))?;
        println!("Животных на поле: {}", entities.total_animals());
        field.simulate(&mut entities)?;
        stdout.flush()?;

        // thread::sleep(delay);

        // Выход из цикла
        if crossterm::event::poll(time::Duration::from_millis(50))? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                if key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    // Отключаем "сырой режим" перед выходом
    terminal::disable_raw_mode()?;
    Ok(())
}

fn run_test_simulation() -> Result<(), std::io::Error> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    // let delay = time::Duration::from_millis(300);

    let mut field = Field::from_test_case(test_case1());
    let mut entities = field.get_entities();

    // Обеспечиваем отключение "сырого режима" при выходе
    let _guard = std::panic::catch_unwind(|| {
        terminal::disable_raw_mode().ok();
    });

    stdout.execute(terminal::Clear(ClearType::All))?;
    while entities.total_animals() > 0 {
        stdout.execute(terminal::Clear(ClearType::All))?;
        field.simulate(&mut entities)?;
        stdout.flush()?;

        // thread::sleep(delay);

        // Выход из цикла
        if crossterm::event::poll(time::Duration::from_millis(50))? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                if key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    // Отключаем "сырой режим" перед выходом
    terminal::disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    run()?;
    // run_test_simulation()?;
    Ok(())
}
