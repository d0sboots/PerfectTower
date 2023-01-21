TARG=dimensions
CC=g++
CFLAGS=-O3 -m64
CXXFLAGS=$(CFLAGS) -std=c++17
LDLIBS=-lm

all: $(TARG)

%.s: %.cc
	$(CC) -g -S $(CFLAGS) $< -o $@

$(TARG): $(TARG).cc

clean:
	rm $(TARG)
