
// 18

extern crate graphic;

use self::graphic::*;
use std::iter::repeat;

pub fn longest_path_from_top_to_bottom(triangle: &str) -> i64 {

    let ns: Vec<Vec<i64>> = triangle.lines().skip(1).map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();

    let head: i64 = match triangle.lines().take(1).next() {
        None => 0,
        Some(n) => n.parse().unwrap()
    };

    let mut graph : DirectedGraph<String> = DirectedGraph::new();

    let head_vertex = graph.add_vertex("head".to_string());

    let v0_0 = graph.add_vertex(vertex_name(0, 0));
    graph.connect(head_vertex, v0_0, head).unwrap();

    let mut vertices = vec![v0_0];

    for (weights, row_n) in ns.iter().zip(1..) {
        let new_vertices: Vec<VertexId> =
            repeat(row_n)
            .zip(0..)
            .take(weights.len())
            .map(|(r,c)| graph.add_vertex(vertex_name(r,c)))
            .collect();

        for (&vertex, vertex_ix) in vertices.iter().zip(0..) {
            println!("Connecting {} to {} with weight {}", vertex, new_vertices[vertex_ix], weights[vertex_ix]);
            graph.connect(vertex, new_vertices[vertex_ix], weights[vertex_ix]).unwrap();
            println!("Connecting {} to {} with weight {}", vertex, new_vertices[vertex_ix+1], weights[vertex_ix+1]);
            graph.connect(vertex, new_vertices[vertex_ix+1], weights[vertex_ix+1]).unwrap();
        }

        vertices = new_vertices;
    }

    graph.longest_distance_from(head_vertex).unwrap().1
}

fn vertex_name(row: u32, column: u32) -> String {
    format!("{}-{}", row, column)
}
