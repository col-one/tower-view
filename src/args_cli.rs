use structopt::StructOpt;
use std::iter::Iterator;

#[derive(Debug, StructOpt)]
/// tower is a new kind of image viewer software, adjust your images like a mood-board,
/// compare, move, view them freely.
#[structopt(name = "Tower", about = "Tower is an opensource project made with rust.")]
pub struct Opt {
    /// Input images
    #[structopt(required=true, multiple=true, number_of_values=1)]
    pub inputs: Vec<String>,
}


