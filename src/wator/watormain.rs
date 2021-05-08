use std::{io, thread};
use std::io::{Error, Read};
use std::io::Write;
use std::marker::PhantomData;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use crate::common::persistence::HighScores;
use crate::Main;
use crate::wator::wator::Wator;

pub struct WatorMain<W: Write, R: Read> {
    _w_marker: PhantomData<W>,
    _r_marker: PhantomData<R>,
}

impl<W: Write, R: Read> WatorMain<W, R> {
    pub fn new() -> WatorMain<W, R> {
        WatorMain { _w_marker: PhantomData, _r_marker: PhantomData }
    }
}

impl<W: Write, R: Read> Main<W, R> for WatorMain<W, R> {
    fn name(&self) -> &'static str {
        "Wa-tor"
    }

    fn run(&self, mut stdout: &mut W, stdin: &mut R) -> io::Result<Option<u32>> {
        write!(stdout,
               "{}{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               termion::cursor::Goto(1, 2))?;

        stdout.flush()?;

        let mut wator = Wator::new(80, 40);

        let mut time: u32 = 0;

        let mut result: io::Result<Option<u32>> = Result::Ok(None);

        loop {
            time += 1;

            print(&mut stdout, &mut wator)?;

            let b = stdin.keys().next();
            if let Some(Ok(Key::Esc)) = b {
                break;
            }
            thread::sleep(Duration::from_millis(50));
            wator = wator.next();

            let (fishes, sharks) = wator.count();

            if fishes == 0 || sharks == 0 {
                result = Result::Ok(Some(time));
                break;
            }
        }

        while stdin.keys().next().is_some() {}

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