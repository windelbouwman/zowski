
demo: demo.c scanner.c
	gcc -o demo demo.c scanner.c

scanner.c: tokens.txt
	cargo run -- tokens.txt scanner

clean:
	rm scanner.h
	rm scanner.c
	rm demo
