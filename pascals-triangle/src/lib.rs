pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        let mut row = vec![1];
        for _ in 0..(row_count) {
            rows.push(row.clone());
            row = next(&row);
        }
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn next(curr: &Vec<u32>) -> Vec<u32> {
    let mut next = vec![1];
    for chunk in curr.windows(2) {
        next.push(chunk.iter().sum::<u32>());
    }
    next.push(1);
    next
}
