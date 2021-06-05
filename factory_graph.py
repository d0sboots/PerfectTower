#!/usr/bin/python3

import math
import sys

import matplotlib
import matplotlib.pyplot as plt

ORE_POINTS = [      12.5,    164.157,     981.805,     5223.762,     26919.691,
              137812.499, 707792.640, 3662530.247, 19131554.997, 100964071.568]
ORE_PAIRS = [(x, 4**i) for i, x in enumerate(ORE_POINTS)]

matplotlib.use('svg')
fig, ax = plt.subplots(constrained_layout=True)
ax.set_xlabel("Shards")
ax.set_ylabel("Ore/Shard")
x_axis = []
data = []
label = 0
for i in range(4500):
    x = int(math.exp(i/200))
    if i and x == x_axis[-1]:
        # Don't duplicate points
        continue
    x_axis.append(x)
    data.append(sum((x//ore)*tier_value for ore, tier_value in ORE_PAIRS)/x)
    if label < len(ORE_PAIRS) and x >= ORE_PAIRS[label][0]:
        label += 1
        ax.annotate(f'T{label}', xy=(x, data[-1]), xytext=(-12, 3),
                    textcoords="offset points")
ax.semilogx(x_axis, data, label="T1-equivalent Ore", linewidth=1.1)
fig.savefig(sys.argv[1])
print(f"Maximum value: {data[-1]}")
