mod field;

use field::*;

use animals::Boar;
use crossterm::{
    event::KeyCode,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use food::Grass;
use other::Wasteland;
use std::io::Write;
use std::{thread, time};

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
            Entity::Wasteland(Wasteland::new(Point::new(0, 2))),
            Entity::Wasteland(Wasteland::new(Point::new(1, 2))),
            Entity::Grass(Grass::new(Point::new(2, 2))),
        ],
    ]
}

fn run() -> Result<(), std::io::Error> {
    // Инициализация терминала
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    let delay = time::Duration::from_millis(1300);

    let mut field = Field::new(3, 3);

    loop {
        // Очищаем экран
        // stdout.execute(terminal::Clear(ClearType::All))?;

        let mut entities = field.get_entities();
        field.simulate(&mut entities);
        println!("{}", field);
        // Обновляем экран
        stdout.flush()?;
        thread::sleep(delay);

        // Проверка нажатия клавиши для выхода (например, ESC)
        if crossterm::event::poll(delay)? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                if key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn run_test_simulation() -> Result<(), std::io::Error> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    let delay = time::Duration::from_millis(1300);

    stdout.execute(terminal::Clear(ClearType::All))?;
    let mut field = Field::from_test_case(test_case1());
    // println!("{}", field);
    loop {
        // stdout.execute(terminal::Clear(ClearType::All))?;
        // println!("{:#?}", field);
        let mut entities = field.get_entities();
        field.simulate(&mut entities);
        // println!("{:#?}", field);
        print!("{}", field);
        stdout.flush()?;

        thread::sleep(delay);

        if crossterm::event::poll(delay)? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                if key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    // run()?;
    run_test_simulation()?;
    Ok(())
}
