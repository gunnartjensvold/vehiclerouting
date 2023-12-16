struct Node {
    id: u16, // Index in distance matrix
    duration: i32, // Duration required to serve
}

struct Vehicle {
    id: u16,
    nodes: Vec<Node>
}

struct Graph {
    distance_matrix : Vec<Vec<f32>>
}

fn main() {

    let mut solution: Vec<i32> = vec![0,0,1,2,3];

    for node in solution.iter(){
        println!("{node}");
    }

    solution[0] = 1;
    double(&mut solution);

    println!("");

    for node in solution.iter(){
        println!("{node}");
    }
}

fn double(x: &mut Vec<i32>){
    x[0] = 3;
}
