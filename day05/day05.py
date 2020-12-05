#!/usr/bin/env python3

# table to convert to binary string
tbl = "FBLR".maketrans({'F': '0', 'B': '1', 'L': '0', 'R': '1'})

def pass2id(passport): 
	return int(passport.translate(tbl), 2)

max = -1
min = -1
numseats = 2**11
seats = [0] * numseats # map of potential 2048 seats
f = open("testdata/input.txt", "r")
for line in f:
	id = pass2id(line.strip())
	seats[id] = 1
	if id > max:
		max = id
	if min == -1 or id < min:
		min = id
print("part 1:", max)
for i in range(min, max+1):
	if seats[i] == 0:
		print("part 2: ", i)
