#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use anyhow::anyhow;
use host::Guest;
use std::env;

fn main() -> Result<(), anyhow::Error> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let args: Vec<String> = env::args().collect();
    let guest = match args[1].as_str() {
        "sensor" => Ok(Guest::Sensor),
        "display" => Ok(Guest::LCDDisplay),
        "segment" => Ok(Guest::SegmentLEDDisplay),
        _ => Err(anyhow!("Unknown guest!")),
    }?;

    let option = if args.len() > 2 {
        Some(args[2].as_str())
    } else {
        None
    };

    let res = host::execute(guest, option)?;
    println!("{:?}", res);

    Ok(())
}
