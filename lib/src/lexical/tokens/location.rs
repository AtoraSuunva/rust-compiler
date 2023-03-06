use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location { line, column }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: display column, it's pretty funky when combined with tabs since editors decide how wide tabs are, so check how to best report it
        write!(f, "line {}", self.line)
    }
}
