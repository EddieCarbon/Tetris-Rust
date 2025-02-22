use std::ops::{Index, IndexMut};

use rand::{prelude::{SliceRandom, ThreadRng}, thread_rng};
use self::piece::{Piece, Kind as PieceKind};

mod piece;

pub type Coordinate = cgmath::Point2<usize>;
pub type Offset = cgmath::Vector2<isize>;


pub struct Engine {
    matrix: Matrix,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            matrix: Matrix::blank(),
            bag: Vec::new(),
            rng: thread_rng(),
            cursor: None,
        }
    }

    fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut self.rng);
    }

    fn place_cursor(&mut self) {
        let cursor = self.cursor.take().expect("Called place_cursor withoout cursor");
        
        for coord in cursor.cells().expect("Cursor out of bounds") {
            let cell = &mut self.matrix[coord];
            debug_assert_eq!(*cell, false);
            *cell = true;
        }
    }
}


struct Matrix([bool; Self::SIZE]);

impl Matrix {
    const WIDTH: usize = 20;
    const HEIGHT: usize = 10;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn in_bounds(Coordinate { x, y }: Coordinate) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    fn indexing(Coordinate { x, y }: Coordinate) -> usize {
        y * Self::WIDTH + x
    }

    fn blank() -> Self {
        Self([false; Self::SIZE])
    }
}

impl Index<Coordinate> for Matrix {
    type Output = bool;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(Self::in_bounds(coord));
        &self.0[Self::indexing(coord)]
    }
}

impl IndexMut<Coordinate> for Matrix {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(Self::in_bounds(coord));
        &mut self.0[Self::indexing(coord)]
    }
}