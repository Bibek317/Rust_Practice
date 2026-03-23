mod library;
use library::math::{add, square};

use ndarray::*;

fn adds(a: i32, b: i32) -> i32 {
    a + b
}
fn subs(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    println!("Hello, Rust!");

    let arr1 = Array1::from_vec(vec![1, 2, 3, 4, 5]);
    println!("Array1: {:?}", arr1);

    let x = ArrayD::from_shape_vec(vec![2, 2, 2], vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    println!("ArrayD squared: {:?}", x.map(|x| x * x));

    let a = ArrayD::from_shape_vec(vec![2, 2, 2], vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();

    println!("ArrayD squared: {:?}", a.map(|x| x * x));

    let arr2 = Array2::from_shape_vec((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    println!("arr2[0,1]: {}", arr2[[0, 1]]);

    let arr3 = Array2::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();
    println!("Array2 example: {:?}", arr3);

    println!("Hi");

    let x = 50.5;
    println!("The results is {}", square(x));

    println!("The results is {}", square(50.5));
    println!("Result -> {}", add(5, 56));

    let x = array![[1., 2.], [3., 4.]];
    let y = array![[5., 6.], [7., 8.]];

    let dots = x.dot(&y);
    println!("The dot product is {:?}", dots);

    // 3 * 3 zero 2d matrix
    let array2d: Array2<i8> = Array2::zeros((3, 3));
    println!("{:?}", array2d);

    //identity matrix  2 * 2
    let eye2d = Array2::<i32>::eye(3);
    println!("{:?}", eye2d);

    //Use indexing
    println!("The element is {}", y[[1, 1]]);

    //using slicing
    println!("The row is {:?}", x.slice(s![0, ..]));
    println!("Array -> {:?}", eye2d.view());

    let arrfn = [adds, subs]; // Trying to put function inside array
    println!("{:?}", arrfn);

    println!(
        "Calling Add function which is stored in index => 0 , the results is == {}",
        arrfn[0](100, 50)
    );

    
}
