CFLAGS = -std=gnu99 -O3 -fopenmp

barrel-organ: main.o
	gcc -lm -fopenmp $(<) -o $(@)

.PHONY: play

play: barrel-organ
	./barrel-organ | aplay -r 11025 -f FLOAT_LE -
