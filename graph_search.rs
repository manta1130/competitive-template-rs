use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DfsType {
    Preorder,
    Inorder,
    Postorder,
}

#[allow(clippy::ptr_arg)]
pub fn topological_sort(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut r = vec![];
    let mut flag = vec![false; graph.len()];
    for i in 0..graph.len() {
        if !flag[i] {
            flag[i] = true;
            internal_graph_dfs(i, &mut flag, graph, &mut r, DfsType::Postorder);
        }
    }
    r.reverse();
    r
}

#[allow(clippy::ptr_arg)]
pub fn graph_dfs(start_vertex: usize, graph: &Vec<Vec<usize>>, t: DfsType) -> Vec<usize> {
    let mut r = vec![];
    let mut flag = vec![false; graph.len()];
    flag[start_vertex] = true;
    internal_graph_dfs(start_vertex, &mut flag, graph, &mut r, t);
    r
}

#[allow(clippy::ptr_arg)]
pub fn graph_bfs(start_vertex: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut r = vec![];
    let mut flag = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_front(start_vertex);
    flag[start_vertex] = true;
    while !queue.is_empty() {
        let pop = queue.pop_back().unwrap();
        r.push(pop);
        for &next_vertex in &graph[pop] {
            if flag[next_vertex] {
                continue;
            }
            flag[next_vertex] = true;
            queue.push_front(next_vertex);
        }
    }
    r
}

#[allow(clippy::ptr_arg)]
fn internal_graph_dfs(
    vertex: usize,
    flag: &mut Vec<bool>,
    graph: &Vec<Vec<usize>>,
    r: &mut Vec<usize>,
    t: DfsType,
) {
    if t == DfsType::Preorder {
        r.push(vertex);
    }
    for (idx, &next_vertex) in graph[vertex].iter().enumerate() {
        if flag[next_vertex] {
            continue;
        }
        flag[next_vertex] = true;
        internal_graph_dfs(next_vertex, flag, graph, r, t);
        if t == DfsType::Inorder && idx == 0 {
            r.push(vertex);
        }
    }
    if t == DfsType::Postorder {
        r.push(vertex);
    }
}
