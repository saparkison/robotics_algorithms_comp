from random import rand
from sys import num_physical_cores, simd_width_of

import benchmark
from algorithm import parallelize, vectorize
from complex import ComplexSIMD

alias float_type = DType.float32
alias simd_width = 2 * simd_width_of[float_type]()
alias unit = benchmark.Unit.ms

# Homogenos Vector in 3 dimensions (4th element is 1)
alias Vector3 = SIMD[float_type, 4]

alias min_x = -2.0
alias max_x = 0.6
alias min_y = -1.5
alias max_y = 1.5

@fieldwise_init
struct Transform3(ImplicitlyCopyable, Movable):
    var row_0: SIMD[float_type, 4]
    var row_1: SIMD[float_type, 4]
    var row_2: SIMD[float_type, 4]
    # Row 3 impliclity [0, 0, 0, 1]


@always_inline
fn matmul_1darray(A: Transform3, b: Vector3) -> Vector3:

    var out = Vector3 (
        (A.row_0 * b).reduce_add(),
        (A.row_1 * b).reduce_add(),
        (A.row_2 * b).reduce_add(),
        1.0
    )

    return out

def transform_points(num_points: UInt):
    print("Benchmark for ", num_points)
    var points = List[Vector3](length=num_points, fill=Vector3(1.0, 1.0, 1.0, 1.0))
    var out = List[Vector3](points)

    var T = Transform3(
        SIMD[float_type, 4](1.0, 0.0, 0.0, 5.0),
        SIMD[float_type, 4](0.0, 1.0, 0.0, 7.0),
        SIMD[float_type, 4](0.0, 0.0, 1.0, 6.0),
    )

    @always_inline
    @parameter
    fn benchmark_fn():
        for i in range(num_points):
            out[i] = matmul_1darray(T, points[i])

    var report = benchmark.run[benchmark_fn]()
    report.print(benchmark.Unit.ms)

def main():
    transform_points(8000)
    transform_points(16000)
    transform_points(32000)





