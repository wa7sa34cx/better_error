#[macro_use]
mod error;
mod module;

fn main() {
    let err = module::run();
    println!("{:?}", err);
    
    let err = internal!();
    println!("{:?}", err);
}
