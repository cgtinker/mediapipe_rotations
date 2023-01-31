import mediapipe_rotations
import numpy as np


arr = np.linspace(0, 10, 1)
eye = mediapipe_rotations.eye(3)
print("eye", len(eye))
print("eye", eye)

res = mediapipe_rotations.max_min(np.array([[1.0, 2.0, 3.0], [11.0, 1.3, 0.0]]))
print("mm", res)
print("mm", len(arr), arr)
print("mm", res)

