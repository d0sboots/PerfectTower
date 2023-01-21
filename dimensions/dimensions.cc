#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <getopt.h>

#include <cmath>

// Utility that duplicates The Perfect Tower II's logic for generating
// dimensions, letting you find them more efficiently than clicking around.

// Random number implementation stolen from Java. Used for various operations,
// primarily to seed Unity's RNG.
struct JavaRNG {
  uint64_t seed;
  static_assert(sizeof(long) == sizeof(uint64_t));
  static constexpr uint64_t MASK = 0xffffffffffffL;
  static constexpr uint64_t PRIME = 0x5deece66dL;

  static JavaRNG from_coords(int x, int y) {
    JavaRNG rng;
    rng.set_seed(y * 0x40000000L + x);
    return rng;
  }
  void set_seed(uint64_t seed) {
    this->seed = (seed ^ PRIME) & MASK;
  }
  float next_float() {
    seed = (seed * PRIME + 0xb) & MASK;
    return (seed >> 24) * (1.0f / (1 << 24));
  }
  int int_range(int min, int max) {
    return std::floor(((double)max - min) * next_float() + min);
  }
  int float_range(float min, float max) {
    return (max - min) * next_float() + min;
  }
};

// Unity's internal RNG (https://docs.unity3d.com/ScriptReference/Random.html)
// Used for most of the actual random number generation.
struct UnityRNG {
  uint32_t state[4];

  void set_seed(int32_t seed) {
    constexpr int32_t PRIME = 0x6c078965;
    for (int i = 0; i < 4; ++i) {
      state[i] = seed;
      seed = seed * PRIME + 1;
    }
  }
  uint32_t next_uint() {
    uint32_t t = state[0];
    t ^= (t << 11);
    t ^= (t >> 8);
    state[0] = state[1];
    state[1] = state[2];
    uint32_t s = state[3];
    state[2] = s;
    state[3] = t ^ s ^ (s >> 19);
    return state[3];
  }
  float next_float() {
    return (next_uint() & 0x7fffff) * (1.0f / (1 << 23));
  }
  int int_range(int min, int max) {
    if (min < max) {
      return min + next_uint() % ((uint32_t)max - (uint32_t)min);
    }
    if (max < min) {
      return min - next_uint() % ((uint32_t)min - (uint32_t)max);
    }
    return min;
  }
};

static void generate_resource_name(char* out, UnityRNG* rng) {
  const static char VOWELS[] = "AEIOU";
  const static char CONSONANTS[] = "BCDFGHJKLMNPQRSTVWXYZ";

  const int pairs = rng->int_range(0, 3) + 2;
  for (int i = 0; i < pairs; ++i) {
    if (rng->next_float() < 0.1) {
      *out++ = VOWELS[rng->int_range(0, 5)];
      *out++ = VOWELS[rng->int_range(0, 5)];
    } else {
      if (rng->next_float() < 0.5) {
        *out++ = VOWELS[rng->int_range(0, 5)];
        *out++ = CONSONANTS[rng->int_range(0, 21)];
      } else {
        *out++ = CONSONANTS[rng->int_range(0, 21)];
        *out++ = VOWELS[rng->int_range(0, 5)];
      }
    }
  }
  if (rng->next_float() < 0.5) {
    if (memchr(VOWELS, out[-1], 5)) {
      *out++ = CONSONANTS[rng->int_range(0, 21)];
    } else {
      *out++ = VOWELS[rng->int_range(0, 5)];
    }
  }
  *out = '\0';
}

// This should return some sort of struct, but that's for later.
void generate_dimension(int xcoord, int ycoord) {
  char buf[10];
  JavaRNG rng = JavaRNG::from_coords(xcoord, ycoord);
  UnityRNG urng;
  urng.set_seed(rng.int_range(-0x80000000, 0x7fffffff));
  generate_resource_name(buf, &urng);
  printf("Dim %d %d: %s\n", xcoord, ycoord, buf);
}

// =================================================
// Non-reverse engineered code only below this point
// =================================================

int usage(const char* name) {
  fprintf(stderr, "Usage: %s xcoord ycoord\n\n", name);
  fprintf(stderr, "Options\n");
  fprintf(stderr, "  --help: Print this message and exit\n");
  return 1;
}

int parseint(const char* num, const char* name) {
  char* endptr;
  long res = strtol(num, &endptr, 0);
  if (*num == '\0' || *endptr != '\0') {
    fprintf(stderr, "Invalid number: '%s'\n", num);
    exit(usage(name));
  }
  if (res < -0x80000000L || res > 0x7fffffff) {
    fprintf(stderr, "Number out of range: '%s'\n", num);
    exit(usage(name));
  }
  return res;
}

int main(int argc, char* argv[]) {
  const option long_options[] = {
    {"help", no_argument, nullptr, 'h'},
    {0, 0, 0, 0},
  };
  while (true) {
    int opt_idx;
    int c = getopt_long(argc, argv, "h", long_options, &opt_idx);
    if (c == -1) break;

    if (c == 'h' || c == '?') {
      return usage(argv[0]);
    }
  }
  if (argc - optind != 2) {
    return usage(argv[0]);
  }
  int xcoord, ycoord;
  xcoord = parseint(argv[optind], argv[0]);
  ycoord = parseint(argv[optind + 1], argv[0]);
  generate_dimension(xcoord, ycoord);
  return 0;
}
