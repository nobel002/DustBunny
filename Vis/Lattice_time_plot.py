import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

data = pd.read_csv("Q_Latice.csv", sep="\t", header=None)

t = data[0]

len(data) - 1

xs =
fig, ax = plt.subplots(figsize=(8, 8))

ax.set_title("Trajectory in Configuration Space")
ax.set_xlabel(r"$q_x$")
ax.set_ylabel(r"$q_y$")

ax.minorticks_on()
ax.grid(False)  # IMPORTANT: disable for speed

# ----------------------------
# stable axis limits
# ----------------------------
x = np.concatenate([qx1, qx2, qx3])
y = np.concatenate([qy1, qy2, qy3])

xmin, xmax = np.min(x), np.max(x)
ymin, ymax = np.min(y), np.max(y)

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
p1, = ax.plot([], [], ".", color="C0", markersize=8, label="C1")
p2, = ax.plot([], [], ".", color="C1", markersize=8, label="C2")
p3, = ax.plot([], [], ".", color="C2", markersize=8, label="C3")

time_text = ax.text(0.02, 0.98, "", transform=ax.transAxes, va="top")

ax.legend()

# ----------------------------
# update function (constant time)
# ----------------------------
def update(i):
    i = frames[i]

    p1.set_data([qx1[i]], [qy1[i]])
    p2.set_data([qx2[i]], [qy2[i]])
    p3.set_data([qx3[i]], [qy3[i]])

    time_text.set_text(f"t = {t[i]:.3e}")

    return p1, p2, p3, time_text

ani = FuncAnimation(
    fig,
    update,
    frames=len(frames),
    interval=10,
    blit=True
)

plt.tight_layout()
plt.show()