use std::{io, thread};
use std::io::Error;
use std::io::Write;
use std::time::Duration;

use termion::AsyncReader;

use crate::wator::wator::Wator;
use std::marker::PhantomData;
use crate::Main;
use crate::common::persistence::HighScores;
use termion::input::Keys;
use termion::event::Key;

pub struct WatorMain<W: Write> {
    _marker: PhantomData<W>,
}

impl <W: Write> WatorMain<W> {
    pub fn new() -> WatorMain<W> {
        WatorMain { _marker: PhantomData }
    }
}

impl <W: Write> Main<W> for WatorMain<W> {

    fn name(&self) -> &'static str {
        "Wator"
    }

    fn run(&self, mut stdout: &mut W, stdin: &mut Keys<AsyncReader>) -> io::Result<Option<u32>> {
        write!(stdout,
               "{}{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               termion::cursor::Goto(1, 2))?;

        stdout.flush()?;

        let mut wator = Wator::new(80, 40);

        let mut time: u32 = 0;

        let mut result : io::Result<Option<u32>> = Result::Ok(None);

        loop {
            time += 1;

            print(&mut stdout, &mut wator)?;

            let b = stdin.next();
            if let Some(Ok(Key::Esc)) = b {
                break
            }
            thread::sleep(Duration::from_millis(50));
            wator = wator.next();

            let (fishes, sharks) = wator.count();

            if fishes == 0 || sharks == 0 {
                result = Result::Ok(Some(time));
                break;
            }
        }

        while stdin.next().is_some() { }

        result
    }

    fn high_scores(&self) -> Result<HighScores, Error> {
        HighScores::read(".wator")
    }

}

fn print<W: Write>(mut stdout: &mut W, wator: &Wator) -> io::Result<()> {
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))?;
    wator.print(&mut stdout, true)
}