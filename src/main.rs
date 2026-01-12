use ndarray::Array2;

fn main() {
    println!("Hello, Rust!");
    println!("\n");
    
    let arr = Array2::from_shape_vec((2,3), vec![1,2,3,4,5,6]).unwrap();
    println!("The array is {:?}",arr);
}