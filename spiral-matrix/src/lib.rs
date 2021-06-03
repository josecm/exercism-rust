pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }

    let size = size as usize;
    let mut matrix = vec![vec![0; size]; size];

    let mut counter = 1;
    for l in 0..(size / 2 + 1) {
        for i in l..(size - l) {
            matrix[l][i] = counter;
            counter += 1;
        }
        for i in l + 1..(size - l) {
            matrix[i][size - 1 - l] = counter;
            counter += 1;
        }
        for i in (l..(size - l - 1)).rev() {
            matrix[size - 1 - l][i] = counter;
            counter += 1;
        }
        for i in (l + 1..(size - l - 1)).rev() {
            matrix[i][l] = counter;
            counter += 1;
        }
    }

    matrix
}
