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

    /*    #[test]
    #[should_panic]
    fn test_todo_panic() {
        todo!();
        todo!(true لااا, by 2021-01-01);
    }*/
}
