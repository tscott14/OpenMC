use crate::{block::blocks::Block, utils::position::{self, global::{GlobalPosition, GlobalUnit}}};


pub struct Strata (GlobalUnit, GlobalUnit, Block);
pub struct StrataDescription(Vec<Strata>);

impl StrataDescription {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, strata: Strata) {
        self.0.push(strata);
    }

    pub fn get(&self, position: GlobalPosition) -> Block {
        let (_x, y, _z) = position.into();
        for strata in self.0.iter() {
            if (strata.0.into()..strata.1.into()).contains(&y) {
                return strata.2;
            }
        }

        Block::Air
    }
}