use std::fs::File;
use flussab_cnf::{cnf::Parser, Dimacs};
use itertools::Itertools;

/// 0-based variable index, possibly negated — `false` in the `bool` field means negated
#[derive(Clone, Copy, PartialEq, Eq)]
struct Literal(usize, bool);

fn print_clause(mut clause: Vec<Literal>, num_variables: usize) {
    let mut pattern = vec![None; num_variables];
    for Literal(var, positive) in clause {
        // We negate it here, as we need to match the assignments that *don't* satisfy
        // the clause. While not doing this would generate an equivalent instance, this
        // way the results rustc outputs directly correspond with our input.
        pattern[var] = Some(!positive);
    }

    let pattern = pattern.into_iter()
        .map(|pat| match pat {
            None => "_",
            Some(true) => "true",
            Some(false) => "false",
        })
        .join(", ");

    println!("({}) => {{}}", pattern);
}

impl Literal {
    fn from_dimacs(x: i32) -> Literal {
        if x > 0 {
            Literal(x as usize - 1, true)
        } else {
            Literal(-x as usize - 1, false)
        }
    }
}

fn main() {
    let file = File::open("example.cnf").unwrap();
    let mut parser = Parser::from_read(file, true).unwrap();
    let var_count = parser.header().unwrap().var_count;
    println!("fn main() {{ match todo!() {{");
    while let Some(clause) = parser.next_clause().unwrap() {
        let clause = clause.iter().copied().map(Literal::from_dimacs).collect();
        print_clause(clause, var_count as usize);
    }
    println!("}} }}");
}
