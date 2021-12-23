use rand::Rng;
use rand::seq::index::sample;

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

// Update the color_gen function if more colors are added or removed
#[derive(Copy, Clone)]
pub enum Color {
    Unassigned,
    Blue,
    Red,
    Purple,
    Green,
    Yellow,
}

pub fn color_gen() -> Color {
    const COLOR_COUNT: i32 = 5; // number of possible colors
    let mut random = rand::thread_rng();
    match random.gen_range(1..COLOR_COUNT) {
        1 => Color::Blue,
        2 => Color::Red,
        3 => Color::Purple,
        4 => Color::Green,
        _ => Color::Yellow,
    }
}

/// BlockType -> Factory -> reference to block; used when describing the blocks allowed in the tetris
/// game.
pub fn block_factory(block_type: BlockType) -> &'static dyn Shape {
    match block_type {
        BlockType::Square => {
            // TODO: FIX this later to not unassigned
           &Square{color: Color::Unassigned}
        },
        // BlockType:: T => {
        //     &T{}
        // },
        //BlockType:: Line => Box::new(L::new()),
        _ => panic!("Shape does not exist!")
    }
}

// gets the 3x3 grid placement of the shape
pub trait Shape {
    fn get_config(&self) -> [[i8;3]; 3];
    fn get_size(&self) -> usize;
    fn get_color(&self) -> Color;
}

struct Square {
    color: Color,
}

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
    fn get_color(&self) -> Color {
        self.color
    }
}

impl Square {
    fn new() -> Self {
        //TODO: fis this
        Self{color: Color::Unassigned}
    }
}

// struct T {}
// impl Shape for T {
//     fn get_config(&self) -> [[i8; 3]; 3] {
//         [[1,1,1],
//          [0,1,0],
//          [0,0,0]]
//     }
//     fn get_size(&self) -> usize {
//         let n: usize = 3;
//         return n
//     }
// }
// impl T{
//     fn new() -> Self {
//         Self{}
//     }
// }

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
