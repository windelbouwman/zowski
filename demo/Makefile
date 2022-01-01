
all: demo pascal

demo: demo.c scanner.c scanner.h
	gcc -o demo demo.c scanner.c

scanner.c: tokens.txt
	cargo run -- tokens.txt scanner

clean:
	rm -f scanner.c scanner.h
	rm -f pascal_scanner.c pascal_scanner.h
	rm demo pascal


pascal: pascal.c pascal_scanner.c
	gcc -o pascal pascal.c pascal_scanner.c

pascal_scanner.c: ../examples/pascal.zowski
	cargo run -- ../examples/pascal.zowski pascal_scanner
