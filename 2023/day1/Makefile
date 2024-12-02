
all: trebuchet trebuchet2
	
trebuchet: trebuchet.c
	clang -o $@ $^

trebuchet2: trebuchet2.c
	clang -o $@ $^

test: trebuchet2
	./trebuchet2
