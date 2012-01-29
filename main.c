#include <stdio.h>
#include <string.h>
#include <math.h>

#define RATE 11025
#define FRAC 5
#define BUF_LEN (RATE / FRAC)
#define LINE_LEN 72

long double gt = 0.0;
char line[LINE_LEN];
float buffer[BUF_LEN];

int main(int argc, char **argv) {
   while (fgets(line, LINE_LEN, stdin)) {
      if (line[0] == '#') {
         memset(line, 0, LINE_LEN);
         continue;
      }

#pragma omp parallel for schedule(static)
      for (int i = 0; i < BUF_LEN; ++i) {
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
            }
         }

         buffer[i] = s / 8;
      }

      fwrite(buffer, sizeof(float), BUF_LEN, stdout);
      memset(line, 0, LINE_LEN);
      gt += 2*M_PI/FRAC;
   }

   return 0;
}
