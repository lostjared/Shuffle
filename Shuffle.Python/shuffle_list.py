import sys
import random
lines = []
if(len(sys.argv) == 1):
	for i in sys.stdin:
		lines.append(i.rstrip())
else:
	for i in range(1, len(sys.argv)):
		file = open(sys.argv[i])
		for z in file:
			lines.append(z.rstrip())
random.shuffle(lines)
for i in lines:
	print(i)
