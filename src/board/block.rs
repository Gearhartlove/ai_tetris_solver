//use crate::board::Board;

// N E W S permutations for each block_type
pub enum BlockType{
    RightL,
    LeftL,
    RightZ,
    LeftZ,
    Line,
    T,
    Square,
}

// gets the 3x3 grid placement of the shape
pub trait Shape {
    fn get_config() -> [[i8;3];3] {
        [[0,0,0],
         [0,0,0],
         [0,0,0]]
    }
}

pub struct Square {
}

impl Shape for Square {
    fn get_config() -> [[i8; 3]; 3] {
        todo!()
    }
}

// pub struct RightL {
//
// }
//
// impl Shape for RightL {
//     fn get_config() {
//         todo!()
//     }
// }
