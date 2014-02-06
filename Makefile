CC=rustc

%.o: %.cpp
	$(CC) -c -o $@ $< $(OFLAGS)

test: todotxt.rs
	$(CC) todotxt.rs --test -o todotxt-test
	./todotxt-test

example: todo_example.rs
	$(CC) todo_example.rs 

clean:
	rm -f ./todotxt-test
	rm -f ./todotxt
