use crate::app::App;
use std::fmt::{Debug, Display};
use std::io::Write;
use std::process::{ExitCode, Termination};

pub struct Result {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub status: Option<ExitCode>,
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.stdout {
            Some(stdout) if !stdout.is_empty() => {
                write!(f, "{stdout}")?;
                writeln!(f)?;
            }
            _ => (),
        };
        Ok(())
    }
}

impl Debug for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.stderr {
            Some(stderr) if !stderr.is_empty() => {
                write!(f, "{stderr}")?;
                writeln!(f)?;
            }
            _ => (),
        };
        Ok(())
    }
}

impl Result {
    pub fn dump(&self) {
        self.dump_to(std::io::stdout(), std::io::stderr());
    }

    pub fn dump_to<O, E>(&self, mut stdout: O, mut stderr: E)
    where
        O: Write,
        E: Write,
    {
        write!(stdout, "{self}").expect("dumping result to stdout");
        write!(stderr, "{self:?}").expect("dumping result to stderr");
    }
}

impl Termination for Result {
    fn report(self) -> ExitCode {
        self.status.unwrap_or(ExitCode::SUCCESS)
    }
}

pub trait Execute {
    async fn execute(self, app: App) -> Result;
}
