use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// The SPI device to use
    #[arg(short, long)]
    pub dev: String,
    /// Whether to read once, or read continuesly
    #[clap(short, long, default_value_t = false)]
    pub once: bool,
    /// The interval at which the SPI device should be read. Only has an effect if `--once` is true
    #[clap(short, long, default_value_t = 500)]
    pub read_interval: u64,
    /// The size of the buffer to read, in bytes.
    #[clap(short, long, default_value_t = 32)]
    pub buffer_size: usize
}