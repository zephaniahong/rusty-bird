mod flappybird;
use crossterm::{
    cursor, execute,
    style::{Color, Print, SetBackgroundColor},
    terminal::{Clear, ClearType},
    Result,
};
pub use flappybird::{FlappyBird, Position};

use std::thread;
use std::{io::stdout, time::Duration};

fn main() -> Result<()> {
    // Initial setup
    let bird = FlappyBird::new(String::from("O"), Position::new(0, 0));
    let mut stdout = stdout();

    execute!(
        stdout,
        Clear(ClearType::All),
        SetBackgroundColor(Color::DarkGrey),
        cursor::MoveTo(0, 0),
        cursor::Hide
    )?;

    loop {
        execute!(stdout, cursor::MoveRight(1), Clear(ClearType::All),)?;

        bird.display(&mut stdout)?;

        thread::sleep(Duration::new(0, 500000000)) // slows down movement of bird
    }
}
