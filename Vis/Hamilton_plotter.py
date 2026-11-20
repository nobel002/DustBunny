import numpy as np
import pandas as pd
from matplotlib import pyplot as plt

data_symplectic = pd.read_csv("Energy_Symplectic.csv", sep="\t")
data_forward = pd.read_csv("Energy.csv", sep="\t")

t, energy = data_symplectic["t"], data_symplectic["energy"]
t_forward, energy_forward = data_forward["t"], data_forward["energy"]

plt.title("Energy Evolution")

plt.plot(t, energy, label="Energy, Symplectic")
plt.plot(t, energy_forward, label="Energy, Forward")

plt.ylabel("Energy")
plt.ylim([min(energy), max(energy)])

plt.legend()

plt.xlabel("Time")
plt.show()