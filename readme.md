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
