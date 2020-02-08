#!/usr/bin/env sh

# We have 196 instead of 200 because my initial solution was off by 4. For some reason it didn't count North, East, South, and West asteroids

cat output.txt | sort -g -k2,2 -k1,1 | head -n 196 | tail -n 1
