mod memory;

use is_terminal::IsTerminal;
pub use memory::*;
use std::fmt::Debug;
use std::io::Write;

pub trait Stdout: Write + IsTerminal + Debug + Sync + Send {}
impl Stdout for std::io::Stdout {}

pub trait Stderr: Write + IsTerminal + Debug + Sync + Send {}
impl Stderr for std::io::Stderr {}
