pub enum BlockAtlas {
    Stone,
    Cobblestone,
    Dirt,
    Grass,
    Bedrock,
    OakLogSide,
    OakLogInner,
    BirchLogSide,
    BirchLogInner,
    OakPlanks,
    BirchPlanks,
    OakLeaves,
    BirchLeaves,
    Water0,
    Lava0,
    Sand,
    Gravel,
    CoalOre,
    IronOre,
    GoldOre,
    DiamondOre,
    CoalBlock,
    IronBlock,
    GoldBlock,
    DiamondBlock,
    SpongeDry,
    SpongeWet,
    Glass,
    WoolWhite,
    WoolLightGray,
    WoolGray,
    WoolBlack,
    WoolBrown,
    WoolRed,
    WoolOrange,
    WoolYellow,
    WoolLime,
    WoolGreen,
    WoolCyan,
    WoolLightBlue,
    WoolBlue,
    WoolPurple,
    WoolMagenta,
    WoolPink,
    SmoothStone,
    SlabStone,
    Brick,
    TNTSide,
    TNTTop,
    TNTBottom,
    Bookshelf,
    Obsidian,
    MossyCobblestone,
    SaplingOak,
    SaplingBirch,
    FlowerRose,
    FlowerDandelion,
    MushroomBrown,
    MushroomRed,
}

impl BlockAtlas {
    pub fn index(&self) -> (u8, u8) {
        match self {
            Self::Stone => (0, 1),
            Self::Cobblestone => (0, 2),
            Self::Dirt => (0, 3),
            Self::Grass => (0, 4),
            Self::Bedrock => (0, 5),
            Self::OakLogSide => (0, 6),
            Self::OakLogInner => (0, 7),
            Self::BirchLogSide => (1, 0),
            Self::BirchLogInner => (1, 1),
            Self::OakPlanks => (1, 2),
            Self::BirchPlanks => (1, 3),
            Self::OakLeaves => (1, 4),
            Self::BirchLeaves => (1, 5),
            Self::Water0 => (1, 6),
            Self::Lava0 => (1, 7),
            Self::Sand => (2, 0),
            Self::Gravel => (2, 1),
            Self::CoalOre => (2, 2),
            Self::IronOre => (2, 3),
            Self::GoldOre => (2, 4),
            Self::DiamondOre => (2, 5),
            Self::CoalBlock => (2, 6),
            Self::IronBlock => (2, 7),
            Self::GoldBlock => (3, 0),
            Self::DiamondBlock => (3, 1),
            Self::SpongeDry => (3, 2),
            Self::SpongeWet => (3, 3),
            Self::Glass => (3, 4),
            Self::WoolWhite => (3, 5),
            Self::WoolLightGray => (3, 6),
            Self::WoolGray => (3, 7),
            Self::WoolBlack => (4,0),
            Self::WoolBrown => (4,1),
            Self::WoolRed => (4,2),
            Self::WoolOrange => (4,3),
            Self::WoolYellow => (4,4),
            Self::WoolLime => (4,5),
            Self::WoolGreen => (4, 6),
            Self::WoolCyan => (4, 7),
            Self::WoolLightBlue => (5,0),
            Self::WoolBlue => (5,1),
            Self::WoolPurple => (5,2),
            Self::WoolMagenta => (5,3),
            Self::WoolPink => (5,4),
            Self::SmoothStone => (5, 5),
            Self::SlabStone => (5, 6),
            Self::Brick => (5, 7),
            Self::TNTSide => (6,0),
            Self::TNTTop => (6,1),
            Self::TNTBottom => (6,2),
            Self::Bookshelf => (6,3),
            Self::Obsidian => (6,4),
            Self::MossyCobblestone => (6,5),
            Self::SaplingOak => (7,0),
            Self::SaplingBirch => (7,1),
            Self::FlowerRose => (7,2),
            Self::FlowerDandelion => (7,3),
            Self::MushroomBrown => (7,4),
            Self::MushroomRed => (7,5),
        }
    }
}

pub struct BlockTextureConfig {
    right: BlockAtlas,
    left: BlockAtlas,
    top: BlockAtlas,
    bottom: BlockAtlas,
    front: BlockAtlas,
    back: BlockAtlas,
}

impl BlockTextureConfig {
    pub fn new() -> Self {
        Self {
            right: BlockAtlas::Air,
            left: BlockAtlas::Air,
            top: BlockAtlas::Air,
            bottom: BlockAtlas::Air,
            front: BlockAtlas::Air,
            back: BlockAtlas::Air,
        }
    }

    pub fn set(mut self, atlas: BlockAtlas) -> Self {
        self.set_right(atlas)
            .set_left(atlas)
            .set_top(atlas)
            .set_bottom(atlas)
            .set_front(atlas)
            .set_back(atlas)
    }

    pub fn right(mut self, atlas: BlockAtlas) -> Self {
        self.right = atlas;
        self
    }

    pub fn left(mut self, atlas: BlockAtlas) -> Self {
        self.left = atlas;
        self
    }

    pub fn top(mut self, atlas: BlockAtlas) -> Self {
        self.top = atlas;
        self
    }

    pub fn bottom(mut self, atlas: BlockAtlas) -> Self {
        self.bottom = atlas;
        self
    }

    pub fn front(mut self, atlas: BlockAtlas) -> Self {
        self.front = atlas;
        self
    }

    pub fn back(mut self, atlas: BlockAtlas) -> Self {
        self.back = atlas;
        self
    }

    pub fn animate(mut self, from: BlockAtlas, to: BlockAtlas) -> Self {
        // self.right = from;
        // self.left = to;
        self.set(from)
    }
}
