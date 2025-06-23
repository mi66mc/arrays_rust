mod array;

use array::RawArray;

fn main() {
    let mut array = RawArray::new(8);

    array.push(2);

    println!("{:?}", array);
}
