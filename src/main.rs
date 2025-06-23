mod array;

use array::RawArray;

fn main() {
    let mut array: RawArray<&str> = RawArray::new(2);

    array.push("2");
    array.push("5");

    println!("{:?}", array.get(0));
    println!("{:?}", array);
    
    array.drop_last();

    println!("{:?}", array.get(0));
    println!("{:?}", array);
}
