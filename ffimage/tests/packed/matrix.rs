use ffimage::packed::Matrix;

#[test]
fn new() {
    let matrix = Matrix::new(3, 3, 0u8);
    assert_eq!(matrix.rows(), 3);
    assert_eq!(matrix.cols(), 3);
    assert_eq!(matrix.as_ref().len(), 3 * 3);
}

#[test]
fn resize() {
    let mut matrix = Matrix::new(0, 0, 0u8);
    matrix.resize(2, 4, 0);
    assert_eq!(matrix.rows(), 2);
    assert_eq!(matrix.cols(), 4);
    assert_eq!(matrix.as_ref().len(), 2 * 4);
}

#[test]
fn from_buf() {
    let mem = [0u8; 5];
    let matrix = Matrix::from_buf(&mem, 2, 2).expect("This is a valid matrix");
    assert_eq!(matrix.rows(), 2);
    assert_eq!(matrix.cols(), 2);
    assert_eq!(matrix.as_ref().len(), 2 * 2 + 1);
}

#[test]
fn row_stride() {
    let mem = [0u16; 6];
    let matrix = Matrix::from_buf_with_stride(&mem, 2, 3, 2).expect("This is a valid matrix");
    assert_eq!(matrix.row_stride(), 3);
    assert_eq!(matrix.as_ref().len(), 6);
}

#[test]
fn index() {
    let matrix = Matrix::new(2, 2, 2u8);
    assert_eq!(matrix[1], [2, 2]);
}

#[test]
fn index_mut() {
    let mut matrix = Matrix::new(2, 2, 2u8);
    matrix[1][0] = matrix[1][0] + 1;
    assert_eq!(matrix[1], [3, 2]);
}
