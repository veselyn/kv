use crate::app::App;
use std::io::{self, Read, Write};
use std::result;
use thiserror::Error;

pub type Result = result::Result<Output, Error>;

pub struct Output {
    stdout: Box<dyn Read>,
    stderr: Box<dyn Read>,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            stdout: Box::new(io::empty()),
            stderr: Box::new(io::empty()),
        }
    }
}

impl Output {
    pub fn dump(&mut self) {
        self.dump_to(&mut io::stdout(), &mut io::stderr())
            .expect("dumping output");
    }

    fn dump_to<O, E>(&mut self, stdout: &mut O, stderr: &mut E) -> io::Result<()>
    where
        O: Write,
        E: Write,
    {
        let bytes_copied = io::copy(&mut self.stdout, stdout)?;
        if bytes_copied > 0 {
            writeln!(stdout)?;
        }
        let bytes_copied = io::copy(&mut self.stderr, stderr)?;
        if bytes_copied > 0 {
            writeln!(stderr)?;
        }
        Ok(())
    }

    pub fn stdout<O: Read + 'static>(mut self, stdout: O) -> Self {
        self.stdout = Box::new(stdout);
        self
    }

    pub fn stderr<E: Read + 'static>(mut self, stderr: E) -> Self {
        self.stderr = Box::new(stderr);
        self
    }
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct Error {
    message: String,
    pub status: u8,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            message: "Something went wrong".to_owned(),
            status: 1,
        }
    }
}

impl Error {
    pub fn dump(&self) {
        self.dump_to(&mut io::stderr()).expect("dumping error");
    }

    fn dump_to<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        if !self.message.is_empty() {
            write!(writer, "Error: {}", self.message)?;
            writeln!(writer)?;
        }
        Ok(())
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn status(mut self, status: u8) -> Self {
        self.status = status;
        self
    }
}

pub trait Execute {
    async fn execute(self, app: &App) -> Result;
}

#[cfg(test)]
mod tests;
