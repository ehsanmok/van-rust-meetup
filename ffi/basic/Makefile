ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

all: target/basic
	target/basic

target:
	mkdir -p $@

target/basic: target/main.o target/debug/libbasic.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libbasic.a: src/lib.rs Cargo.toml
	cargo build

target/main.o: src/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
