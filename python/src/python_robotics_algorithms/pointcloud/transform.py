import numpy as np
from scipy.spatial.transform import RigidTransform as Tf
from scipy.spatial.transform import Rotation as R
import timeit


def SetUp(numpoints):
    points = np.random.rand(numpoints, 3).astype(np.float32)
    t = np.random.rand(3).astype(np.float32)
    r = R.from_euler("XYZ", [90, 30, 0], degrees=True)
    transform = Tf.from_components(t, r)
    return points, transform

for num_points in [8000, 16000, 32000]:
    setup_code = f"points, transform = SetUp({num_points})" 
    run_code = "transform.apply(points)"

    print(f"Num points: {num_points}, time: {timeit.timeit(run_code, setup=setup_code, number=1000, globals=globals())} ms")
