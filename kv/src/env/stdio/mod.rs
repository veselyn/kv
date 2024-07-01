use is_terminal::IsTerminal;
use std::fmt::Debug;
use std::io::Write;

pub trait Stdout: Write + IsTerminal + Debug + Sync + Send {}
impl Stdout for std::io::Stdout {}

pub trait Stderr: Write + IsTerminal + Debug + Sync + Send {}
impl Stderr for std::io::Stderr {}

#[cfg(test)]
mod memory;
#[cfg(test)]
pub use memory::*;
