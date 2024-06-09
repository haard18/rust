- all the variables in rust have types and can be inferred by the compiler
- i32 is a 32 bit integer
- i depicts signed integer
- boolean type is bool
- it has two values true and false
-  println!("Hello, world!");
- string are very complicated in rust
-  let greeting=String::from("Hello World");
-  println!("{}",greeting);
-  let char0=greeting.chars().nth(0);
- option type is used to handle null values
- we have to unwrap the option type to get the value
-  println!("{}",char0.unwrap());
-  print!("{}",greeting.chars().nth(0));
- conditional loops
``` 
   let x=10;
    if x>=10{
        println!("x is greater than 10 or equal to 10");
    }
    else{
        println!("x is less than 10");
    }
    //looping in rust
    for i in 0..10{
        println!("{}",i);
    }
```
# Mutability
- ALL VARIABLES IN RUST ARE IMMUTABLE BY DEFAULT
- Mutable variables are created using the mut keyword

# Memory Management
- Rust Uses both stack and heap memory
![alt text](image.png)
- Stack memory is used for static memory allocation
- stack memory is faster than heap memory
- It is used for primitive data types
- Heap memory is used for dynamic memory allocation
- It is used for storing data of unknown size at compile time
- Heap memory is slower than stack memory
# Ownership
- when a variable goes out of scope it is deallocated
- Rust has a concept of ownership
- Ownership is a way of managing memory in rust
- Ownership rules
- Each value in rust has a variable that is called its owner
- There can only be one owner at a time
- When the owner goes out of scope the value will be deallocated
```
fn main(){
    let mut s=String::from("Hello");
    let s2=s1;
    //here s1 is moved to s2
}
```
## Borrowing
- Borrowing is a way to pass a reference
- Can have multiple Borrowers
- Mutable references are not allowed to have multiple borrowers

# Structs
- Structs are used to create custom data types
- Structs are used to create complex data types
- Objects are created using structs
- Structs can be implemented using the impl keyword
# Enums
- Enums are used to create custom data types
- Enums are used to create complex data types
- Enums can have multiple variants