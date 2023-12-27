/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right } => {
            let left_value = eval(*left)?;
            let right_value = eval(*right)?;
            match op {
                Operation::Add => {
                    if left_value.checked_add(right_value).is_none() {
                        return Err(String::from("integer overflow"));
                    } else {
                        return Ok(left_value + right_value);
                    }
                }
                Operation::Sub => Ok(left_value - right_value),
                Operation::Mul => {
                    if left_value.checked_mul(right_value).is_none() {
                        return Err(String::from("integer overflow"));
                    } else {
                        return Ok(left_value * right_value);
                    }
                }
                Operation::Div => {
                    if right_value == 0 {
                        return Err(String::from("division by zero"));
                    } else {
                        return Ok(left_value / right_value);
                    }
                }
            }
        }
        Expression::Value(x) => Ok(x),
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
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
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}

#[test]
fn test_overflow() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(9223372036854775807)),
            right: Box::new(Expression::Value(2)),
        }),
        Err(String::from("integer overflow"))
    )
}
