#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define RATE 11025
#define LINE_LEN 72

long double gt = 0.0;
float frac = 5.0;

char line[LINE_LEN];

int main(int argc, char **argv) {
   srand(time(NULL));
   while (fgets(line, LINE_LEN, stdin)) {
      if (line[0] == '#') {
         memset(line, 0, LINE_LEN);
         continue;
      }
      if (line[0] == '!') {
         frac = atof(line + 1);
         memset(line, 0, LINE_LEN);
         continue;
      }
      int buf_len = RATE / frac;
      float *buffer = malloc(sizeof(float) * buf_len);

#pragma omp parallel for schedule(static)
      for (int i = 0; i < buf_len; ++i) {
         const float t = gt + i*2*M_PI/RATE;

         float s = 0;
         for (int j = 0; j < LINE_LEN; ++j) {
            const float freq = 110.0 * pow(2.0, j / 12.0);

            switch (line[j]) {
               case '$': // sine
                  s += 2 * sin(freq * t);
                  break;

               case '*': // triangle
                  s += fabs(2 - fmod(t * freq/M_PI_2, 4.0)) - 1;
                  break;

               case '+': // sawtooth
                  s += (fmod(t * freq/M_PI, 2.0) - 1) / 4;
                  break;

               case '%': // square
                  s += copysign(1.0, sin(freq * t)) / 4;
                  break;

               case '/': // noise
                  s += (50.0 / freq) * (fmod(rand(), 2.0));
                  break;
            }
         }

         buffer[i] = s / 8;
      }

      fwrite(buffer, sizeof(float), buf_len, stdout);
      memset(line, 0, LINE_LEN);
      gt += 2*M_PI/frac;
      free(buffer);
   }

   return 0;
}
