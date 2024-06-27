use crate::app::App;
use std::fmt::{Debug, Display};
use std::io::{self, Write};
use std::process::ExitCode;
use std::result;
use thiserror::Error;

pub type Result = result::Result<Output, Error>;

#[derive(Error)]
#[error("{:?}", self)]
pub struct Error {
    pub message: String,
    pub status: ExitCode,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            message: "Something went wrong".to_string(),
            status: ExitCode::FAILURE,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.message.is_empty() {
            write!(f, "{}", self.message)?;
        }
        Ok(())
    }
}

impl Error {
    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn _status(mut self, status: ExitCode) -> Self {
        self.status = status;
        self
    }
}

#[derive(Default, Debug)]
pub struct Output {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("wow really")
    }
}

impl Output {
    pub fn dump(&self) {
        self.dump_to(&mut std::io::stdout(), &mut std::io::stderr());
    }

    pub fn dump_to<O, E>(&self, stdout: &mut O, stderr: &mut E)
    where
        O: Write,
        E: Write,
    {
        self.dump_stdout(stdout).expect("dumping result stdout");
        self.dump_stderr(stderr).expect("dumping result stderr");
    }

    fn dump_stdout<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        match &self.stdout {
            Some(stdout) if !stdout.is_empty() => {
                write!(writer, "{}", stdout)?;
                writeln!(writer)?;
            }
            _ => (),
        };
        Ok(())
    }

    fn dump_stderr<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        match &self.stderr {
            Some(stderr) if !stderr.is_empty() => {
                write!(writer, "{}", stderr)?;
                writeln!(writer)?;
            }
            _ => (),
        };
        Ok(())
    }
}

pub trait Execute {
    async fn execute(self, app: App) -> Result;
}