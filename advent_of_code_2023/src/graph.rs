pub struct Graph<T> {
    edges: Vec<Edge<T>>,
}

pub struct Edge<T> {
    from: T,
    to: T,
}

pub struct Node<T> {
    value: T,
    weight: u32,
}
