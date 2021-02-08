cargo flamegraph

perf record --call-graph dwarf -- ./target/release/pruebas-rust
perf report -g graph --no-children         
