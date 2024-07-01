use super::*;
use is_terminal::IsTerminal;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug)]
pub struct MemoryStdout {
    pub buf: Vec<u8>,
    terminal: bool,
}

impl MemoryStdout {
    pub fn new(terminal: bool) -> Self {
        Self {
            buf: Vec::new(),
            terminal,
        }
    }
}

impl Stdout for MemoryStdout {}

impl Write for MemoryStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf.flush()
    }
}

impl IsTerminal for MemoryStdout {
    fn is_terminal(&self) -> bool {
        self.terminal
    }
}

#[derive(Debug)]
pub struct MemoryStderr {
    pub buf: Vec<u8>,
    terminal: bool,
}

impl MemoryStderr {
    pub fn new(terminal: bool) -> Self {
        Self {
            buf: Vec::new(),
            terminal,
        }
    }
}

impl Stderr for MemoryStderr {}

impl Write for MemoryStderr {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf.flush()
    }
}

impl IsTerminal for MemoryStderr {
    fn is_terminal(&self) -> bool {
        self.terminal
    }
}
