use ndarray::*;

fn main() {
    println!("Hello, Rust!");
    
    let arr1 = Array1::from_vec(vec![1,2,3,4,5]);
    println!("Array1: {:?}", arr1);
    
    let x = ArrayD::from_shape_vec(
        vec![2,2,2],
        vec![1,2,3,4,5,6,7,8]
    ).unwrap();
    println!("ArrayD squared: {:?}", x.map(|x| x * x));
    
    let arr2 = Array2::from_shape_vec((3,3), vec![1,2,3,4,5,6,7,8,9]).unwrap();
    println!("arr2[0,1]: {}", arr2[[0,1]]);
    
    let arr3 = Array2::from_shape_vec((2,3), vec![1,2,3,4,5,6]).unwrap();
    println!("Array2 example: {:?}", arr3);
    
}