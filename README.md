# Robotics Algorithm Comp

This repository is a sandbox that I am using to compare the implementation of standard robotics algorithms in various languages. I only consider myself proficient in C++ and Python, so the implementation I use in other languages shouldn't be taken as a reference.

## Point Cloud Algorithms

### Transform

Simple isometric transformation of a point cloud.

```python
points : List[Point3]
T      : Isometry3
out = [T*p for p in points]
```

|       | C++ - Naive | C++ - PCL | Rust | Mojo | Julia | Python - Numpy/Scipy |
| ---   | ---         | ----      | ---- | ---  | ---   | ---                  |
| 8000  |  4.2 us | 6.4 us | 10.1 us | 13.0 us | 361.7 us  | 154.7 us |
| 16000 |  8.9 us | 12.7 us | 21.3 us | 25.9 us | 727.9 us  | 499.8 us |
| 32000 | 17.6 us | 25.4 us  | 40.4 us | 51.9 us | 1467.0 us | 957.9 us |

### KD Tree Build

|       |  C++ - PCL | Rust |
| ---   | ---        | ---- |
| 8000  |  0.708 ms  | 0.535 ms |
| 16000 |  1.619 ms  | 1.170 ms |
| 32000 |  3.580 ms  | 2.435 ms  |

### KD Tree Query

|       |  C++ - PCL | Rust |
| ---   | ---        | ---- |
| 8000  |  5.508 ms  | 7.096 ms |
| 16000 |  11.898 ms  | 18.128 ms |
| 32000 |  23.390 ms  | 46.650 ms  |


