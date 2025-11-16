use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{black_box, criterion_group, criterion_main};
extern crate nalgebra as na;
use na::{Point3, Isometry3, Translation3, UnitQuaternion, Vector3};
use rand::prelude::*;

fn gen_random_points(count: usize) -> Vec<Point3<f32>> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| Point3::new(rng.random::<f32>(), rng.random::<f32>(), rng.random::<f32>()))
        .collect()
}


fn transform_benchmark(c: &mut Criterion) {
    let tra = Translation3::new(0.0, 0.0, 3.0);
    let rot = UnitQuaternion::from_scaled_axis(Vector3::y() * std::f32::consts::PI);
    let iso = Isometry3::from_parts(tra, rot);

    let num_points : usize = 8000;
    let points: Vec<Point3<f32>> = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("transform {}", num_points), format!("{}",  points.len())), &(iso, points), |b, &(i, ref ps)| {
        let mut out: Vec<Point3<f32>> = Vec::with_capacity(num_points);
        b.iter(|| {
            out = ps.iter().map(|p| i * p).collect();
            black_box(&out);
        }
    );
    });

    let num_points : usize = 16000;
    let points: Vec<Point3<f32>> = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("transform {}", num_points), format!("{}",  points.len())), &(iso, points), |b, &(i, ref ps)| {
        let mut out: Vec<Point3<f32>> = Vec::with_capacity(num_points);
        b.iter(|| {
            out = ps.iter().map(|p| i * p).collect();
            black_box(&out);
        }
    );
    });

    let num_points : usize = 32000;
    let points: Vec<Point3<f32>> = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("transform {}", num_points), format!("{}",  points.len())), &(iso, points), |b, &(i, ref ps)| {
        let mut out: Vec<Point3<f32>> = Vec::with_capacity(num_points);
        b.iter(|| {
            out = ps.iter().map(|p| i * p).collect();
            black_box(&out);
        }
    );
    });
    
}

criterion_group!(benches, transform_benchmark);
criterion_main!(benches);