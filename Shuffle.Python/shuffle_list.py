import sys
import random
lines = []
for i in sys.stdin:
	lines.append(i.rstrip())
random.shuffle(lines)
for i in lines:
	print(i)
