fn main() {
    let mut x: i32 = 2; 
    println!("Hello, world!");

    println!("x is {x}");
    double(&mut x);
    println!("x is {x}");
}

fn double(x: &mut i32){
    *x *= 4;
}
