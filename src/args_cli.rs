use structopt::StructOpt;
use std::iter::Iterator;

#[derive(Debug, StructOpt)]
/// tower is a new kind of image viewer software, adjust your images like a mood-board,
/// compare, move, view them freely.
#[structopt(name = "Tower", about = "Tower is an open source project made with rust and powered by Amethyst\n\
It's a multi image viewer that allow to manipulate and adjust, like a mood board, several images at same time.")]
pub struct Opt {
    /// Input images
    #[structopt(required=false, multiple=true, number_of_values=1)]
    pub inputs: Vec<String>,
}


