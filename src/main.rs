use std::cmp::PartialEq;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;

use std::fmt;
use std::fmt::Formatter;

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return BufReader::new(file).lines();
}

fn return_potential(filename: String) -> Vec<String> {
    // Returns a vec of potential words given a filename
    let lines = read_lines(filename);

    let mut out: Vec<String> = Vec::new();

    for line in lines {
        let l = line.unwrap(); // Because line: Result<String>
        if l.len() == 5 && l.to_lowercase().starts_with("cr") {
            out.push(l.to_string())
        }
    }

    return out;
}

// Generic function to remove duplicates for a Vec of elements
// that implement PartialEq Traits
fn remove_duplicates<T: PartialEq>(src: Vec<T>) -> Vec<T> {
    let mut out: Vec<T> = Vec::new();

    for e in src.into_iter() {
        if !out.iter().any(|i| *i == e) {
            out.push(e);
        }
    }

    return out;
}

// To define tree
struct Tree {
    level: usize,
    value: i32,
    children: Vec<Tree>,
    end: bool,
}

// Implement some methods to the tree
impl Tree {
    // new and new_termination are here to make code more readable. They are not particularly
    // necessary here

    fn new(level: usize, value: usize) -> Self {
        return Tree { level, value: value as i32, children: Vec::new(), end: false };
    }

    fn new_termination(level: usize) -> Self {
        return Tree { level, value: 0, children: Vec::new(), end: true };
    }

    // Same, probably not the mose useful but makes it readable
    fn append_children(&mut self, new_children: &mut Vec<Tree>) {
        self.children.append(new_children)
    }

    // Recursively flatten the tree into an array of solutions
    fn flatten(&self) -> Vec<Vec<i32>> {
        if self.end {
            return vec![vec![]];
        }

        let mut out: Vec<Vec<i32>> = Vec::new();

        for child in self.children.iter() {
            let mut temp: Vec<Vec<i32>> = child.flatten();

            for r in temp.iter_mut() {
                r.insert(0, self.value);
            }

            out.append(&mut temp);
        }

        return out;
    }
}

// Recursively find solutions given a word and a substitute dictionary
fn look_for_dict(word: &String, dict: &Vec<String>, i0: usize) -> Option<Vec<Tree>> {
    let mut out: Vec<Tree> = Vec::new();

    // Check for the end
    if i0 == word.len() {
        return Some(vec![Tree::new_termination(i0)]);
    } else if i0 >= word.len() {
        return None;
    }

    // For every word in the subs dict
    for (i, art) in dict.iter().enumerate() {
        // If it works
        if word[i0..].starts_with(art) {
            let mut tree = Tree::new(i0, i);

            // Search deeper
            // Note: This is a cool implementation permitted by rust, using the Option<> enum
            // to specify if a branch has a real end or not
            // (see the end checking & the return to see how we delete a branch)
            let result: bool = match look_for_dict(word, dict, i0 + art.len()) {
                Some(mut trees) => {
                    tree.append_children(&mut trees);

                    true
                }
                None => {
                    false
                }
            };

            // If there is at least one children -> create a new branch
            if result {
                out.push(tree)
            }
        }
    }

    // If we have at least one sub-branch, return as a new branch
    return if out.len() == 0 {
        None
    } else {
        Some(out)
    }
}

// Solution formatting / filtering
struct Solution {
    word: String,
    code: Vec<i32>,
    code_length: usize
}

impl Solution {
    // Allows us to compute the code_length in a proper method
    fn new(word: String, code: Vec<i32>) -> Self {
        let code_length = code.iter().fold(0, |acc, e| acc + (format!("{}", e).len() as i32));

        // println!("Computed code length of {} for {:.?}", code_length, code);

        return Solution { word, code, code_length: code_length as usize }
    }
}

// So that we can println!("{}", solution)
impl fmt::Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Solution: {} | {}",
            self.word.to_lowercase(),
            self.code.iter().fold("".to_string(),
                |mut acc, e| {
                    acc.push_str((e + 1).to_string().as_str());
                    return acc
                }))
    }
}

// Filter the solutions according to our conditions
fn filter_solution(input: Vec<Solution>) -> Vec<Solution> {
    let mut out: Vec<Solution> = Vec::new();

    for sol in input {
        // Here we have -1 because it's a [periodic: Vec<String>] index
        // The second condition can be removed because we already filter in return_potential
        if sol.code_length == 7 && sol.code[0] == 24-1 {
            out.push(sol);
        }
    }

    return out;
}

/// TODO: none


fn main() {
    let periodic: Vec<String> = vec!["H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na",
                                     "Mg", "Al", "Si", "P", "S", "Cl", "Ar", "K", "Ca", "Sc", "Ti",
                                     "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge",
                                     "As", "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo",
                                     "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In", "Sn", "Sb", "Te",
                                     "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm",
                                     "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf",
                                     "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb",
                                     "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U",
                                     "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md", "No",
                                     "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn",
                                     "Uut", "Fl", "Uup", "Lv", "Uus", "Uuo"]
        .iter().map(|e| e.to_lowercase()).collect();

    println!("Loading database...");

    let mut database = return_potential("./ods6.txt".to_string());

    database.append(&mut return_potential("./words.txt".to_string()));

    println!("There is {} possibilities", database.len());

    let mut duplicate_free = remove_duplicates(database);

    println!("Duplicates removed, there is now {} possibilities", duplicate_free.len());


    let mut set: Vec<Solution> = Vec::new();

    // For every word we're gonna check
    for word in duplicate_free {
        let mut p: Vec<Vec<i32>> = Vec::new();

        // Look if there is at least one solution
        match look_for_dict(&word, &periodic, 0) {
            Some(tree) => {
                // If there is
                for possibility in tree {
                    let mut flat = possibility.flatten();

                    // We append the solution to a solution list
                    p.append(&mut flat);
                }
            }
            None => {}
        }

        // If the word has at least one solution
        // (can be compacted but no for the sake of visibility)
        if p.len() != 0 {
            println!(" **** {} : {}", word, p.len());

            // for code in partial solution set
            for c in p {
                // Append the [solution: Solution] to the solution set
                set.push(Solution::new(word.to_string(), c.clone()));

                // for code_part in code
                for c_P in c {
                    print!("{} ", c_P+1);
                }

                println!();
            }
        }
    }

    // We filter our solutions
    let final_sols: Vec<Solution> = filter_solution(set);

    // Print the end
    println!("\n\n\n ******************** \n There is {} solutions\n", final_sols.len());

    for sol in final_sols {
        println!("{}", sol);
    }
}
