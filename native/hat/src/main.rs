fn main() -> Result<(), anyhow::Error> {
    // println!("rH = {}.{}%", humidity_x2 >> 1, 5 * (humidity_x2 & 0b1));
    let res = hat::execute()?;
    println!("{:?}", res);
    Ok(())
}
