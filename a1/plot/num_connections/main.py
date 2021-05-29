import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
plt.style.use('ggplot')

df = pd.read_csv("data2.csv")

x = df["command"]
means = df["mean"]
mins = df["min"]
maxes = df["max"]
plt.plot(x, means)
plt.errorbar(x, means, [means - mins, maxes - means],
             fmt='.k', ecolor='gray', lw=1)
plt.xlabel("Max Connections")
plt.ylabel("Time (s)")
plt.savefig(f'graph')
plt.title("Max Connections for Non-Blocking I/O vs Time")
# plt.show()
plt.figure()








df = pd.read_csv("data3.csv")

x = df["command"]
means = df["mean"]
plt.plot(x, means, label="Non-Blocking I/O")

blocking_500 = np.array([89.235 for i in range(len(x))])
plt.plot(x, blocking_500, label="Blocking I/O")

plt.xlabel("Max Connections")
plt.ylabel("Time (s)")
plt.legend()
plt.title("Max Connections for Non-Blocking I/O vs Time")
plt.savefig(f'graph3')