fn collatz_length(mut n: i32) -> u32 {
    let mut length = 0;
    while n != 1 { 
        if n % 2 == 0 { // Если n[i] четное
            n /= 2;
        } else { // Если n[i] нечетное
            n = 3 * n + 1;
        }
        length += 1;
    }
    length // возвращаем длину последовательнисти Коллатца для числа n
}   

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 14); 
}

fn main() {
    println!("Длина: {}", collatz_length(11));
}