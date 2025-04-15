# Semechko Interpreter

<p align="center">
  A fresh look at programming through the eyes of a cat with a tiny brain.
</p>

![Маскот](maskot.jpg)

## Language
Translate rich set of Semechko instructions to Rust code.
```
рыгнуть 42;
рыгнуть 1 + 2;
```

will be transpiled to

```Rust
fn main() {
println!("{}", 42);
println!("{}", (1 + 2));
}
```

## Idea

Semechko language, as well as its inspirer, can do only one thing - do the burp.

As it is a toy project, there is no strict rules for language to function. I make this rules out of thin air every time i make changes in this codebase.

## Usage
If you are curious enough to use this thing - you can do this:

1. ```https://github.com/tinarao/semechko.git semechko```
2. ``` cd semechko ```
3. Create main.sk file in the directory, where Cargo.toml locates
4. Write some "рыгнуть()" instructions
5. ```cargo run main.sk output.rs```

Voila! You got and output.rs file with generated code.
