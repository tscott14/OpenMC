/*
 * Created by Tristan Scott [tscott14+git@proton.me]
 * May 9, 2024
 *
 * A module to define Blocks available in the game.
 */

 use std::fmt::{Debug, Display};
 
  #[derive(Copy, Clone, Default, PartialEq, Eq)]
  pub enum Block {
    #[default]
     Air,
     Stone,
     Dirt,
     Grass,
     Cobblestone,
     Water,
     Sand,
     Bedrock,
 }
 
 
 impl Block {
     pub fn from_u8(code: u8) -> Self {
         match code {
             0 => Self::Air,
             1 => Self::Stone,
             2 => Self::Dirt,
             3 => Self::Grass,
             4 => Self::Cobblestone,
             5 => Self::Water,
             6 => Self::Sand,
             7 => Self::Bedrock,
             _ => panic!("Invalid block code"),
         }
     }
 
     pub fn as_u8(&self) -> u8 {
         self.clone() as u8
     }
 
     pub fn opaque(&self) -> bool {
         match self {
             Self::Air => false,
             Self::Stone => true,
             Self::Dirt => true,
             Self::Grass => true,
             Self::Cobblestone => true,
             Self::Water => false,
             Self::Sand => false,
             Self::Bedrock => true,
         }
     }
 
     pub fn transparent(&self) -> bool {
         match self {
             Self::Air => true,
             Self::Stone => false,
             Self::Dirt => false,
             Self::Grass => false,
             Self::Cobblestone => false,
             Self::Water => true,
             Self::Sand => true,
             Self::Bedrock => false,
         }
     }
 }
 
 impl Display for Block {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self {
             Self::Air => write!(f, "Air"),
             Self::Stone => write!(f, "Stone"),
             Self::Dirt => write!(f, "Dirt"),
             Self::Grass => write!(f, "Grass"),
             Self::Cobblestone => write!(f, "Cobblestone"),
             Self::Water => write!(f, "Water"),
             Self::Sand => write!(f, "Sand"),
             Self::Bedrock => write!(f, "Bedrock"),
         }
     }
 }
 
 impl Debug for Block {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         write!(f, "[{}:{}]", self, self.as_u8())
     }
 }