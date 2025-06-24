import json
import matplotlib.pyplot as plt
import numpy as np

# Įkeliam result.json
with open("result.json", "r") as f:
    data = json.load(f)

# Paverčiam į numpy masyvą
array = np.array(data)

# Nupiešiam
plt.imshow(array, cmap="Greys", interpolation="nearest")
plt.title("Ląstelių gyvybingumas (1 – gyva, 0 – mirusi)")
plt.colorbar(label="Gyvybingumas")
plt.show()
