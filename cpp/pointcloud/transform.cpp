#include <vector>
#include <random>

#include <benchmark/benchmark.h>

#include <Eigen/Core>
#include <Eigen/Geometry>

#include <pcl/point_cloud.h>
#include <pcl/common/transforms.h>

static void BM_NaiveTransformation(benchmark::State& state) {
  std::vector<Eigen::Vector3f> src(state.range(0));
  std::vector<Eigen::Vector3f> dst(state.range(0));

  Eigen::Isometry3f T = Eigen::Isometry3f();
  
  for (auto _ : state) {
    for (size_t i = 0; i < src.size(); i++) {
        dst[i] = T * src[i];
    }
  }
}

// Register the function as a benchmark
BENCHMARK(BM_NaiveTransformation)->Arg(8000)->Arg(16000)->Arg(32000);

pcl::PointCloud<pcl::PointXYZ> RandomPCLCloud (const size_t num_points) {
  pcl::PointCloud<pcl::PointXYZ> out;

  std::random_device rd{};
  std::mt19937 gen{rd()};
  std::normal_distribution d{0.0f, 3.0f};

  for (size_t i = 0; i < num_points; i++) {
    pcl::PointXYZ p(d(gen), d(gen), d(gen));
    out.push_back(p);
  }

  return out;
}

static void BM_PCLTransformation(benchmark::State& state) {
  auto src = RandomPCLCloud(state.range(0));
  auto dst = RandomPCLCloud(state.range(0));

  Eigen::Isometry3f T = Eigen::Isometry3f();
  
  for (auto _ : state) {
    pcl::transformPointCloud (src, dst, T.matrix());
  }
}

// Register the function as a benchmark
BENCHMARK(BM_PCLTransformation)->Arg(8000)->Arg(16000)->Arg(32000);

BENCHMARK_MAIN();