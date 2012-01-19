CFLAGS = -std=gnu99 -O3 -fopenmp

barrel-organ: main.o
	gcc -lm -fopenmp $(<) -o $(@)

.PHONY: play

play: barrel-organ
	./barrel-organ | play -r 11025 -b 32 -e float -t raw -
