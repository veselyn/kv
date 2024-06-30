use std::{
    fmt::Debug,
    io::{IsTerminal, Write},
};

pub trait Stdout: Write + IsTerminal + Debug {}
impl Stdout for std::io::Stdout {}

pub trait Stderr: Write + IsTerminal + Debug {}
impl Stderr for std::io::Stderr {}

#[derive(Debug)]
pub struct Env {
    pub stdout: Box<dyn Stdout>,
    pub stderr: Box<dyn Stderr>,
}

impl Env {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    stdout: Option<Box<dyn Stdout>>,
    stderr: Option<Box<dyn Stderr>>,
}

impl Builder {
    pub fn build(self) -> Env {
        let stdout = self.stdout.unwrap_or_else(|| Box::new(std::io::stdout()));
        let stderr = self.stderr.unwrap_or_else(|| Box::new(std::io::stderr()));

        Env { stdout, stderr }
    }

    pub fn stdout<S: Stdout + 'static>(mut self, stdout: S) -> Self {
        self.stdout = Some(Box::new(stdout));
        self
    }

    pub fn stderr<S: Stderr + 'static>(mut self, stderr: S) -> Self {
        self.stderr = Some(Box::new(stderr));
        self
    }
}
