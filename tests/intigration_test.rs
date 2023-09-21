use todo2::todo;

#[test]
#[should_panic]
fn test_by_condition_with_expired_date() {
    todo!("Delete this test", by: 2023-9-20);
}

#[test]
#[should_panic]
fn test_by_condition_with_expired_date_and_time() {
    todo!("Delete this test", by: 2023-9-20 at 9:00);
}

#[test]
#[should_panic]
fn test_by_condition_with_expired_year() {
    todo!("Are you time travel?", by: 2003);
}

#[test]
#[should_panic]
fn test_by_condition_with_expired_year_and_hour() {
    todo!("hello", by: 2003 at 9);
}

#[test]
#[should_panic]
fn test_by_condition_with_expired_year_and_minute() {
    todo!("hey", by: 2003 at 9:01);
}

#[test]
#[should_panic]
fn test_by_condition_with_expired_year_and_month() {
    todo!("Maybe you wanna update this", by: 2003-2);
}

#[test]
#[should_panic]
fn test_by_condition_with_expired_year_and_month_and_hour() {
    todo!("Hi", by: 2003-2 at 9);
}

#[test]
#[should_panic]
fn test_by_condition_with_expired_year_and_month_and_minute() {
    todo!("I was not here yet!", by: 2003-2 at 9:01);
}

#[test]
fn test_by_condition_with_future_date() {
    todo!("hey, old man 🧓 update this test", by: 2033-03-26);
}

#[test]
fn test_by_condition_with_future_date_and_time() {
    todo!("Hey, are you still alive? update this test if you", by: 2034-03-26 at 9:00);
}

#[test]
#[should_panic]
fn test_if_condition_straightforward_true() {
    todo!("Delete this test", if: 1 == 1);
}

#[test]
#[should_panic]
fn test_if_condition_with_variable_true() {
    let a = 1;
    todo!("Delete this test", if: a == 1);
}

#[test]
fn test_if_condition_straightforward_false() {
    todo!("Math has updated!, your so old", if: 1 == 2);
}

#[test]
fn test_if_condition_with_variable_false() {
    let a = 1;
    todo!("Math has updated!, your so old", if: a == 2);
}

#[test]
#[should_panic]
fn test_two_conditions_panic_by_the_first() {
    todo!("Delete this test", if: 1 == 1, by: 2028-9-20);
}

#[test]
#[should_panic]
fn test_tow_conditions_panic_by_the_second() {
    todo!("Delete this test", by: 2028-9-20, if: 1 == 1);
}

#[test]
#[should_panic]
fn test_todo_with_time_at_symbol_and_expired_date() {
    todo!("Delete this test", by: 2023-9-20@9:00);
}

#[test]
#[should_panic]
fn test_todo_with_time_at_symbol_and_expired_year() {
    todo!("Are you time travel?", by: 2003@9:00);
}

#[test]
#[should_panic]
fn test_todo_with_time_at_symbol_and_expired_year_and_hour() {
    todo!("hello", by: 2003@9);
}

#[test]
#[should_panic]
fn test_todo_with_time_at_symbol_and_expired_year_and_minute() {
    todo!("hey", by: 2003@9:01);
}

#[test]
#[should_panic]
fn test_todo_with_time_at_symbol_and_expired_year_and_month() {
    todo!("Maybe you wanna update this", by: 2003-2@9:00);
}

#[test]
#[should_panic]
fn test_todo_with_time_at_symbol_and_expired_year_and_month_and_hour() {
    todo!("Hi", by: 2003-2@9);
}

#[test]
#[should_panic]
fn test_todo_with_time_at_symbol_and_expired_year_and_month_and_minute() {
    todo!("I was not here yet!", by: 2003-2@9:01);
}

#[test]
fn test_todo_with_time_at_symbol_future_date() {
    todo!("Do you still remember me?", by: 2033-03-26@9:00);
}

#[test]
fn test_todo_with_time_at_symbol_and_hour() {
    todo!("I'm not sure if you are still alive", by: 2053-03-26@9);
}
