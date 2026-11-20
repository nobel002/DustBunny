import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

data = pd.read_csv("Q_Latice.csv", sep="\t", header=None)
t = data[0].to_numpy()
xs = data[1:10].to_numpy()
ys = data[10:].to_numpy()


fig, ax = plt.subplots(figsize=(8, 8))

ax.set_title("Trajectory in Configuration Space")
ax.set_xlabel(r"$q_x$")
ax.set_ylabel(r"$q_y$")

ax.minorticks_on()
ax.grid(False)  # IMPORTANT: disable for speed

# ----------------------------
# stable axis limits
# ----------------------------)

xmin, xmax = np.min(xs), np.max(xs)
ymin, ymax = np.min(ys), np.max(ys)

xpad = 1e-3 if np.isclose(xmin, xmax) else 0.1 * (xmax - xmin)
ypad = 1e-3 if np.isclose(ymin, ymax) else 0.1 * (ymax - ymin)

ax.set_xlim(xmin - xpad, xmax + xpad)
ax.set_ylim(ymin - ypad, ymax + ypad)

# ----------------------------
# STRIDE DOWNSAMPLING (key speedup)
# ----------------------------
stride = 100  # increase if still slow (e.g. 500, 1000)

frames = np.arange(0, len(t), stride)

# ----------------------------
# scatter-like points (NO TRAILS)
# ----------------------------

ps = []
for set in range(len(xs)):
    ps.append(ax.plot([], [], ".", color="C0", markersize=8, label=set))

time_text = ax.text(0.02, 0.98, "", transform=ax.transAxes, va="top")

ax.legend()

# ----------------------------
# update function (constant time)
# ----------------------------
def update(i):
    i = frames[i]
    for idx, p in enumerate(ps):
        p[0].set_data([xs[idx]], [ys[idx]])  # p[0] not p[i], idx not i

    time_text.set_text(f"t = {t[i]:.3e}")

    return [p[0] for p in ps] + [time_text]  # flat list of artists
    return ps, time_text

ani = FuncAnimation(
    fig,
    update,
    frames=len(frames),
    interval=10,
    blit=True
)

plt.tight_layout()
plt.show()