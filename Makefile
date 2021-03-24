TASKSET = taskset -c 2
BENCH_STR = --bench=bench-env
TARGET_MUSL = --target=x86_64-unknown-linux-musl

all:

bench-all: bench bench-musl

bench-build-all: bench-build bench-build-musl


bench: bench.1

bench-musl: bench.1-musl

bench-build:
	cargo bench --no-run

bench-build-musl:
	cargo bench --no-run $(TARGET_MUSL)

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

report:
	cargo xtask shape_benchmark_results


bench.1:
	@rm -f z.bench.1.log
	cargo bench --no-run
	$(TASKSET) cargo bench $(BENCH_STR) -- -n | tee -a z.bench.1.log

bench.1-musl:
	@rm -f z.musl.bench.1.log
	cargo bench --no-run $(TARGET_MUSL)
	$(TASKSET) cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.1.log
