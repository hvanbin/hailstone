#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "metadata.h"

extern short const vers_major;
extern short const vers_minor;
extern short const vers_patch;

static const char *msg_errors[] = {"Option needs a value.\n",
                                   "Option is unknown.\n"};

// Function hail computes the hailstone sequence.
// Param: a number in the hailstone sequence
// Return: the next number in the sequence
static short hail(short);

// Function unhail computes the hailstone sequence in reverse.
// Param: a number in the hailstone sequence
// Return: the previous number in the sequence
static short unhail(short);

// Procedure put_hsval puts a number of '*' characters to standard out.
// Param: the number of '*' characters
static void put_hsval(short);

// Function usage prints usage.
static int usage(char const *const progname, char const *const options);

// Function run is a subroutine which uses the arguments to perform the calls to
// hail and unhail.
static int run(bool const, short, short, bool const);

// Function main uses an option string to gather verbosity, start values, etc.
// By default, a sequence is started at 1 and runs for 1 computation.
int main(int argc, char *argv[]) {
  int optret = -1;
  const char *optstr = ":hln:s:vV";
  short optstart = 1;
  short optcounter = 1;
  bool optloop = false;
  bool optverbose = false;

  while ((optret = getopt(argc, argv, optstr)) != -1) {
    switch (optret) {
    case 'V':
      printf("version:\t%d.%d.%d\n", vers_major, vers_minor, vers_patch);
      exit(EXIT_SUCCESS);
    case 'h':
      exit(usage(argv[0], optstr));
    case 'n':
      optcounter = atoi(optarg);
      break;
    case 'l':
      optloop = true;
      break;
    case 's':
      optstart = atoi(optarg);
      break;
    case 'v':
      optverbose = true;
      break;
    case ':':
      fprintf(stderr, "%s: issue(%d)\t%s", argv[0], 0, msg_errors[0]);
      usage(argv[0], optstr);
      exit(EXIT_FAILURE);
    case '?':
      fprintf(stderr, "%s: issue(%d) with %c\t%s", argv[0], 1, optopt,
              msg_errors[1]);
      usage(argv[0], optstr);
      exit(EXIT_FAILURE);
    }
  }
  exit(run(optverbose, optstart, optcounter, optloop));
}

static int usage(char const *const progname, char const *const options) {
  char const *description = "Print hailstone sequence values.\n";
  printf("usage:\t%s [ %s ]\ndescription: %s", progname, options, description);
}
static int run(bool const in_verbose, short const in_start,
               short const in_counter, bool const in_loop) {
  short hsval = in_start;
  printf("counter: %d\n", in_counter);
  do {
    for (int i = abs(in_counter); i > 0; --i) {
      hsval = (in_counter > 0) ? hail(hsval) : unhail(hsval);
      printf("%02d", hsval);
      if (in_verbose) {
        printf("\t");
        put_hsval(hsval);
      }
      printf("\n");
    }
  } while (in_loop);
  return EXIT_SUCCESS;
}

static short hail(short in_hsval) {
  return (in_hsval & 1) ? in_hsval * 3 + 1 : in_hsval / 2;
}
static short unhail(short in_hsval) {
  return (in_hsval & 1) ? in_hsval * 2 : (in_hsval - 1) / 3;
}

static void put_hsval(short in_val) {
  for (int j = 0; j < in_val; ++j) {
    putc('*', stdout);
  }
}
