/// Операция над двумя выражениями.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// Выражение в форме узла дерева.
#[derive(Debug)]
enum Expression {
    /// Операция над двумя дочерними выражениями.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Значение
    Value(i64),
}

/// Рекурсивная функция для вычисления значения выражения.
fn eval(e: Expression) -> i64 {
    match e {
        Expression::Value(value) => value, // Если это значение, возвращаем его
        Expression::Op { op, left, right } => {
            let left_value = eval(*left); // Рекурсивно вычисляем левое поддерево
            let right_value = eval(*right); // Рекурсивно вычисляем правое поддерево

            // Выполняем операцию в зависимости от типа операции
            match op {
                Operation::Add => left_value + right_value,
                Operation::Sub => left_value - right_value,
                Operation::Mul => left_value * right_value,
                Operation::Div => left_value / right_value,
            }
        }
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() { // 10 + 20
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() { // (10 * 9) + ((3 - 4) * 5)
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

#[test]
fn test_zeros() { 
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
}

fn main() {
    // Пример использования 10 + 20
    let expr = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(20)),
    };
    println!("Результат: {}", eval(expr));
}