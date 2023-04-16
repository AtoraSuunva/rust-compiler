use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
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
        write!(f, "[line {}]", self.line)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.column.cmp(&other.column),
            x => x,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
