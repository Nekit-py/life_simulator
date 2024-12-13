mod field;

use field::*;

use crossterm::{
    event::KeyCode,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::Write;
use std::{thread, time};

fn main() -> Result<(), std::io::Error> {
    // Инициализация терминала
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    let delay = time::Duration::from_millis(10);

    let mut field = Field::new(3, 4);
    stdout.execute(terminal::Clear(ClearType::All))?;
    println!("{}", field);

    loop {
        // Очищаем экран
        stdout.execute(terminal::Clear(ClearType::All))?;

        let mut entities = field.get_entities();
        field.simulate(&mut entities);
        println!("{}", field);
        // Обновляем экран
        stdout.flush()?;
        thread::sleep(delay);

        // Проверка нажатия клавиши для выхода (например, ESC)
        if crossterm::event::poll(delay).unwrap() {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                if key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    Ok(())
}
