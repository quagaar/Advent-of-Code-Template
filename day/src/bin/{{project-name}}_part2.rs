use {{crate_name}}::{part2, INPUT};

fn main() -> Result<(), part2::Error> {
    println!("{:?}", part2::solve(INPUT)?);
    Ok(())
}
