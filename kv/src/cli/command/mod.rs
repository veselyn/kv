use crate::app::App;
use std::fmt::Debug;
use std::io::{self, Write};
use std::result;
use thiserror::Error;

pub type Result = result::Result<Output, Error>;

#[derive(Default, Debug)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
}

impl Output {
    pub fn dump(&self) {
        self.dump_to(&mut std::io::stdout(), &mut std::io::stderr())
            .expect("dumping output");
    }

    fn dump_to<O, E>(&self, stdout: &mut O, stderr: &mut E) -> io::Result<()>
    where
        O: Write,
        E: Write,
    {
        if !self.stdout.is_empty() {
            write!(stdout, "{}", self.stdout)?;
            writeln!(stdout)?;
        }
        if !self.stderr.is_empty() {
            write!(stderr, "{}", self.stderr)?;
            writeln!(stderr)?;
        }
        Ok(())
    }

    pub fn stdout(mut self, stdout: String) -> Self {
        self.stdout = stdout;
        self
    }

    #[allow(dead_code)]
    pub fn stderr(mut self, stderr: String) -> Self {
        self.stderr = stderr;
        self
    }
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct Error {
    pub message: String,
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
        self.dump_to(&mut std::io::stderr()).expect("dumping error");
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

    #[allow(dead_code)]
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
