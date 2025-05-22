use std::time::Instant;
use std::time::Duration;
use histogram::Histogram;
use std::fs::OpenOptions;
use std::path::PathBuf;
use memmap2::MmapMut;
use clap::Parser;

#[derive(Parser)]
#[command(name = "histogram")]
#[command(about = "Generates histogram data to an memory mapped output file")]
struct Args {
    /// Output file path for histogram data
    #[arg(help = "Path to the output file")]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file = OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(true)
                           .truncate(true)
                           .open(&args.output).expect("failed to open");
    file.set_len(1920 * 8).expect("error setting length");

    let mut histogram = Histogram::new(5, 64).unwrap();
    let mut mmap = unsafe { MmapMut::map_mut(&file).expect("failed to mmap") };

    let interval = Duration::from_millis(1000 / 60);

    loop {
        let next = Instant::now() + interval;
        std::thread::sleep(interval);
        let now = Instant::now();

        let delay = now.checked_duration_since(next).map(|v| v.as_nanos() as u64).unwrap_or(0);

        let _ = histogram.increment(delay);

        let (_prefix, buckets, _suffix) = unsafe { mmap.align_to_mut::<u64>() };
        buckets[0..1920].copy_from_slice(&histogram.as_slice()[0..1920]);
    }
}
