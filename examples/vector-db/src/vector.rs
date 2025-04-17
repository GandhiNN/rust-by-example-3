#![allow(unused)]

type Vector = [f32; 3]; // example for 3D vectors

pub struct VectorDB {
    pub vectors: Vec<Vector>,
}

impl VectorDB {
    pub fn new() -> Self {
        VectorDB {
            vectors: Vec::new(),
        }
    }

    pub fn add_vector(&mut self, vector: Vector) {
        self.vectors.push(vector);
    }

    pub fn get_vector(&self, index: usize) -> Option<&Vector> {
        self.vectors.get(index)
    }

    pub fn find_closest(&self, query: Vector) -> Option<&Vector> {
        self.vectors.iter().min_by(|&a, &b| {
            let distance_a = VectorDB::euclidean_distance(&query, a);
            let distance_b = VectorDB::euclidean_distance(&query, b);
            distance_a.partial_cmp(&distance_b).unwrap()
        })
    }

    fn euclidean_distance(a: &Vector, b: &Vector) -> f32 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}
