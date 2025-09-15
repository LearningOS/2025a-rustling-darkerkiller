/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    /// DFS辅助函数：递归实现深度优先搜索
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 标记当前节点为已访问
        visited.insert(v);
        // 记录访问顺序
        visit_order.push(v);
        
        // 递归访问所有未被访问的邻接节点
        // 注意：为了保证测试顺序一致，我们对邻接节点进行排序
        let mut neighbors = self.adj[v].clone();
        neighbors.sort_unstable();
        
        for &neighbor in &neighbors {
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    /// 执行深度优先搜索，返回访问节点的顺序
    fn dfs(&self, start: usize) -> Vec<usize> {
        // 检查起始节点是否有效
        if start >= self.adj.len() {
            return Vec::new();
        }
        
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }

    #[test]
    fn test_dfs_single_node() {
        let graph = Graph::new(1);
        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0]);
    }

    #[test]
    fn test_dfs_invalid_start() {
        let graph = Graph::new(3);
        let visit_order = graph.dfs(5); // 起始节点超出范围
        assert!(visit_order.is_empty());
    }
}


