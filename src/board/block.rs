//use crate::board::Board;

// N E W S permutations for each block_type
// public facing api
pub enum BlockType{
    RightL,
    LeftL,
    RightZ,
    LeftZ,
    Line,
    T,
    Square,
}

/// BlockType -> Factory -> reference to block; used when describing the blocks allowed in the tetris
/// game.
pub fn block_factory(block_type: BlockType) -> &'static dyn Shape {
    match block_type {
        BlockType::Square => {
           &Square{}
        },
        BlockType:: T => {
            &T{}
        },
        //BlockType:: Line => Box::new(L::new()),
        _ => panic!("Shape does not exist!")
    }
}

// gets the 3x3 grid placement of the shape
pub trait Shape {
    fn get_config(&self) -> [[i8;3]; 3];
    fn get_size(&self) -> usize;
}

struct Square {}
impl Shape for Square {
    fn get_config(&self) -> [[i8; 3]; 3] {
        [[1,1,0],
         [1,1,0],
         [0,0,0]]
    }
    fn get_size(&self) -> usize {
        let n: usize = 3;
        return n
    }
}

impl Square {
    fn new() -> Self {
        Self{}
    }
}

struct T {}
impl Shape for T {
    fn get_config(&self) -> [[i8; 3]; 3] {
        [[1,1,1],
         [0,1,0],
         [0,0,0]]
    }
    fn get_size(&self) -> usize {
        let n: usize = 3;
        return n
    }
}
impl T{
    fn new() -> Self {
        Self{}
    }
}

// struct Line {}
// impl Shape for Line {
//     fn get_config(&self) -> [[i8; 4]; 4] {
//         [[1,0,0,0],
//          [1,0,0,0],
//          [1,0,0,0],
//          [1,0,0,0]]
//
//     }
//     fn get_size(&self) -> usize {
//         let n: usize = 4;
//         return n
//     }
// }
// impl Line {
//     fn new() -> Self {
//         Self{}
//     }

// pub struct RightL {
//
// }
//
// impl Shape for RightL {
//     fn get_config() {
//         todo!()
//     }
// }
