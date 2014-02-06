CC=rustc

%.o: %.cpp
	$(CC) -c -o $@ $< $(OFLAGS)

test: todotxt.rs
	$(CC) todotxt.rs --test -o todotxt-test
	./todotxt-test

clean:
	rm -f ./todotxt-test
	rm -f ./todotxt
