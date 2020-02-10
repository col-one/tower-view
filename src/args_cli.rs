use structopt::StructOpt;
use std::iter::Iterator;

#[derive(Debug, StructOpt)]
/// tower is an image viewer software made for animation and vfx.
#[structopt(name = "Tower", about = "Tower is an opensource project made with rust.")]
pub struct Opt {
    /// Input images
    #[structopt(required=true, multiple=true, number_of_values=1)]
    pub inputs: Vec<String>,
}


