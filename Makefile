hello-world:
	@cargo run --release examples/hello-world.bf hello-world.c
	@gcc hello-world.c -o hello-world
	@./hello-world

fibonacci:
	@cargo run --release examples/fibonacci.bf fibonacci.c
	@gcc fibonacci.c -o fibonacci
	@./fibonacci

mandelbrot:
	@cargo run --release examples/mandelbrot.bf mandelbrot.c
	@gcc mandelbrot.c -o mandelbrot
	@./mandelbrot

clean:
	@rm hello-world.c hello-world fibonacci.c fibonacci mandelbrot.c mandelbrot
