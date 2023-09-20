use todo_or_die::todo;

fn main() {
    todo!("Hi", by: 2024-01-01);
}

#[cfg(test)]
mod tests {
    use todo_or_die::todo;

    #[test]
    #[should_panic]
    fn test_todo_work() {
        todo!("Delete this test", by: 2023-9-20);
    }

    #[test]
    #[should_panic]
    fn test_if_work() {
        todo!("Delete this test", if: true);
    }

    #[test]
    #[should_panic]
    fn test_if_work2() {
        todo!("Delete this test", if: 1 == 1);
    }

    #[test]
    #[should_panic]
    fn test_multiple_conditions_panic_by_the_first() {
        todo!("Delete this test", if: 1 == 1, by: 2028-9-20);
    }

    #[test]
    #[should_panic]
    fn test_multiple_conditions_panic_by_the_second() {
        todo!("Delete this test", by: 2028-9-20, if: 1 == 1);
    }

    #[test]
    #[should_panic]
    fn test_todo_with_time_work() {
        todo!("Delete this test", by: 2023-9-20 at 9:00);
    }

    /*    #[test]
    #[should_panic]
    fn test_todo_panic() {
        todo!();
        todo!(true لااا, by 2021-01-01);
    }*/
}
