# Building a Raw Array from Scratch in Rust

This project is a simple and educational implementation of a `RawArray` in Rust. It demonstrates how to manually manage memory and provides basic dynamic array operations.

---

## 📁 Structure

- `RawArray<T>`: A generic array-like structure that manages memory manually and supports basic operations.

---

## 🚀 Features

## RawArray<T>
> Generic type `T` for flexibility.
- `new(capacity: usize) -> Self`: Creates a new array with the given capacity.
- `push(value: T)`: Adds a value to the end of the array.
- `get(index: usize) -> Option<&T>`: Returns a reference to the value at the given index, or `None` if out of bounds.
- `set(index: usize, value: T)`: Sets the value at the given index.
- `drop_last()`: Removes the last element from the array.
- `get_size() -> usize`: Returns the current number of elements in the array.
- `get_capacity() -> usize`: Returns the total capacity of the array.
- `is_empty() -> bool`: Checks if the array is empty.

---

## 🛠️ Example Usage

```rust
    let mut array: RawArray<&str> = RawArray::new(2);

    array.push("2");
    array.push("5");

    println!("{:?}", array.get(0)); // Some("2")
    println!("{:?}", array);        // ["2", "5"]

    array.drop_last();

    println!("{:?}", array.get(0)); // Some("2")
    println!("{:?}", array);        // ["2"]
```