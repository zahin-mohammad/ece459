import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec

plt.style.use('ggplot')

df = pd.read_csv("data.csv")
print(df.columns)
grouped = df.groupby(["Method"])

val = grouped.get_group("Non-Blocking")
x1 = val["Number of Puzzle’s"]
means1 = val["Time (s)"]
mins1 = val["Min (s)"]
maxes1 = val["Max (S)"]
std1 = val["Standard Deviation"]

val = grouped.get_group("Blocking")
x2 = val["Number of Puzzle’s"]
means2 = val["Time (s)"]
mins2 = val["Min (s)"]
maxes2 = val["Max (S)"]
std2 = val["Standard Deviation"]

fig = plt.figure(constrained_layout=True)

gs = fig.add_gridspec(2, 2)

main = fig.add_subplot(gs[0, :])
main.set_title("Blocking vs Non-Blocking IO")
main.plot(x1,means1, label="Non-Blocking")
main.plot(x2,means2, label="Non-Blocking")
main.errorbar(x1, means1, [means1 - mins1, maxes1 - means1],
             fmt='.k', ecolor='gray', lw=1)
main.errorbar(x2, means2, [means2 - mins2, maxes2 - means2],
             fmt='.k', ecolor='gray', lw=1)
main.set_ylabel('Time in S')
main.set_xlabel('Number of Puzzles')

# axes.color_cycle    : 348ABD, 7A68A6, A60628, 467821, CF4457, 188487, E24A33
#                       # E24A33 : orange
#                       # 7A68A6 : purple
#                       # 348ABD : blue
#                       # 188487 : turquoise
#                       # A60628 : red
#                       # CF4457 : pink
#                       # 467821 : green

left = fig.add_subplot(gs[1, :1])
left.set_title("Non-Blocking IO")
left.plot(x1,means1)

right = fig.add_subplot(gs[1, 1:2])
right.set_title("Blocking IO")
right.plot(x2,means2, color='#348ABD')

# plt.show()
plt.savefig(f'graph')

