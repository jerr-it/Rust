/// Intersection over Union, also known as the Jaccard similarity coefficient, is a measure of the similarity between two sets.
/// Its defined as the size of the intersection divided by the size of the union of two sets.
/// Its commonly used in computer vision tasks to evaluate the performance of object detection algorithms.
///
/// Wikipedia: <https://en.wikipedia.org/wiki/Jaccard_index>
use std::collections::HashSet;

pub fn intersection_over_union<T: Eq + std::hash::Hash>(set1: &HashSet<T>, set2: &HashSet<T>) -> f64 {
    let intersection = set1.intersection(set2).count() as f64;
    let union = set1.union(set2).count() as f64;
    intersection / union
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_over_union() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
        assert_eq!(intersection_over_union(&set1, &set2), 2.0 / 4.0);

        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [4, 5, 6].iter().cloned().collect();
        assert_eq!(intersection_over_union(&set1, &set2), 0.0);
    }
}