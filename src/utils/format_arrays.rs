use ndarray::{Array1, Array2};

pub fn matrix_to_string_ndarray(a: &Array2<f64>) -> String {
    let mut result = String::from("[\n");
    for row in a.rows() {
        result.push_str("  [");
        result.push_str(
            &row.iter()
                .map(|x| format!("{:>10.6}", x))
                .collect::<Vec<_>>()
                .join(", "),
        );
        result.push_str("],\n");
    }
    result.push(']');
    result
}

pub fn vector_to_string_ndarray(b: &Array1<f64>) -> String {
    let mut result = String::from("[\n");
    for val in b.iter() {
        result.push_str(&format!("  {:>10.6},\n", val));
    }
    result.push(']');
    result
}