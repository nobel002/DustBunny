import sys

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

# Usage: python Vis/lattice.py [path/to/Q_Latice.csv]
path = sys.argv[1] if len(sys.argv) > 1 else "Q_Latice.csv"
data = pd.read_csv(path, sep="\t")

t = data["t"].to_numpy()

# Only qx/qy are written as data columns; select them by name.
x_cols = sorted((c for c in data.columns if c.startswith("qx")), key=lambda c: int(c[2:]))
n = len(x_cols)
xs = data[x_cols].to_numpy()                                # shape (frames, n)
ys = data[[f"qy{i}" for i in range(1, n + 1)]].to_numpy()   # shape (frames, n)

fig, ax = plt.subplots(figsize=(8, 8))

ax.set_title("Trajectory in Configuration Space")
ax.set_xlabel(r"$q_x$")
ax.set_ylabel(r"$q_y$")
ax.set_aspect("equal")
ax.minorticks_on()
ax.grid(False)  # disabled for speed

# ----------------------------
# stable axis limits
# ----------------------------
xmin, xmax = np.min(xs), np.max(xs)
ymin, ymax = np.min(ys), np.max(ys)

xpad = 1e-3 if np.isclose(xmin, xmax) else 0.1 * (xmax - xmin)
ypad = 1e-3 if np.isclose(ymin, ymax) else 0.1 * (ymax - ymin)

ax.set_xlim(xmin - xpad, xmax + xpad)
ax.set_ylim(ymin - ypad, ymax + ypad)

# ----------------------------
# faint full paths + moving points
# ----------------------------
for k in range(n):
    ax.plot(xs[:, k], ys[:, k], "-", color=f"C{k % 10}", lw=0.8, alpha=0.3)

points = ax.scatter(xs[0], ys[0], c=[f"C{k % 10}" for k in range(n)], s=60, zorder=3)
time_text = ax.text(0.02, 0.98, "", transform=ax.transAxes, va="top")

# ----------------------------
# stride downsampling (speedup)
# ----------------------------
stride = 10  # increase if the animation is slow
frames = np.arange(0, len(t), stride)


def update(frame):
    i = frames[frame]
    points.set_offsets(np.column_stack([xs[i], ys[i]]))
    time_text.set_text(f"t = {t[i]:.3e}")
    return points, time_text


ani = FuncAnimation(fig, update, frames=len(frames), interval=10, blit=True)

plt.tight_layout()
plt.show()
