import numpy as np
import matplotlib.pyplot as plt
import pandas as pd

data = pd.read_csv("Q_symplectic.csv", sep="\t")
t = data["t"]
qx1, qy1 = data["qx1"], data["qy1"]
qx2, qy2 = data["qx2"], data["qy2"]
qx3, qy3 = data["qx3"], data["qy3"]

fig, ax = plt.subplots(figsize=(8, 8))

mask = t>0.0

ax.plot(qx1[mask], qy1[mask], ".", markersize=1, color="C0",label="C1")
ax.plot(qx2[mask], qy2[mask], ".", markersize=1, color="C1", label="C2")
ax.plot(qx3[mask], qy3[mask], ".", markersize=1, color="C2", label="C3")
ax.set_title("Trajectory in Configuration Space")
ax.set_xlabel(r"$q_x$")
ax.set_ylabel(r"$q_y$")

ax.legend()

ax.minorticks_on()
ax.grid(which="major", linestyle="-", alpha=0.4)
ax.grid(which="minor", linestyle=":", alpha=0.2)

plt.tight_layout()
plt.show()