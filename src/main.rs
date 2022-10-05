pub mod a{
    pub mod b{
        pub mod c{
            pub mod d{
                pub fn e(){}
            }
        }
    }
}

use a::b::c::d;

fn main() {
    //add e function different way
    //a::b::c::d::e();

    d::e();


}
