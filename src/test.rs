
extern crate std;
use std::println;
use std::boxed::Box;

use super::*;



#[test]
fn test() {
    const SIZE:usize = 64;
    let holder = Box::into_raw(Box::new([0u32;SIZE]));
    let mut array:Array<u32, SIZE> = Array::new(holder as *mut u32);
    
    for number in 0..5u32 {
        let _ = array.append(number);
        println!("{:?}", array.len()-1);
    }
    
    println!("{:#?}", &array);
    assert_eq!(&array[..], &[0,1,2,3,4]);
    
}

