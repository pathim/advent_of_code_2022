mod d01;
mod d02;
mod d03;

pub fn get_funcs() -> Vec<crate::AocFun> {
    vec![d01::f, d02::f, d03::f]
}
