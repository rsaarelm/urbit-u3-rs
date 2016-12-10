U3RS_PATH=../target/debug/
CFLAGS=-Wall -I../include
LDFLAGS=-L$(U3RS_PATH) -lu3rs -Wl,-rpath $(U3RS_PATH)

test: test.c $(U3RS_PATH)/u3rs.so
	gcc -o $@ test.c $(CFLAGS) $(LDFLAGS)

.PHONY: $(U3RS_PATH)/u3rs.a
$(U3RS_PATH)/u3rs.so:
	cargo build --manifest-path ../Cargo.toml

clean:
	rm -f test
