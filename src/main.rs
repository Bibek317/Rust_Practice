use ndarray::*;

fn main() {
    println!("Hello, Rust!");
    
    let arr1 = Array1::from_vec(vec![1,2,3,4,5]);
    println!("The array is {:?}", arr1);
    
    // let x = arr1.mapv(|v| v.sin());
    let x = ArrayD::from_shape_vec(
        vec![2,2,2],
        vec![1,2,3,4,5,6,7,8]
    ).unwrap();
    println!("{:?}", x.map(|x| x * x));
    
    let arr2 = Array2::from_shape_vec(
        (3,3),
        vec![1,2,3,4,5,6,7,8,9]
    ).unwrap();//.for_each(&|x|print!("{x} "));
    
    println!("{}", arr2[[0,1]]);
    
}