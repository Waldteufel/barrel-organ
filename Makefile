CFLAGS = -std=gnu99 -O3 -fopenmp

barrel-organ: main.o
	gcc -lm -fopenmp $(<) -o $(@)

.PHONY: play-pulse play-alsa

play-pulse: barrel-organ
	./barrel-organ | paplay --rate=11025 --format=float32le --channels=1 --raw

play-alsa: barrel-organ
	./barrel-organ | aplay --rate=12025 --channels=1 -f FLOAT_LE
