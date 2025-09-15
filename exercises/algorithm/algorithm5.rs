/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// 定义无向图结构（邻接表表示）
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // 创建包含n个顶点的空图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 为图添加无向边（src与dest互相关联）
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    /// 广度优先搜索实现：从start顶点出发，返回节点访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 1. 边界检查：若起始顶点超出图的范围，返回空数组
        if start >= self.adj.len() {
            return vec![];
        }

        // 2. 初始化：访问标记数组（false=未访问，true=已访问）
        let mut visited = vec![false; self.adj.len()];
        // 队列：存储待访问的节点（BFS核心数据结构）
        let mut queue = VecDeque::new();
        // 结果数组：存储节点访问顺序
        let mut visit_order = vec![];

        // 3. 启动BFS：标记起始节点为已访问，加入队列
        visited[start] = true;
        queue.push_back(start);

        // 4. 循环处理队列，直到队列为空
        while let Some(current) = queue.pop_front() {
            // 4.1 记录当前节点的访问顺序
            visit_order.push(current);

            // 4.2 遍历当前节点的所有邻接节点
            for &neighbor in &self.adj[current] {
                // 若邻接节点未访问，则标记并加入队列
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }

    // 新增测试：起始节点超出范围
    #[test]
    fn test_bfs_invalid_start() {
        let graph = Graph::new(3); // 顶点0、1、2
        let visited_order = graph.bfs_with_return(3); // 起始顶点3无效
        assert_eq!(visited_order, vec![]);
    }

    // 新增测试：非连通图（仅访问起始节点所在的连通分量）
    #[test]
    fn test_bfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        // 连通分量1：0-1-2
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        // 连通分量2：3-4（与分量1无连接）
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]); // 仅访问分量1
    }
}

