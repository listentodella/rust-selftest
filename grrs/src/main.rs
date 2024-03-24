use log::{info, warn};
use anyhow::{Context, Result};
use structopt::StructOpt;

use indicatif::{ProgressBar, ProgressIterator};

use std::{thread, time::Duration};

use signal_hook::{consts::SIGINT, iterator::Signals};

#[derive(Debug,StructOpt)]
struct Cli {
    /// the pattern to look for
    pattern:String,
    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    path:std::path::PathBuf
}


fn main()->Result<()> {
    env_logger::init();
    let args = Cli::from_args();
    info!("{:#?}", args);
    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file"))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            //println!("=>>{}", line);
            info!("=>{}", line);
        }
    }

    //let spinner = ProgressBar::new_spinner();
    //spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    let bar = ProgressBar::new(100);
    // for _ in (0..100).progress() {
        // spinner.inc(1)
    // }
    for _ in 0..50 {
        bar.inc(1);
    }



    bar.finish();
    //spinner.finish();


    // ctrlc::set_handler(move || {
    //     info!("received Ctrl + C !");
    // }).expect("Error setting Ctrl+C handler!");
    let mut signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            warn!("Recv signal {:#?}", sig);
        }
    });



    loop {
    }




    Ok(())
}
