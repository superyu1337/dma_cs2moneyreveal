use clap::Parser;

use crate::{cli::Cli, dma::DmaCtx};

mod pattern;
mod memory;
mod dma;
mod cli;

fn main() -> anyhow::Result<()> {

    let cli: Cli = Cli::parse();

    let mut dma_ctx = DmaCtx::setup(cli.connector, cli.pcileech_device)?;

    let is_hltv_address = memory::get_function(&mut dma_ctx.process, &dma_ctx.client)?;
    let original = memory::patch(&mut dma_ctx.process, is_hltv_address)?;

    let (tx, rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");

    memory::restore(&mut dma_ctx.process, is_hltv_address, original)?;

    Ok(())
}