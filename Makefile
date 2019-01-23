
all:
	@cargo build --release

build-dev:
	@cargo build

test:
	@cargo test

dev: build-dev test

clean:
	@cargo clean

check:
	@cargo check

watch:
	while find src/ -print0 | xargs -0 inotifywait -e delete_self -e modify ; do \
		echo "============ at `date` ==========" ; \
		make check ; \
	done
