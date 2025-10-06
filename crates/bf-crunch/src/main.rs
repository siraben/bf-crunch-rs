mod cruncher;
mod math;
mod node;
mod options;
mod path;
mod solver;
mod util;

use anyhow::Result;
use clap::Parser;

use crate::cruncher::Cruncher;
use crate::options::Options;

fn main() -> Result<()> {
    let opts = Options::parse();
    let mut cruncher = Cruncher::new(&opts)?;

    if let Some(max_init) = opts.max_init {
        for len in opts.min_init..=max_init {
            cruncher.crunch(len);
        }
        Ok(())
    } else {
        let mut len = opts.min_init;
        loop {
            cruncher.crunch(len);
            len += 1;
        }
    }
}
