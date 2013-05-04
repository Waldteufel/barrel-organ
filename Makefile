CFLAGS = -std=gnu99 -O3 -fopenmp

barrel-organ: main.o
	gcc -lm -fopenmp $(<) -o $(@)

.PHONY: play

play: barrel-organ
	./barrel-organ | paplay --rate=11025 --format=float32le --channels=1 --raw
