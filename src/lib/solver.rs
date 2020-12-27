use super::replace_chars;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::{collections::HashMap, collections::HashSet, hash::Hash};

pub enum Path {
    Down,
    Left,
    Right,
}

impl FromStr for Path {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Path::Down),
            "<" => Ok(Path::Left),
            ">" => Ok(Path::Right),
            _ => {
                println!("'{}' is not a correct character", s);
                return Err(ParserError::IncorrectChar);
            }
        }
    }
}

pub struct StickPathSolver<T, Y> {
    outputs: Vec<Y>,
    c_index: HashMap<usize, HashSet<T>>,
}

#[derive(Debug)]
pub enum ParserError {
    IncorrectChar,
    Empty,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::IncorrectChar => write!(f, "incorrect character found"),
            ParserError::Empty => write!(f, "line was empty"),
        }
    }
}

#[derive(Debug)]
pub enum InputError {
    IncorrectInputSize,
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InputError::IncorrectInputSize => write!(f, "incorrect input size"),
        }
    }
}

impl<I, O> StickPathSolver<I, O>
where
    I: Eq + Hash + Clone,
{
    /// Initiates a new StickPathSolver with I and O as input type & output type
    pub fn new() -> StickPathSolver<I, O> {
        StickPathSolver {
            outputs: Vec::new(),
            c_index: HashMap::new(),
        }
    }
    /// Initiates the internal hashset based on the problem entries
    pub fn add_entries(&mut self, entries: Vec<I>) -> Result<(), InputError> {
        let nb_entries = entries.len();
        for index in 0..nb_entries {
            let mut hashset = HashSet::new();
            hashset.insert(entries[index].clone());
            self.c_index.insert(index, hashset);
        }
        Ok(())
    }
    /// Register the outputs internally to compute the paths
    pub fn add_outputs(&mut self, outputs: Vec<O>) -> Result<(), InputError> {
        if outputs.len() != self.c_index.len() {
            println!(
                "Expected {} outputs, got {} outputs",
                self.c_index.len(),
                outputs.len()
            );
            return Err(InputError::IncorrectInputSize);
        }
        self.outputs = outputs;
        Ok(())
    }
    /// Compute, for each entry, their position, based on the string path given as parameter
    pub fn add_path(&mut self, path: &str) -> Result<(), ParserError> {
        let new_path = replace_chars(path);
        let chars = new_path.split_ascii_whitespace().collect::<Vec<&str>>();
        if chars.is_empty() {
            return Err(ParserError::Empty);
        }
        let mut new_indexes = HashMap::new();
        for (index, c) in chars.into_iter().enumerate() {
            let new_index = match Path::from_str(c) {
                Ok(c_path) => match c_path {
                    Path::Down => index, // Nothing to do here,
                    Path::Left => index - 1,
                    Path::Right => index + 1,
                },
                Err(error) => return Err(error),
            };
            for item in self.c_index.get(&index).unwrap().into_iter() {
                new_indexes
                    .entry(new_index)
                    .or_insert(HashSet::new())
                    .insert(item.clone());
            }
        }
        self.c_index = new_indexes;
        Ok(())
    }
    /// Returns the input and output for each entry, as a vector
    pub fn get_paths(&self) -> Vec<(&I, &O)> {
        let mut paths = Vec::new();
        for (index, entries) in self.c_index.iter() {
            for entry in entries.iter() {
                paths.push((entry, &self.outputs[*index]));
            }
        }
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::StickPathSolver;
    #[test]
    fn test_first_example() {
        let mut solver: StickPathSolver<&str, i32> = StickPathSolver::new();
        if let Err(error) = solver.add_entries(vec!["A", "B", "C"]) {
            panic!("cannot add entries due to unexpected error {}", error);
        };
        for path in vec!["| | |", "|--| |", "| |--|", "| |--|", "| | |"].iter() {
            if let Err(error) = solver.add_path(path) {
                panic!("cannot add path due to unexpected error {}", error);
            }
        }
        if let Err(error) = solver.add_outputs(vec![1, 2, 3]) {
            panic!("cannot add outputs due to unexpected error {}", error);
        }
        let paths = solver.get_paths();
        let expected_outputs = vec![(&"A", &2i32), (&"B", &1i32), (&"C", &3i32)];
        for expected_output in expected_outputs.iter() {
            assert!(paths.contains(expected_output));
        }
    }

    #[test]
    fn test_second_example() {
        let mut solver: StickPathSolver<&str, i32> = StickPathSolver::new();
        if let Err(error) = solver.add_entries(vec!["P", "Q", "R", "S", "T", "U", "V", "W"]) {
            panic!("cannot add entries due to unexpected error {}", error);
        };
        for path in vec![
            "| | | | |--| | |",
            "| | |--| | | |--|",
            "| |--| |--| | | |",
            "|--| |--| | | |--|",
            "|--| | | | |--| |",
            "| |--| | |--| |--|",
            "| | | |--| |--| |",
            "|--| | | |--| | |",
            "| | |--| | | | |",
            "| | | |--| | |--|",
            "| | | | |--| | |",
            "|--| | | | | | |",
            "|--| |--| | | |--|",
            "| |--| | |--| | |",
            "| | |--| | | |--|",
            "|--| |--| | |--| |",
        ]
        .iter()
        {
            if let Err(error) = solver.add_path(path) {
                panic!("cannot add path due to unexpected error {}", error);
            }
        }
        if let Err(error) = solver.add_outputs(vec![1, 2, 3, 4, 5, 6, 7, 8]) {
            panic!("cannot add outputs due to unexpected error {}", error);
        }
        let paths = solver.get_paths();
        let expected_outputs = vec![
            (&"P", &3i32),
            (&"Q", &7),
            (&"R", &8),
            (&"S", &5),
            (&"T", &6),
            (&"U", &2),
            (&"V", &4),
            (&"W", &1),
        ];
        for expected_output in expected_outputs.iter() {
            assert!(paths.contains(expected_output));
        }
    }
}
