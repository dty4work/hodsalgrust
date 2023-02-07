fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let x: i32 = "3".parse()?;

    Ok(())
}

#[test]
fn parsing_works() -> Result<(), Box<dyn std::error::Error>> {
    let x: i32 = "3".parse()?;

    Ok(())
}
