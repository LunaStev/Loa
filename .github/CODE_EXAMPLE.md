# Wave Programming Language - Code Examples and Explanation

This document provides a series of code examples written in the Wave programming language. Each example demonstrates a specific feature or concept in the language, accompanied by a detailed explanation.

## Input/Output
```
fun main() {
    print("Hello World?\n");
    println("Hello World!");
}
```
### Explanation:
* `fun main()`: This defines the main function, which serves as the entry point for the program. 
In Wave, the `main` function is where the execution of the program begins, just like in C or Rust.

* `print("Hello World?\n");`: The `print` function outputs the string `"Hello World?"` without appending a newline character at the end. 
The `\n` ensures the output is followed by a newline.

* `println("Hello World!");`: The `println` function is used to print the string `"Hello World!"` to the console, followed by a newline character.

* Output:
```
Hello World?
Hello World!
```

This example demonstrates how to use `print` for output without a newline and `println` for output with a newline.

## Variables and Conditionals
```
fun main() {
    var x: i32 = 5;
    var y: i32 = 10;
    
    if (x < y) {
        println("x is less than y");
    } else {
        println("x is greater than or equal to y");
    }
}
```
### Explanation:
* `var x: i32 = 5;` and `var y: i32 = 10;`: These lines declare two variables, `x` and `y`, of type `i32` (32-bit integer), 
and assign them initial values of 5 and 10, respectively.

* `if (x < y) { ... } else { ... }`: This is and `if-else` conditional statement.
It checks if the value of `x` is less than `y`. If the condition is true, it prints `x is less than y`.
Otherwise, it prints `x is greater than or equal to y`.

* Output:
```
x is less than y
```

This example demonstrates how to declare variables, compare them, and use conditional logic.

## Loops
### `while` Loop
```
fun main() {
    var i: i32 = 0;
    
    while (i < 5) {
        println(i);
        i = i + 1;
    }
}
```

#### Explanation:
* `var i: i32 = 0;`: This line initializes a variable `i` with the value 0.

* `while (i < 5) { ... }`: The `while` loop continues executing the code block as long as the condition `i < 5` holds true.
Inside the loop, the current value of `i` is printed, and `i` is incremented by 1.

* Output:
```
0
1
2
3
4
```

### `for` Loop
```
fun main() {
    for (i in 0..4) {
        println(i);
    }
}
```

#### Explanation:
* `for (i in 0..4) { ... }`: The `for` loop iterates over the range `0..4`, effectively printing the values of `i` from 0 to 4.

* Output:
```
0
1
2
3
4
```

This example demonstrates how the same task can be accomplished using a `for` loop, iterating through a specified range instead of manually managing the loop condition.

## Functions
```
fun greet(name: str) {
    println("Hello, {} !", name);
}

fun main() {
    greet("Wave");
}
```

### Explanation:
* `fun greet(name: str)`: This defines a function called `greet` that takes a single parameter `name` of type `str` (String).

* `"println("Hello, {} !"), name"`: inside the function, we use a formatted string to print a greeting message that includes the value of `name`.

* `greet("Wave")`: In the `main` function, we call the `greet` function with the argument `"Wave"`, which outputs the greeting message.

* Output:
```
Hello, Wave !
```

This example demonstrate how to define and call a function with parameters in Wave, along with string formatting.

## Error Handling
```
fun divide(a: i32, b: i32) -> i32 {
    if (b == 0) {
        println("Error: Divison by zero");
        return -1;
    }
    return a / b;
}

fun main() {
    var result: i32 = divide(10, 2);
    
    if result != -1 {
        println("Result: {}", result);
    }
}
```

### Explanation

* `fun divide(a: i32, b: i32) -> i32`: This defines a function called `divide` that takes two integers as parameters and returns an integer result.
* `if (b == 0) { ... }`: Inside the function, we check if `b` is zero. If it is, we print an error message and return `-1` to indicate an error. Otherwise, the function performs the division and returns the result.
* `var result: i32 = divide(10, 2)`: In the `main` function, we call `divide` with the arguments `10` and `2`, storing the result in the variable `result`.
* `if result != -1 { ... }`: We then check if the result is not equal to `-1` (indicating an error) and, if valid, print the result.

* Output:
```
Result: 5
```

This example demonstrates how to handle errors, such as division by zero, and return an error value.

## Arrays
```
fun main() {
    var arr = [1, 2, 3, 4, 5];
    
    for (num in arr) {
        println(num);
    }
}
```

### Explanation:

* `var arr = [1, 2, 3, 4, 5];`: This creates an array named `arr` containing the integers from 1 to 5.

* `for (num in arr) { ... }`: The `for` loop iterates over each element in the array `arr`. In each iteration, the current element is stored in the variable `num`.

* `println(num);`: Inside the loop, the current value of `num` is printed.

* Output:
```
1
2
3
4
5
```

This example demonstrates how to declare an array and iterate through its elements using a `for` loop.