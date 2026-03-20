fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3]; // Создаем пустую матрицу для результата

    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j]; // Меняем строки и столбцы местами
        }
    }

    result // Возвращаем транспонированную матрицу
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [
        [1, 2, 3], // <-- комментарий заставляет rustfmt добавить новую строку
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("матрица: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("транспонированная матрица: {:#?}", transposed);
}