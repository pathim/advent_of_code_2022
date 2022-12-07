mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;

pub fn get_funcs() -> Vec<crate::AocFun> {
    vec![d01::f, d02::f, d03::f, d04::f, d05::f, d06::f]
}
