use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{black_box, criterion_group, criterion_main};
use nalgebra::{Point3};
use parry3d::query;
use rand::prelude::*;
use sif_kdtree::{KdTree, Object, WithinDistance};

mod kd_tree;

fn gen_random_points(count: usize) -> Vec<Point3<f32>> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| Point3::new(rng.random::<f32>(), rng.random::<f32>(), rng.random::<f32>()))
        .collect()
}

#[derive(Clone, Copy)]
struct Something([f32; 3]);

impl Object for Something {
    type Point = [f32; 3];

    fn position(&self) -> &Self::Point {
        &self.0
    }
}

fn gen_random_somethings(count: usize) -> Vec<Something> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| Something([rng.random::<f32>(), rng.random::<f32>(), rng.random::<f32>()]))
        .collect()
}

fn gen_random_sif_points(count: usize) -> Vec<[f32; 3]> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| [rng.random::<f32>(), rng.random::<f32>(), rng.random::<f32>()])
        .collect()
}

fn kd_tree_benchmark(c: &mut Criterion) {

    let num_points : usize = 8000;
    let points: Vec<Point3<f32>> = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("KD-tree sort {}", num_points), format!("{}",  points.len())), &(points), |b, &ref ps| {
        b.iter(|| {
            let kd_tree = kd_tree::KDTree::sort(ps);
            black_box(&kd_tree);
            }
        );
    });

    c.bench_with_input(BenchmarkId::new(format!("SIF-KD-tree sort {}", num_points), format!("{}",  points.len())), &(points), |b, &ref ps| {
        let somethings = gen_random_somethings(num_points);
        b.iter(|| {
            let kd_tree = KdTree::new(somethings.clone());
            black_box(&kd_tree);
            }
        );
    });

    let num_points : usize = 16000;
    let points: Vec<Point3<f32>> = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("KD-tree sort {}", num_points), format!("{}",  points.len())), &(points), |b, &ref ps| {
        b.iter(|| {
            let kd_tree = kd_tree::KDTree::sort(ps);
            black_box(&kd_tree);
            }
        );
   });

    let num_points : usize = 32000;
    let points: Vec<Point3<f32>> = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("KD-tree sort {}", num_points), format!("{}",  points.len())), &(points), |b, &ref ps| {
        b.iter(|| {
            let kd_tree = kd_tree::KDTree::sort(ps);
            black_box(&kd_tree);
            }
        );
    });


    let num_points : usize = 8000;
    let points : Vec<Point3<f32>> = gen_random_points(num_points);
    let kd_tree = kd_tree::KDTree::sort(&points);
    let queries = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("KD-tree query {}", num_points), format!("{}",  points.len())), &(points), |b, &ref ps| {
        b.iter(|| {
            queries.iter().for_each( |q|{
                let index = kd_tree.query(q);
                black_box(&index);
            });
            }
        );
    });

    c.bench_with_input(BenchmarkId::new(format!("SIF-KD-tree query {}", num_points), format!("{}",  points.len())), &(points), |b, &ref ps| {
        let somethings = gen_random_somethings(num_points);
        let kd_tree = KdTree::new(somethings.clone());
        let query_somethings = gen_random_sif_points(num_points);
        b.iter(|| {
            query_somethings.iter().for_each(|q| {
                let found = kd_tree.nearest(q);
                black_box(found);
            });
            }
        );
    });

    let num_points : usize = 16000;
    let points : Vec<Point3<f32>> = gen_random_points(num_points);
    let kd_tree = kd_tree::KDTree::sort(&points);
    let queries = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("KD-tree query {}", num_points), format!("{}",  points.len())), &(points), |b, &ref ps| {
        b.iter(|| {
            queries.iter().for_each( |q|{
                let index = kd_tree.query(q);
                black_box(&index);
            });
            }
        );
    });

    let num_points : usize = 32000;
    let points : Vec<Point3<f32>> = gen_random_points(num_points);
    let kd_tree = kd_tree::KDTree::sort(&points);
    let queries = gen_random_points(num_points);
    c.bench_with_input(BenchmarkId::new(format!("KD-tree query {}", num_points), format!("{}",  points.len())), &(points), |b, &ref ps| {
        b.iter(|| {
            queries.iter().for_each( |q|{
                let index = kd_tree.query(q);
                black_box(&index);
            });
            }
        );
    });
}

criterion_group!(benches, kd_tree_benchmark);
criterion_main!(benches);