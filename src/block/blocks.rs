/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * May 9, 2024
 *
 * A module to define Blocks available in the game.
 */

use std::fmt::{Debug, Display};

use super::{
    atlas::{BlockAtlas, BlockTextureConfig},
    cobblestone::Cobblestone,
    flower::Flower,
    mushroom::Mushroom,
    sponge::Sponge,
    wood::Wood,
    wool::Wool,
};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum Block {
    #[default]
    Air,
    Stone,
    Cobblestone {
        variant: Cobblestone,
    },
    Dirt,
    Grass,
    Bedrock,
    Log {
        variant: Wood,
    },
    Planks {
        variant: Wood,
    },
    Leaves {
        variant: Wood,
    },
    Water,
    Lava,
    Sand,
    Gravel,
    CoalOre,
    IronOre,
    GoldOre,
    DiamondOre,
    Coal,
    Iron,
    Gold,
    Diamond,
    Sponge {
        variant: Sponge,
    },
    Glass,
    Wool {
        variant: Wool,
    },
    SmoothStone,
    DoubleSlab,
    Slab,
    Bricks,
    Dynomite,
    Bookshelf,
    Obsidian,
    Sapling {
        variant: Wood,
    },
    Flower {
        variant: Flower,
    },
    Mushroom {
        variant: Mushroom,
    },
}

impl Block {
    pub fn opaque(&self) -> bool {
        !matches!(
            self,
            Self::Air
                | Self::Water
                | Self::Lava
                | Self::Glass
                | Self::Sapling
                | Self::Flower
                | Self::Mushroom
                | Self::Slab
        )
    }

    pub fn atlas_position(&self) -> Option<BlockTextureConfig> {
        match self {
            Self::Air => None,
            Self::Stone => Some(BlockTextureConfig::new().set(BlockAtlas::Stone)),
            Self::Cobblestone { variant } => match variant {
                Cobblestone::Normal => Some(BlockTextureConfig::new().set(BlockAtlas::Cobblestone)),
                Cobblestone::Mossy => {
                    Some(BlockTextureConfig::new().set(BlockAtlas::MossyCobblestone))
                }
            },
            Self::Dirt => Some(BlockTextureConfig::new().set(BlockAtlas::Dirt)),
            Self::Grass => Some(
                BlockTextureConfig::new()
                    .set(BlockAtlas::Dirt)
                    .top(BlockAtlas::Grass),
            ),
            Self::Cobblestone => Some(BlockTextureConfig::new().set(BlockAtlas::Cobblestone)),
            Self::Water => Some(
                BlockTextureConfig::new()
                    .set(BlockAtlas::Water)
                    .animate(BlockAtlas::Water0, BlockAtlas::Water0),
            ),
            Self::Lava => Some(
                BlockTextureConfig::new()
                    .set(BlockAtlas::Lava)
                    .animate(BlockAtlas::Lava0, BlockAtlas::Lava0),
            ),
            Self::Bedrock => Some(BlockTextureConfig::new().set(BlockAtlas::Bedrock)),
            Self::Log { variant } => match variant {
                Wood::Oak => Some(
                    BlockTextureConfig::new()
                        .set(BlockAtlas::OakLogSide)
                        .top(BlockAtlas::OakLogInner)
                        .bottom(BlockAtlas::OakLogInner),
                ),
                Wood::Birch => Some(
                    BlockTextureConfig::new()
                        .set(BlockAtlas::BirchLogSide)
                        .top(BlockAtlas::BirchLogInner)
                        .bottom(BlockAtlas::BirchLogInner),
                ),
            },
            Self::Planks { variant } => match variant {
                Wood::Oak => Some(BlockTextureConfig::new().set(BlockAtlas::OakPlanks)),
                Wood::Birch => Some(BlockTextureConfig::new().set(BlockAtlas::BirchPlanks)),
            },
            Self::Leaves { variant } => match variant {
                Wood::Oak => Some(BlockTextureConfig::new().set(BlockAtlas::OakLeaves)),
                Wood::Birch => Some(BlockTextureConfig::new().set(BlockAtlas::BirchLeaves)),
            },
            Self::Sand => Some(BlockTextureConfig::new().set(BlockAtlas::Sand)),
            Self::Gravel => Some(BlockTextureConfig::new().set(BlockAtlas::Gravel)),
            Self::CoalOre => Some(BlockTextureConfig::new().set(BlockAtlas::CoalOre)),
            Self::IronOre => Some(BlockTextureConfig::new().set(BlockAtlas::IronOre)),
            Self::GoldOre => Some(BlockTextureConfig::new().set(BlockAtlas::GoldOre)),
            Self::DiamondOre => Some(BlockTextureConfig::new().set(BlockAtlas::DiamondOre)),
            Self::Coal => Some(BlockTextureConfig::new().set(BlockAtlas::CoalBlock)),
            Self::Iron => Some(BlockTextureConfig::new().set(BlockAtlas::IronBlock)),
            Self::Gold => Some(BlockTextureConfig::new().set(BlockAtlas::GoldBlock)),
            Self::Diamond => Some(BlockTextureConfig::new().set(BlockAtlas::DiamondBlock)),
            Self::Sponge { variant } => match variant {
                Sponge::Dry => Some(BlockTextureConfig::new().set(BlockAtlas::SpongeDry)),
                Sponge::Wet => Some(BlockTextureConfig::new().set(BlockAtlas::SpongeWet)),
            },
            Self::Glass => Some(BlockTextureConfig::new().set(BlockAtlas::Glass)),
            Self::Wool { variant } => match variant {
                Wool::White => Some(BlockTextureConfig::new().set(BlockAtlas::WoolWhite)),
                Wool::LightGray => Some(BlockTextureConfig::new().set(BlockAtlas::WoolLightGray)),
                Wool::Gray => Some(BlockTextureConfig::new().set(BlockAtlas::WoolGray)),
                Wool::Black => Some(BlockTextureConfig::new().set(BlockAtlas::WoolBlack)),
                Wool::Brown => Some(BlockTextureConfig::new().set(BlockAtlas::WoolBrown)),
                Wool::Red => Some(BlockTextureConfig::new().set(BlockAtlas::WoolRed)),
                Wool::Orange => Some(BlockTextureConfig::new().set(BlockAtlas::WoolOrange)),
                Wool::Yellow => Some(BlockTextureConfig::new().set(BlockAtlas::WoolYellow)),
                Wool::Lime => Some(BlockTextureConfig::new().set(BlockAtlas::WoolLime)),
                Wool::Green => Some(BlockTextureConfig::new().set(BlockAtlas::WoolGreen)),
                Wool::Cyan => Some(BlockTextureConfig::new().set(BlockAtlas::WoolCyan)),
                Wool::LightBlue => Some(BlockTextureConfig::new().set(BlockAtlas::WoolLightBlue)),
                Wool::Blue => Some(BlockTextureConfig::new().set(BlockAtlas::WoolBlue)),
                Wool::Purple => Some(BlockTextureConfig::new().set(BlockAtlas::WoolPurple)),
                Wool::Magenta => Some(BlockTextureConfig::new().set(BlockAtlas::WoolMagenta)),
                Wool::Pink => Some(BlockTextureConfig::new().set(BlockAtlas::WoolPink)),
            },

            Self::SmoothStone => Some(BlockTextureConfig::new().set(BlockAtlas::SmoothStone)),
            Self::DoubleSlab => Some(
                BlockTextureConfig::new()
                    .set(BlockAtlas::Slab)
                    .top(BlockAtlas::SmoothStone)
                    .bottom(BlockAtlas::SmoothStone),
            ),
            Self::Slab => Some(BlockTextureConfig::new().set(BlockAtlas::Slab)),
            Self::Bricks => Some(BlockTextureConfig::new().set(BlockAtlas::Bricks)),
            Self::Dynomite => Some(
                BlockTextureConfig::new()
                    .set(BlockAtlas::TNTSide)
                    .top(BlockAtlas::TNTTop)
                    .bottom(BlockAtlas::TNTBottom),
            ),
            Self::Bookshelf => Some(
                BlockTextureConfig::new()
                    .set(BlockAtlas::Bookshelf)
                    .top(BlockAtlas::Planks)
                    .bottom(BlockAtlas::Planks),
            ),
            Self::Obsidian => Some(BlockTextureConfig::new().set(BlockAtlas::Obsidian)),
            Self::Sapling { variant } => match variant {
                Wood::Oak => Some(BlockTextureConfig::new().set(BlockAtlas::SaplingOak)),
                Wood::Birch => Some(BlockTextureConfig::new().set(BlockAtlas::SaplingBirch)),
            },
            Self::Flower { variant } => match variant {
                Flower::Rose => Some(BlockTextureConfig::new().set(BlockAtlas::FlowerRose)),
                Flower::Dandelion => {
                    Some(BlockTextureConfig::new().set(BlockAtlas::FlowerDandelion))
                }
            },
            Self::Mushroom { variant } => match variant {
                Mushroom::Brown => Some(BlockTextureConfig::new().set(BlockAtlas::MushroomBrown)),
                Mushroom::Red => Some(BlockTextureConfig::new().set(BlockAtlas::MushroomRed)),
            },
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "Air"),
            Self::Stone => write!(f, "Stone"),
            Self::Cobblestone => write!(f, "Cobblestone"),
            Self::Dirt => write!(f, "Dirt"),
            Self::Grass => write!(f, "Grass"),
            Self::Cobblestone => write!(f, "Cobblestone"),
            Self::Water => write!(f, "Water"),
            Self::Sand => write!(f, "Sand"),
            Self::Gravel => write!(f, "Gravel"),
            Self::Bedrock => write!(f, "Bedrock"),
            Self::Log => write!(f, "Log"),
            Self::Planks => write!(f, "Planks"),
            Self::Leaves => write!(f, "Leaves"),
            Self::Lava => write!(f, "Lava"),
            Self::GoldOre => write!(f, "Gold Ore"),
            Self::IronOre => write!(f, "Iron Ore"),
            Self::CoalOre => write!(f, "Coal Ore"),
            Self::DiamondOre => write!(f, "Diamond Ore"),
            Self::Gold => write!(f, "Gold"),
            Self::Iron => write!(f, "Iron"),
            Self::Coal => write!(f, "Coal"),
            Self::Diamond => write!(f, "Diamond"),
            Self::Sponge => write!(f, "Sponge"),
            Self::Glass => write!(f, "Glass"),
            Self::Wool => write!(f, "Wool"),
            Self::Sapling => write!(f, "Sapling"),
            Self::Flower => write!(f, "Flower"),
            Self::Mushroom => write!(f, "Mushroom"),
            Self::SmoothStone => write!(f, "Smooth Stone"),
            Self::DoubleSlab => write!(f, "Double Slab"),
            Self::Slab => write!(f, "Slab"),
            Self::Bricks => write!(f, "Bricks"),
            Self::Dynomite => write!(f, "Dynamite"),
            Self::Bookshelf => write!(f, "Bookshelf"),
            Self::Obsidian => write!(f, "Obsidian"),
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ 'name' : '{}' ]", self)
    }
}
