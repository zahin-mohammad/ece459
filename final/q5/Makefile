PERF_FLAGS := -F 1000 -g --call-graph dwarf

EXEC_FLAGS := --html examples/perf-rainbow.html --css examples/perf-rainbow.css

flamegraph:
	cargo flamegraph -c "record $(PERF_FLAGS)" -- $(EXEC_FLAGS)

hyperfine:
	hyperfine --warmup 1 'target/release/robinson $(EXEC_FLAGS)'
