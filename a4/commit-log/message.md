# Title

20x Speedup: file I/O, prints, channel messaging, mutex locking, encoding

# Summary

Using `flamegraph` bottlenecks where verified and using `hyperfine` performance was verified.
Performance bottlenecks included unnecessary message passing, mutex locking, hex encoding, file I/O, and console I/O.

# Technical details

Within `package.rs` the `fs::read_to_string("data/packages.txt")` command was called multiple times during a single `run`.
By moving this function call outside a loop, it is only called once.

Within `pacakge.rs` the shared mutex was updated within a `for-loop`.
This caused the thread to block on aquiring locks.
This was solved by introducing a local variable to hold the checksum for that thread and update the shared checksum after the `for-loop` finishes.

Within `checksum.rs` there was unnececcary encoding and decoding from a string to a hex vector.
This was optimized by simply storing a `Vec<u8>` in the checksum class opposed to a string. The call to print the checksum is then encoded into a string.

Within `checksum.rs` there where two `Vec<u8>` allocated for the update (`a` and `b`).
Due to the previous optimization, this can now be reduced to just `b` and `a` is the current class (`self`).

Within `student.rs` there where calls to `writeln` that where not needed, therefore those console outputs where removed.

Within `student.rs` there was unnecessary message passing as student threads accepted ideas when they already had one and thus needed to send these ideas back into the queue.
This back and forth was removed by creating 3 message channels for each of the 3 types of events.
Students now only accept ideas when they currents do not have one, and when they do have one they wait for all their packages to finish downloading.
If there are no ideas in the channel and the idea generator signals it is done, the student will finish the hackathon.

# Testing for correctness

Before commencing optimization, the original checksum was recorded.
After any code change was made, a `diff` was used to compare the original checksum and the new checksum.
The script that performs this is present in `run-and-verify.sh`.

# Testing for performance.

Before commencing optimization, the started code was measured for performance using `hyperfine` and `flamegraph`.

Once the code has been measured, the flamegraph was analyzed to find bottlenecks in performance (wide blocks).
The bottlenecks are described above.

After these bottlenecks where observed, they where fixed and a new `hyperfine` and `flamegraph` was produced.

By comparing the original and new measurements, a judgment on the performance can be made.

The following results where collected on `ecetesla3`.

```bash
# Original Hyperfine
Benchmark #1: target/release/lab4
  Time (mean ± σ):     560.6 ms ± 119.5 ms    [User: 1.259 s, System: 0.091 s]
  Range (min … max):   277.4 ms … 1306.0 ms    1000 runs

# New Hyperfine
Benchmark #1: target/release/lab4
  Time (mean ± σ):      52.9 ms ±  18.4 ms    [User: 108.6 ms, System: 2.6 ms]
  Range (min … max):    20.2 ms … 135.5 ms    1000 runs
```

The following results where collected on `ecetesla2`.

```bash
# Original Hyperfine
Benchmark #1: target/release/lab4
  Time (mean ± σ):     988.1 ms ± 330.7 ms    [User: 812.8 ms, System: 85.9 ms]
  Range (min … max):   372.7 ms … 2138.6 ms    1000 runs

# New Hyperfine
Benchmark #1: target/release/lab4
  Time (mean ± σ):      39.5 ms ±  23.8 ms    [User: 74.9 ms, System: 4.1 ms]
  Range (min … max):    16.3 ms … 359.4 ms    1000 runs
```

The results shown below are collected from my local machine.

```bash
# Old Hyperfine
Benchmark #1: target/release/lab4
  Time (mean ± σ):     311.9 ms ±  67.3 ms    [User: 2.736 s, System: 0.067 s]
  Range (min … max):   189.6 ms … 477.7 ms    100 runs

# New Hyperfine
Benchmark #1: target/release/lab4
  Time (mean ± σ):      13.6 ms ±   0.4 ms    [User: 104.7 ms, System: 2.9 ms]
  Range (min … max):    10.8 ms …  17.1 ms    1000 runs
```

From the hyperfine results, there is a `10.6x` and `25.0x` speedup when tested on `ecetesla3` and `ecetesla2` respectively.
On my local machine, I observed a `22.9x` speedup.
It should be noted that the `eceteslaX` server results where obtained during high server load, and different times, so the results are not the most accurate.
