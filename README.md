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
| 8000  |  4.2 us | 438.0 us | 10.1 us | 13.0 us | 361.7 us  | 154.7 us |
| 16000 |  8.9 us | 870.0 us | 21.3 us | 25.9 us | 727.9 us  | 499.8 us |
| 32000 | 17.6 us | 1766 us  | 40.4 us | 51.9 us | 1467.0 us | 957.9 us |