use std::io;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::args::Args;

mod args;

fn main() {
    let args = Args::parse();
    configure_tracing();

    if !Path::new(&args.dev).exists() {
        error!("Provided device does not exist");
        exit(10);
    }

    let mut spidev = match Spidev::open(args.dev) {
        Ok(x) => x,
        Err(e) => {
            error!("Unable to open SPI device: {e}");
            exit(11);
        }
    };

    let opts = SpidevOptions::new()
        .mode(SpiModeFlags::SPI_MODE_1)
        .build();
    match spidev.configure(&opts) {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to configure SPI device: {e}");
            exit(12);
        }
    }

    if args.once {
        let data = match read_spidev(&mut spidev, args.buffer_size) {
            Ok(x) => x,
            Err(e) => {
                error!("Failed to read SPI device: {e}");
                exit(13);
            }
        };

        info!("Read {} bytes from the SPI device", data.len());
        info!("{data:02X?}");
    } else {
        info!("Reading every {}ms", args.read_interval);

        loop {
            let data = match read_spidev(&mut spidev, args.buffer_size) {
                Ok(x) => x,
                Err(e) => {
                    error!("Failed to read SPI device: {e}");
                    exit(13);
                }
            };
            info!("{data:02X?}");

            sleep(Duration::from_millis(args.read_interval));
        }
    }
}

fn read_spidev(spidev: &mut Spidev, len: usize) -> io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(len);
    for i in 0..=len {
        buf.push(i as u8);
    }

    let read = spidev.read(&mut buf)?;

    if read == len {
        Ok(buf)
    } else {
        Ok(buf[0..read].to_vec())
    }
}

fn configure_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}