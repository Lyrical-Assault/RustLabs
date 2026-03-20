// Вычислите модуль вектора просуммировав квадраты его координат
// и вычислив кввдратный корень полученного значения. Используйте метод `sqrt()` для вычисления
// корня, следующим образом: v.sqrt().

fn magnitude(vector: &[f64; 3]) -> f64 {
    let sum_of_squares = vector[0].powi(2) + vector[1].powi(2) + vector[2].powi(2);
    sum_of_squares.sqrt()
}

// Нормализуйте вектор вычислив его модуль и разделив все его координаты на 
// этот модудль.
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    vector[0] /= mag;
    vector[1] /= mag;
    vector[2] /= mag;
}

// Основная функция для проверки
fn main() {
    println!("Модуль единичного вектора: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Модуль {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Модуль {v:?} после нормализации: {}", magnitude(&v));
}