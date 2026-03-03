use core::f32;
use std::cmp::Ordering;
use std::ops::Index;

use nalgebra::Point3;
use parry3d::bounding_volume::Aabb;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
enum SplitAxis {
    X = 0,
    Y = 1,
    Z = 2
}

impl SplitAxis {
    fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(SplitAxis::X),
            1 => Some(SplitAxis::Y),
            2 => Some(SplitAxis::Z),
            _ => None,
        }
    }
}

impl Index<&SplitAxis> for Point3<f32> {
    type Output = f32;

    fn index(&self, split: &SplitAxis) -> &Self::Output {
        match split {
            SplitAxis::X => &self.coords[0],
            SplitAxis::Y => &self.coords[1],
            SplitAxis::Z => &self.coords[2],
        }

    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct SplitInfo {
    axes : SplitAxis,
    left_child : Option<usize>,
    right_child : Option<usize>,
}

pub struct KDTree {
    points : Vec<Point3<f32>>,
    info : Vec<SplitInfo>,
    entry_point : usize
}



impl KDTree {
    pub fn sort(points: &[Point3<f32>]) -> KDTree {
        let mid: usize = points.len() / 2;
        let mut tree = KDTree{
            points :points.to_vec(),
            info : vec![
                SplitInfo{
                    axes : SplitAxis::X,
                    left_child : Some(0),
                    right_child : Some(0),
                };
                points.len()
            ],
            entry_point : mid,
        };

        struct RegionToSort{
            position : usize,
            left_bound : usize,
            right_bound : usize,
            axes : usize,
        }

        let mid: usize = points.len() / 2;

        let start: RegionToSort = RegionToSort{
            position : mid,
            left_bound : 0,
            right_bound: points.len() - 1,
            axes : 0,
        };
        let mut queue : Vec::<RegionToSort> = Vec::new();
        queue.push(start);

        while !queue.is_empty() {
            let current_region : RegionToSort = match queue.pop() {
                Some(top) => top,
                None => RegionToSort{
                    position : 0,
                    left_bound : 0,
                    right_bound : 0,
                    axes : 0,
                },
            };

            let current_slice : &mut [Point3<f32>] = &mut tree.points[current_region.left_bound..current_region.right_bound];

            //let current_aabb = Aabb::from_points_ref(&*current_slice);
            //let largest_axis_index = current_aabb.extents().argmax().0;
            let largest_axis_index = current_region.axes;

            let comp : Box<dyn Fn(&Point3<f32>, &Point3<f32>) -> Ordering> = {
                match largest_axis_index {
                    0 => Box::new(|a, b| { 
                            if a.x < b.x {
                                return Ordering::Less
                            } else if a.x > b.x {
                                return Ordering::Greater
                            }
                            Ordering::Equal
                        }),
                    1 => Box::new(|a, b| {
                            if a.y < b.y {
                                return Ordering::Less
                            } else if a.y > b.y {
                                return Ordering::Greater
                            }
                            Ordering::Equal
                        }),
                    _ => Box::new(|a, b| {
                            if a.z < b.z {
                                return Ordering::Less
                            } else if a.z > b.z {
                                return Ordering::Greater
                            }
                            Ordering::Equal
                        }),
                }
            };
            
            let mid = current_region.position - current_region.left_bound;
            current_slice.select_nth_unstable_by(mid, comp);
            let left_child_right_bound = if current_region.position > 0usize {
                current_region.position - 1
            } else {
                0
            };
            let left_child_left_bound = current_region.left_bound;
            let left_child_position = match left_child_left_bound >= left_child_right_bound {
                true => None,
                false => Some((left_child_right_bound + left_child_left_bound) / 2)
            };
            if let Some(position) = left_child_position {
                queue.push(RegionToSort{
                    position : position,
                    left_bound : left_child_left_bound,
                    right_bound : left_child_right_bound,
                    axes : (current_region.axes + 1) % 3
                });
            }

            let right_child_right_bound = current_region.right_bound;
            let right_child_left_bound = if current_region.position < points.len() - 2 {
                current_region.position + 1
            } else {
                points.len() - 1
            };
            let right_child_position = match right_child_left_bound >= right_child_right_bound {
                true => None,
                false => Some((right_child_right_bound + right_child_left_bound) / 2)
            };
            if let Some(position) = right_child_position {
                queue.push(RegionToSort{
                    position : position,
                    left_bound : right_child_left_bound,
                    right_bound : right_child_right_bound,
                    axes : (current_region.axes + 1) % 3
                });
            }

            let info = tree.info.get_mut(current_region.position).unwrap();
            info.axes = SplitAxis::from_usize(largest_axis_index).unwrap();
            info.left_child = left_child_position;
            info.right_child = right_child_position;
        }

        tree
    }

    pub fn query(&self, query: &Point3<f32>) -> usize {

        struct BestMatch {
            distance : f32,
            index : usize,
        }
        let mut best = BestMatch{
            distance : f32::INFINITY,
            index : self.points.len() + 1,
        };

        let log_2_size  = self.points.len().ilog2();
        let mut p_queue : Vec<usize> = Vec::with_capacity(log_2_size.try_into().unwrap());
        p_queue.push(self.entry_point);

        while !p_queue.is_empty() {
            let current_index = p_queue.pop().unwrap();
            let current_point = &self.points[current_index];
            let current_axis = &self.info[current_index].axes;
            let distance = (current_point - query).norm();

            if distance < best.distance {
                best.distance = distance;
                best.index = current_index;
            }

            let offset_to_axis = current_point[current_axis] - query[current_axis];
            let query_is_greater = offset_to_axis.is_sign_positive();

            if query_is_greater {
                if let Some(child) = self.info[current_index].right_child {
                    p_queue.push(child);
                }
                if offset_to_axis.abs() < best.distance {
                    if let Some(child) = self.info[current_index].left_child {
                        p_queue.push(child)
                    }
                }
            } else {
                if let Some(child) = self.info[current_index].left_child {
                    p_queue.push(child);
                }
                if offset_to_axis.abs() < best.distance {
                    if let Some(child) = self.info[current_index].right_child {
                        p_queue.push(child)
                    }
                }
            }
        }
        best.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point3;
    use parry3d::query;

    #[test]
    fn sort() {
        let points : Vec<Point3<f32>>= vec![
            Point3::new(1.0f32, 2.0, 3.0),
            Point3::new(2.0, 3.0, 1.0),
            Point3::new(0.0, 1.0, 5.0),
            Point3::new(0.0, 1.0, 5.0),
            Point3::new(0.0, 1.0, 5.3),
            Point3::new(0.0, 1.1, 5.0),
        ];

        let kd_tree = KDTree::sort(&points);
        assert_eq!(kd_tree.points.len(), points.len());
        assert_eq!(kd_tree.info.len(), points.len());
    }

    #[test]
    fn query() {
        let points : Vec<Point3<f32>>= vec![
            Point3::new(1.0f32, 2.0, 3.0),
            Point3::new(2.0, 3.0, 1.0),
            Point3::new(0.0, 1.0, 5.0),
            Point3::new(0.0, 1.0, 5.0),
            Point3::new(0.0, 1.0, 5.3),
            Point3::new(0.0, 1.1, 5.0),
        ];

        let kd_tree = KDTree::sort(&points);       
        let query = Point3::new(2.1f32, 3.0, 1.0);
        let true_closest = Point3::new(2.0f32, 3.0, 1.0);

        let closest_index = kd_tree.query(&query);
        assert_eq!(kd_tree.points[closest_index], true_closest);
    }
}