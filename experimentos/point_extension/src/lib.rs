use magnus::{define_module, eval, function, method, prelude::*, Error, RArray, Ruby, Value};

#[magnus::wrap(class = "Point")]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn x(&self) -> isize {
        self.x
    }

    fn y(&self) -> isize {
        self.y
    }

    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn validate_transactions_from_query(target: i64) -> Vec<i64> {
    let transactions = eval::<RArray>("Model.all").unwrap();

    transactions
        .into_iter()
        .filter_map(|transaction| {
            let field1: i64 = transaction.funcall("x", ()).unwrap();
            let field2: i64 = transaction.funcall("y", ()).unwrap();

            if field1 + field2 == target {
                Some(field1 + field2)
            } else {
                None
            }
        })
        .collect()
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let class = ruby.define_class("Point", ruby.class_object())?;
    class.define_singleton_method("new", function!(Point::new, 2))?;
    class.define_method("x", method!(Point::x, 0))?;
    class.define_method("y", method!(Point::y, 0))?;
    class.define_method("distance", method!(Point::distance, 1))?;

    let class = ruby.define_class("Model", ruby.class_object())?;
    class.define_singleton_method(
        "calculate_from_rust",
        function!(validate_transactions_from_query, 1),
    )?;

    Ok(())
}
