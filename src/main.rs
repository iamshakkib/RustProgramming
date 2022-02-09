fn main() {
    let mut _x:u64 = 45; //varibles are by default mutable in rust make it mutable by mut keyword
    let mut _f:f64 = 65.9;
    println!("The value of f is {}",_f );
    _f = 60.8;
    println!("The value of f is {}",_f );
    let b = false;
    if b {
        println!("true");
    }else{
        println!("false");
    }
}