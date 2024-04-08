#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<(), anyhow::Error> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    // println!("rH = {}.{}%", humidity_x2 >> 1, 5 * (humidity_x2 & 0b1));
    let res = hat::execute()?;
    println!("{:?}", res);
    Ok(())
}
