use crate::app::App;
use crate::env::Env;
use std::fmt::Debug;
use std::io;
use std::result;
use thiserror::Error;

pub type Result = result::Result<Output, Error>;

#[derive(Debug)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
    env: Env,
}

impl Output {
    pub async fn dump(&self) {
        self.try_dump().await.expect("dumping output");
    }

    async fn try_dump(&self) -> io::Result<()> {
        if !self.stdout.is_empty() {
            write!(self.env.stdout.lock().await, "{}", self.stdout)?;
            writeln!(self.env.stdout.lock().await)?;
        }

        if !self.stderr.is_empty() {
            write!(self.env.stderr.lock().await, "{}", self.stderr)?;
            writeln!(self.env.stderr.lock().await)?;
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

impl From<Env> for Output {
    fn from(value: Env) -> Self {
        Self {
            stdout: "".to_owned(),
            stderr: "".to_owned(),
            env: value,
        }
    }
}

impl From<&Env> for Output {
    fn from(value: &Env) -> Self {
        Self::from(value.clone())
    }
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct Error {
    pub message: String,
    pub status: u8,
    env: Env,
}

impl From<Env> for Error {
    fn from(value: Env) -> Self {
        Self {
            message: "Something went wrong".to_owned(),
            status: 1,
            env: value,
        }
    }
}

impl From<&Env> for Error {
    fn from(value: &Env) -> Self {
        Self::from(value.clone())
    }
}

impl Error {
    pub async fn dump(&self) {
        self.try_dump().await.expect("dumping error");
    }

    async fn try_dump(&self) -> io::Result<()> {
        if !self.message.is_empty() {
            write!(self.env.stderr.lock().await, "Error: {}", self.message)?;
            writeln!(self.env.stderr.lock().await)?;
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
