use super::utils;
use ndarray::Array2;

/// Check if there is a path from source to target with weight at least k.
///
/// # Arguments
/// * `matrix` - A square matrix representing the graph.
/// * `source` - The source node.
/// * `target` - The target node.
/// * `k` - The minimum weight of the path.
///
/// # Examples
/// ```
/// pub fn has_strong_path_dfs(matrix: &Array2<i32>, source: usize, target: usize, k: i32) -> bool {
///     let n = matrix.dim().0;
///     let mut visited = vec![false; n];
///
///     dfs(matrix, source, target, k, &mut visited)
/// }
/// ```
fn dfs(matrix: &Array2<i32>, node: usize, target: usize, k: i32, visited: &mut Vec<bool>) -> bool {
    if node == target {
        return true;
    }

    visited[node] = true;

    for (neighbor, &weight) in matrix.row(node).iter().enumerate() {
        if weight >= k && !visited[neighbor] && dfs(matrix, neighbor, target, k, visited) {
            return true;
        }
        println!("{} {}", neighbor, weight);
    }

    false
}

pub fn has_strong_path_dfs(matrix: &Array2<i32>, source: usize, target: usize, k: i32) -> bool {
    let n = matrix.dim().0;
    let mut visited = vec![false; n];

    dfs(matrix, source, target, k, &mut visited)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    use utils;

    #[test]
    fn test_has_strong_path_dfs() {
        let matrix = arr2(&[[0, 3, 0, 2], [0, 0, 5, 1], [0, 0, 0, 4], [0, 0, 0, 0]]);

        assert!(has_strong_path_dfs(&matrix, 0, 2, 3));
        assert!(has_strong_path_dfs(&matrix, 0, 3, 2));
        assert!(!has_strong_path_dfs(&matrix, 0, 3, 3));
        assert!(!has_strong_path_dfs(&matrix, 2, 0, 1));
    }
}
