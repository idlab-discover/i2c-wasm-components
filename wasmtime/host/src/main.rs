fn main() -> Result<(), anyhow::Error> {
    let res = host::execute()?;

    println!("{:?}", res);

    Ok(())
}
