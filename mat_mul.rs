
fn main() {
    let matA = [
        [1, 2, 3],
        [5, 4, 3],
        [1, 4, 3],
    ];

    let matB = [
        [1, 2, 3],
        [3, 4, 3],
        [2, 3, 4],
    ];

    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += matA[i][k] * matB[k][j];
            }
        }
    }

    println!("The result of matrix multiplication is:");
    for row in &result {
        println!("{:?}", row);
    }
}


