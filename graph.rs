use std::cmp;
use std::collections::BinaryHeap;

///最短経路計算(ベルマンフォード法)
///
///戻り値がtrueなら負の閉路がある。
///
///graph:グラフ情報(隣接リスト)
///
///dist:最短経路を格納する配列
///
///事前に始点には0を入れ、その他はNoneで初期化する必要がある。
pub fn bellman_ford(graph: &[&[(usize, isize)]], dist: &mut [Option<isize>]) -> Vec<bool> {
    //負の閉路検出用
    let mut neg_flag = vec![false; dist.len()];

    for _ in 0..graph.len() {
        for (from, v) in graph.iter().enumerate() {
            for e in v.iter() {
                let cost = e.1;
                let to = e.0;
                if let Some(x) = dist[from] {
                    if dist[to].is_none() {
                        dist[to] = Some(x + cost);
                    } else if let Some(y) = dist[to] {
                        if y > x + cost {
                            dist[to] = Some(x + cost);
                        }
                    }
                }
            }
        }
    }

    for _ in 0..graph.len() {
        for (from, v) in graph.iter().enumerate() {
            for e in v.iter() {
                let cost = e.1;
                let to = e.0;
                if let Some(x) = dist[from] {
                    if dist[to].is_none() {
                        dist[to] = Some(x + cost);
                        neg_flag[to] = true;
                    } else if let Some(y) = dist[to] {
                        if y > x + cost {
                            dist[to] = Some(x + cost);
                            neg_flag[to] = true;
                        }
                    }
                }
                if neg_flag[from] {
                    neg_flag[to] = true;
                }
            }
        }
    }
    neg_flag
}

///最短経路計算(ダイクストラ法)
///
///最短経路が格納された配列を返す。(到達できない場合はisize::max_value()を返す。)
///
///graph:グラフ情報(隣接リスト)
///
///start:始点

pub fn dijkstra(graph: &[&[(usize, isize)]], start: usize) -> Vec<isize> {
    let mut heap = BinaryHeap::new();
    let mut dist = vec![isize::max_value(); graph.len()];
    dist[start] = 0;

    heap.push((0_isize, start));

    while !heap.is_empty() {
        let e = heap.pop().unwrap();
        if e.0.wrapping_neg() > dist[e.1] {
            continue;
        }
        for next_e in graph[e.1].iter() {
            if dist[next_e.0] > next_e.1 + dist[e.1] {
                dist[next_e.0] = next_e.1 + dist[e.1];
                heap.push(((dist[e.1] + next_e.1).wrapping_neg(), next_e.0));
            }
        }
    }
    dist
}

///最短経路計算(ワーシャルフロイド法)
pub fn warshall_floyd(graph: &mut [&mut [isize]]) {
    for k in 0..graph.len() {
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                graph[i][j] = cmp::min(graph[i][j], graph[i][k] + graph[k][j]);
            }
        }
    }
}
