import json

grid = [
    [0.5, 0.1, 0.05],
    [0.3, 0.25, 0.15],
    [0.4, 0.35, 0.05]
]

with open("voxel_data.json", "w") as f:
    json.dump(grid, f)