/// contains all TwHolderPlace related function and component
use amethyst::ecs::prelude::{Component, DenseVecStorage, };


#[derive(PartialEq, Debug)]
pub struct TwPlaceHolder {
    pub twimage_path: String,
    pub to_cache: bool,
    pub from_next: bool,
}

impl Component for TwPlaceHolder {
    type Storage = DenseVecStorage<Self>;
}


