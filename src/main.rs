use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;
use std::io::{Write, stdout, stdin};
struct Player {
    x: u16,
    y: u16,
}
impl Player {
    fn render<W: Write>(&self, stdout: &mut RawTerminal<W>) -> std::io::Result<()> {
        write!(
            stdout,
            "{}{}@",
            termion::cursor::Goto(self.x, self.y),
            termion::clear::CurrentLine
        )?;
        stdout.flush()?;
        Ok(())
    }
}
fn main() -> std::io::Result<()> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;
    let mut player = Player { x: 1, y: 1 };

    write!(stdout, "{}", termion::clear::All)?;
    player.render(&mut stdout)?;

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
    }

    Ok(())
}
