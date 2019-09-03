//! This crate is a game engine library providing many needed functions.

pub mod boxes;
pub mod collide;
pub mod math;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
