fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b{
        println!("{a} is greater than {b}");
        return a;
    } else if a==b{
        println!("{a} equals {b}");
        return a;
    } else {
        println!("{b} is greater than {a}");
        return b;
    }
}

fn main() {
    bigger(10,10);
    bigger(11,12);
    bigger(12,11);
    // You can optionally experiment here.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
