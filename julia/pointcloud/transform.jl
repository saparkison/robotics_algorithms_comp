using LinearAlgebra
using CoordinateTransformations
using Random
using BenchmarkTools

const SIZES = (8000, 16000, 32000)


R = Float32[1.0 0.0 0.0; 0.0 1.0 0.0; 0.0 0.0 1.0]
t = Float32[2.0, 2.0, 4.0]
A = AffineMap(R, t)

for s in SIZES
    points = []
    for _ in 1:s
        p = rand(Float32, 3)
        push!(points, p)
    end

    b = @benchmark [$A(p) for p in $points]

    println(b)

end