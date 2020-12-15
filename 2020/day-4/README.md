# Day 4 - Passport Processing

For this day I improved my basic rust code somewhat. I had previously been using lots of nesting and use of `unwrap()` to get around the `Option` & `Result` types returned by many of Rust's fundamental methods.

After reading this [excellent blog post](https://qubyte.codes/blog/parsing-input-from-stdin-to-structures-in-rust) I realised how much cleaner and easier it is to use methods such as `filter_map` to iterate through only lines that were successfully read in. 
Previously I was reading from a file whose name was hardcoded. This meant copy and pasting this method into every solution:
```
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```
This value also had to be unwrapped. Even more nesting!
`if let Ok(lines) = read_lines("./input"){ ... }`

Instead the `filter_map` method lets us use a closure to easily return only valid lines within an iterator.
In addition to using `filter_map` I am also following qubyte's blog post in using `stdin` as my input instead of loading and reading the file. This is done by calling `cargo run < file`.

This is my code:

```
let buf = stdin();
let lines = buf.lock().lines().filter_map(|line_result| line_result.ok());
```
Note that I can't just put that all on a single line. If you do then `error[E0716]: temporary value dropped while borrowed` is thrown. 
I believe this is because the lock & lines methods require ownership of `stdin()`.

The solution uses a *lot* of Regex statements, particularly Part 2. I have reused methods parsing the numbers but the code is fairly cumbersome and its something I will look to clean up in later solutions.
