#![allow(non_snake_case)]
pub struct Kyzen{
    pub register: [i64;32],
    pub ula:usize,

    pub remender:usize,


}


impl Kyzen {
    pub fn new()->Kyzen{
        Kyzen{
            register:[0;32],
            ula: 0,
            remender:0,
        }
    }
}