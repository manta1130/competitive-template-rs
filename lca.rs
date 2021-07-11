pub struct Lca {
    graph: Vec<Vec<usize>>,
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
    bit_size: usize,
    dfs_start: usize,
    preprocessing_complete: bool,
}

impl Lca {
    pub fn new(v_size: usize, dfs_start: usize) -> Lca {
        assert!(v_size > dfs_start);
        let mut bit_size = 0;
        while 1 << bit_size < v_size {
            bit_size += 1;
        }
        Lca {
            graph: vec![vec![]; v_size],
            parent: vec![vec![dfs_start; v_size]; bit_size],
            depth: vec![0; v_size],
            bit_size,
            dfs_start,
            preprocessing_complete: false,
        }
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        assert!(!self.preprocessing_complete);
        self.graph[v1].push(v2);
        self.graph[v2].push(v1);
    }

    pub fn calc(&mut self) {
        assert!(!self.preprocessing_complete);
        self.preprocessing_complete = true;
        self.dfs(self.dfs_start, self.dfs_start, 0);

        for i in 0..self.bit_size - 1 {
            for j in 0..self.graph.len() {
                self.parent[i + 1][j] = self.parent[i][self.parent[i][j]];
            }
        }
    }

    pub fn get_lca(&self, mut v1: usize, mut v2: usize) -> usize {
        assert!(self.preprocessing_complete);
        if self.depth[v1] > self.depth[v2] {
            std::mem::swap(&mut v1, &mut v2);
        }
        if self.depth[v1] != self.depth[v2] {
            v2 = self.get_parent(v2, self.depth[v2] - self.depth[v1]);
        }
        if v1 == v2 {
            return v1;
        }
        for i in (0..self.bit_size).rev() {
            if self.parent[i][v1] != self.parent[i][v2] {
                v1 = self.parent[i][v1];
                v2 = self.parent[i][v2];
            }
        }
        self.parent[0][v1]
    }

    pub fn get_depth(&self, v: usize) -> usize {
        assert!(self.preprocessing_complete);
        self.depth[v]
    }

    fn dfs(&mut self, n: usize, p: usize, d: usize) {
        self.depth[n] = d;
        self.parent[0][n] = p;
        for i in self.graph[n].clone() {
            if i == p {
                continue;
            }
            self.dfs(i, n, d + 1);
        }
    }

    fn get_parent(&self, mut n: usize, c: usize) -> usize {
        for i in 0..self.bit_size {
            if (c >> i) & 1 == 1 {
                n = self.parent[i][n];
            }
        }
        n
    }
}
