use std::{error::Error, fmt::Display, fs::File};

pub(crate) type Solution = (Box<dyn Display>, Box<dyn Display>);
pub(crate) type SolverFn = fn(File) -> Result<Solution, Box<dyn Error>>;
