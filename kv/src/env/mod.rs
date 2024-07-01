pub mod stdio;

use async_std::sync::{Arc, Mutex};
use std::fmt::Debug;
pub use stdio::{Stderr, Stdout};

#[derive(Debug, Clone)]
pub struct Env {
    pub stdout: Arc<Mutex<dyn Stdout>>,
    pub stderr: Arc<Mutex<dyn Stderr>>,
}

impl Env {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    stdout: Option<Arc<Mutex<dyn Stdout>>>,
    stderr: Option<Arc<Mutex<dyn Stderr>>>,
}

impl Builder {
    pub fn build(self) -> Env {
        let stdout = self
            .stdout
            .unwrap_or_else(|| Arc::new(Mutex::new(std::io::stdout())));
        let stderr = self
            .stderr
            .unwrap_or_else(|| Arc::new(Mutex::new(std::io::stderr())));

        Env { stdout, stderr }
    }

    pub fn stdout(mut self, stdout: Arc<Mutex<dyn Stdout>>) -> Self {
        self.stdout = Some(stdout);
        self
    }

    pub fn stderr(mut self, stderr: Arc<Mutex<dyn Stderr>>) -> Self {
        self.stderr = Some(stderr);
        self
    }
}
