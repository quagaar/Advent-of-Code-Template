use {{crate_name}}::{part1, INPUT};

fn main() -> Result<(), part1::Error> {
    println!("{:?}", part1::solve(INPUT)?);
    Ok(())
}
