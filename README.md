<div align="center">
  <img src="https://wave-lang.dev/img/favicon.ico" alt="Loa Programming Language Logo" width="100" />
  <h1>Loa Programming Language</h1>
  <p>
    <a href="https://www.wave-lang.dev">Website</a> •
    <a href="https://github.com/LunaStev/Wave/wiki/Contributing">Contributing</a> •
    <a href="LICENSE">LICENSE</a>
  </p>
</div>

> **Warning:**  
> The official version of this project has not yet been released. The first version will be distributed as v0.0.1.

![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/LunaStev/Loa?style=for-the-badge&logo=github?utm_source=oss&utm_medium=github&utm_campaign=LunaStev%2FWave&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)

![Latest version](https://img.shields.io/github/v/release/LunaStev/Loa?style=for-the-badge&include_prereleases)

![Code size](https://img.shields.io/github/languages/code-size/LunaStev/Loa?style=for-the-badge&logo=github)
![Downloads](https://img.shields.io/github/downloads/LunaStev/Loa/total?color=%2324cc24&style=for-the-badge&logo=github)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/LunaStev/Loa/rust.yml?logo=rust&style=for-the-badge&branch=master)

---

## Overview

**Wave** is a next-generation programming language designed for developing operating systems, system software, and applications—entirely using **Wave**.  
We're building a **Wave** compiler with full hardware access.

---

## Examples

### Fibonacci sequence

```wave
fun fibonacci(n: i32) -> i32 {
    if (n == 0) {
        return 0;
    }
    
    if (n == 1) {
        return 1;
    }
    
    var prev :i32 = 0;
    var curr :i32 = 1;
    var next :i32;
    var i :i32 = 2;
    
    while (i <= n) {
        next = prev + curr;
        prev = curr;
        curr = next;
        i = i + 1;
    }
    
    return curr;
}

fun main() {
    var i :i32 = 0;
    var result :i32;
    
    while (i <= 10) {
        result = fibonacci(i);
        println("fibonacci({}) = {}", i, result);
        i = i + 1;
    }

    println("END FIBONACCI");
}
```

## Pointer and Reverse Chalk

```wave
fun main() {
    var a: i32 = 10;
    var b: i32 = 20;
    
    var p1: ptr<i32> = &a;
    var p2: ptr<i32> = &b;
    
    println("Before:");
    println("a = {}, b = {}", a, b);
    println("p1 = {}, p2 = {}", deref p1, deref p2);
    
    var temp: i32 = deref p1;
    deref p1 = deref p2;
    deref p2 = temp;
    
    println("After:");
    println("a = {}, b = {}", a, b);
    println("p1 = {}, p2 = {}", deref p1, deref p2);
}
```

More examples are available inside `test/`.

---

## Concept

<p align="center">
  <img src=".github/scalability1.svg" alt="Wave Concept Diagram" width="60%">
</p>

---

## Sponsors ❤️

A huge thank you to our sponsors for supporting this project!

<p align="center">
  <a href="https://ko-fi.com/heymanbug">
    <img src="https://ko-fi.com/img/anon7.png?v=10" width="100" alt="heymanbug" />
    <br>
    <sub><b>heymanbug</b></sub>
  </a>
</p>

---

## Contributing

Interested in contributing? Check out our [Contributing Guide](https://github.com/LunaStev/Wave/wiki/Contributing) to get started.

---

## Sponsor Us

Wave is an open-source project made possible by community support.  
Consider sponsoring us to help maintain and improve the language.

<p align="center">
  <a href="https://ko-fi.com/X8X311B3SX">
    <img src="https://ko-fi.com/img/githubbutton_sm.svg" alt="Sponsor us on Ko-fi" />
  </a>
</p>

---

## Cool graphs

[![Star History Chart](https://api.star-history.com/svg?repos=LunaStev/Wave&type=Date)](https://star-history.com/#LunaStev/Wave&Date)

---

## LICENSE

Wave is released under the [MPL-2.0 License](LICENSE).
