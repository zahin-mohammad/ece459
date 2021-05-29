## Running the Program
Run the main program via `cargo run --release -- <mode> <filename>.txt`.
Example filename is 1.txt. File must exist in both solved/ and inputfiles/.
```bash
cargo run --release -- solve 500.txt  
cargo run --release -- verify 500.txt  
```
To automate the testing of all `input files` and all `solved` files run the following bash script.
It will build the project and go through all the file's with `hyperfine`. Comment out the tests that you wish to run.
```bash
./benchmark.sh
```



## Notes From Piazza
In regard to submitting the lab. [Piazza](https://piazza.com/class/kiz072yjgnk57a?cid=92_f1).
>Yes, you push your code to gitlab and that's all you need to do.

In regard to how many puzzles to use for benchmarking. [Piazza](https://piazza.com/class/kiz072yjgnk57a?cid=84).
> If the server is stable, you should have no problem benchmarking with 1000.txt (and will probably get more accurate results than with 100.txt). 
If for some reason (server stability, for example) you are unable to reliably run 1000.txt, then use 100.txt and mention it in your report.

In regard to what to include in part two of the report. [Piazza](https://piazza.com/class/kiz072yjgnk57a?cid=73).
>A brief explanation of your code (basically, the changes you made to convert to multi) would be appropriate.

Max connections should be 100. [Piazza](https://piazza.com/class/kiz072yjgnk57a?cid=59_f5).

You can run the sudoku verifier locally if remote is down. [Github](https://github.com/jzarnett/sudoku-verifier-rust), [Piazza](https://piazza.com/class/kiz072yjgnk57a?cid=59_f1).
```bash
cargo +nightly run --release # will bind on the local interface(s) at port 4590
```

In regard to what to include in part one of the report. [Piazza](https://piazza.com/class/kiz072yjgnk57a?cid=36).
>You should describe the algorithm you used, and how it works.
There are some ways of optimizing most algorithms, so even if you didn't do any optimization (since none is required) you should mention possible improvements.

Implementation of `read_puzzle`. Courtesy of [Bernie](https://piazza.com/class/kiz072yjgnk57a?cid=33_f2).
```rust
pub fn read_puzzle(reader: &mut impl Read) -> Option<Box<Sudoku>> {
    // Turn the input stream into an iterator of bytes
    let mut bytes = reader.bytes().map(|b| b.expect("input error")).peekable();
    // Go thru the input until we find a puzzle or EOF (None)
    loop {
        match bytes.peek() {
            Some(b'1'..=b'9') | Some(b'.') => break,
            None => return None,
            _ => {
                bytes.next();
            }
        }
    }

    let mut puzzle = Box::new([[None; 9]; 9]);

    // Fill in the puzzle matrix. Ignore the non-puzzle input bytes.
    for i in 0..9 {
        let mut j = 0;
        while j < 9 {
            let b = bytes.next().expect("unexpected EOF");

            let elem = match b {
                b'1'..=b'9' => NonZeroU8::new(b - b'0'),
                b'.' => None,
                _ => continue,
            };
            puzzle[i][j] = elem;
            j += 1;
        }
    }

    Some(puzzle)
}
```