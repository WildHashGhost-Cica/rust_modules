

pub mod a{
    pub mod b{
        pub mod c{
            pub mod d{
                pub fn e(){}
            }
        }
    }
}
enum Ex{
        A,
        B,
        C,
}

use a::b::c::d;
use Ex::{A,C};
//or
use Ex::*;

fn main() {
    //add e function different way
    //a::b::c::d::e();

    d::e();
    Ex::B;
    A; //use Ex::{A,C}
    C; //use Ex::{A,C}


}
