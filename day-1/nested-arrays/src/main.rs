// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut columns: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            columns[j][i] = matrix[i][j];
        }
    }

    columns
}

#[test]
fn test_transpose() {
    let matrix = [
        [1, 2, 3], //
        [4, 5, 6],
        [7, 8, 9],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [1, 4, 7,], //
            [2, 5, 8,],
            [3, 6, 9,],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
