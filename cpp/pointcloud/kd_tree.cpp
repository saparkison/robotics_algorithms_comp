#include <vector>
#include <random>
#include <ranges>

#include <benchmark/benchmark.h>

#include <Eigen/Core>
#include <Eigen/Geometry>

#include <pcl/point_cloud.h>
#include <pcl/common/transforms.h>
#include <pcl/kdtree/kdtree_flann.h>

std::vector<Eigen::Vector3f> RandomStdVector(const size_t num_points) {
  std::random_device rd{};
  std::mt19937 gen{rd()};
  std::normal_distribution d{0.0f, 3.0f};

  std::vector<Eigen::Vector3f> out;
  for (size_t i = 0; i < num_points; i++) {
    out.emplace_back(d(gen), d(gen), d(gen));
  }
  return out;
}

pcl::PointCloud<pcl::PointXYZ> RandomPCLCloud (const size_t num_points) {
  pcl::PointCloud<pcl::PointXYZ> out;
  out.width = num_points;
  out.height = 1;
  out.is_dense = true;

  std::random_device rd{};
  std::mt19937 gen{rd()};
  std::normal_distribution d{0.0f, 3.0f};

  for (size_t i = 0; i < num_points; i++) {
    pcl::PointXYZ p(d(gen), d(gen), d(gen));
    out.push_back(p);
  }

  return out;
}

static void BM_PCLKDTree(benchmark::State& state) {
  auto src = std::make_shared<pcl::PointCloud<pcl::PointXYZ>>(RandomPCLCloud(state.range(0)));

  for (auto _ : state) {
    pcl::KdTreeFLANN<pcl::PointXYZ> kdtree;
    kdtree.setInputCloud(src);
  }
}

// Register the function as a benchmark
BENCHMARK(BM_PCLKDTree)->Arg(8000)->Arg(16000)->Arg(32000);

static void BM_PCLKDTree_Query(benchmark::State& state) {
  auto src = std::make_shared<pcl::PointCloud<pcl::PointXYZ>>(RandomPCLCloud(state.range(0)));
  pcl::KdTreeFLANN<pcl::PointXYZ> kdtree;
  kdtree.setInputCloud(src);

  auto query = RandomPCLCloud(state.range(0));
  const int k_index = 1;
  std::vector<int> nn_index;
  std::vector<float> nn_distance;
  for (auto _ : state) {
    for (int i = 0; i < query.size(); i++) {
      kdtree.nearestKSearch(query, i,  k_index, nn_index, nn_distance);
    }
  }
}

// Register the function as a benchmark
BENCHMARK(BM_PCLKDTree_Query)->Arg(8000)->Arg(16000)->Arg(32000);

BENCHMARK_MAIN();