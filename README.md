# todo2

todo2(a.k.a. todo or die) - A better `todo!` macro inspired from [searls/todo_or_die](https://github.com/searls/todo_or_die)

[![crates.io](https://img.shields.io/crates/v/todo2.svg)](https://crates.io/crates/todo2)
[![docs.rs](https://docs.rs/todo2/badge.svg)](https://docs.rs/todo2)
[![downloads](https://img.shields.io/crates/d/todo2.svg)](https://crates.io/crates/todo2)
[![license](https://img.shields.io/crates/l/todo2.svg)](https://github.com/0x61nas/todo2/blob/aurora/LICENSE)

This crate provides a better `todo!` macro, which allows you to specify the deadline and the condition when the code should be implemented.
and when the condition or the deadline is met, the code will panic or emit a compile error, or just log an error.

> Note: this crate is still in the early development, so it may have some bugs, and the API may change in the future.
> If you have any suggestions or you found a bug, please open an issue or a pull request.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
todo2 = "0.1.0"
```
or just run this command:
```sh
cargo add todo2
```

```rust
#[macro_use]
extern crate todo2;

fn main() {
  todo!("Hack NASA", by: 2024-3-26 at 9:00);
  get_a_hot_gf(true)
}

fn get_a_hot_gf(single: bool) {
 todo!("Get a hot girlfriend", if: single);
}
```

## Features
- `log` - Just logs an error instead of panicking or emitting a compile error, this may useful in the serious projects, this feature respects that you have added the `log` crate to your dependencies
- `compile-error` - Emits a compile error instead of panicking.
- `with-chrono` - Enables the `chrono` this enables you to specify the deadline for the `by` condition using the [`chrono::Utc`](https://docs.rs/chrono/latest/chrono/struct.Utc.html) or [`chrono::DateTime`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) types. [not implemented yet](#Maybe?)
- `with-time` - Enables the `time` this enables you to specify the deadline for the `by` condition using the [`time::OffsetDateTime`](https://docs.rs/time/latest/time/struct.OffsetDateTime.html) type or the [`time::macros::datetime`](https://docs.rs/time/0.3.28/time/macros/macro.datetime.html) macro. [not implemented yet](#Maybe?)
- `and-time` - allows you to specify a specific time of the day in the `by` condition
- `original-compatibility` - Allows you to use this macro without pass any arguments, or with only the message.
- `strict-syntax` - Enables the strict syntax,, just too force you to put a comma or a semicolon after the message.
- `chrono-backend` - Use the `chrono` as the backend instead of the default implementation for the `by` condition to calculate the unix time stamp. I prefer to enable this feature if I have chrono in the dependencies, because it's more accurate than the default implementation. [Read more](#Backends)
- `time-backend` - Use the `time` as the backend instead of the default implementation for the `by` condition to calculate the unix time stamp. I prefer to enable this feature if I have time in the dependencies, because it's more accurate than the default implementation. [Read more](#Backends)
- `am-cool` - To indicate that you are cool. I love you.
The default features are: `original-compatibility`, `strict-syntax`, `and-time`.

## Examples
### Using the `log` feature
You have to enable the `log` feature in your `Cargo.toml`:
```toml
[dependencies]
# You also have to add the `log` crate to your dependencies
log = "0.4.20"
# Use any implementation you want for the logging, in this example, I will use the `simple_logger` crate
simple_logger = "4.2.0"
# and of course, our beloved `todo2` crate
todo2 = { version = "0.1.0", features = ["log"] }
```
```rust
#[macro_use]
extern crate todo2;
#[macro_use]
extern crate log;

use simple_logger::SimpleLogger;

fn main() {
    // Initialize the logger
    SimpleLogger::new().init().unwrap();

    todo!("Make a cool crate", by: 2024-02-02)
}
```
This will log an error like this when the deadline is met:
```log
2024-02-02T17:27:07.013874956Z ERROR [logging_example] src/main.rs:9: Make a cool crate
```
consider that you can't enable the `compile-error` feature with the `log` feature. only one of them can be enabled at a time.

### Using the `compile-error` feature
First, add the crate to your `Cargo.toml` and enable the `compile-error` feature:
```sh
cargo add todo2 --features compile-error
```
```rust
#[macro_use]
extern crate todo2;

fn main() {
   todo!("Remove this secret", by: 2024-02-23);
   let my_little_secret = "very secret";
}
```
this will emit a compile error like this, when u try to compile the code in release mode.

## Time in the `by` condition
by default, the `by` condition takes a raw date and parse it with our custom parser, which expects the date in the `YYYY-MM-DD` format and `YYYY-MM-DD at HH:MM` or `YYYY-MM-DD @ HH:MM` format if you have the `and-time` feature enabled.
and then it calculates the unix time stamp in UTC, and then compares it with the current time stamp.

this for the parsing part, noting interested here. just macros magic. the complexity comes when we want to calculate the unix time stamp from the parsed date.
here the time zones and the daylight saving time and the leap seconds come to play.
and I don't want to deal with this complexity 'cause I'm lazy and this is a "proc macro" not a normal crate witch means that it runs at compile time, and we all know that the rust compile times is so "fast" :) and I don't want to make it slower.
so I implemented a simple algorithm to calculate the unix time stamp, which is not accurate, but it works. hmm, kinda.

### Backends
and that's why we have the `with-chrono` and the `with-time` features, to use the `chrono` or the `time` crate to calculate the unix time stamp instead of the default implementation.

I encourage you to enable one of them if you don't have a problem with adding yet another dependence to your project dependencies tree, or if you already have one of them in your dependencies already.
at least until we have a better implementation for the default backend.

the backend doesn't affect the parsing part, or the syntax, it only affects the calculation of the unix time stamp, witch is internal thing, so you don't have to worry about it from this perspective.

## Maybe?
Here some ideas that I may implement in the future releases:
- [ ] Implement the `with-chrono` feature, to enable the user to use the [`chrono::Utc`](https://docs.rs/chrono/latest/chrono/struct.Utc.html) or [`chrono::DateTime`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) types
to specify the deadline for the `by` condition instead of the raw date.
example:
```rust
todo!("Make a cool crate", by: chrono::Utc.with_ymd_and_hms(2024, 02, 02, 9, 0, 0));
```
- [ ] Implement the `with-time` feature, to enable the user to use the [`time::OffsetDateTime`](https://docs.rs/time/latest/time/struct.OffsetDateTime.html) type or the [`time::macros::datetime`](https://docs.rs/time/0.3.28/time/macros/macro.datetime.html) macro to specify the deadline for the `by` condition instead of the raw date.
example:
```rust
todo!("Make a cool crate", by: time::macros::datetime!(2024-02-02 09:00:00));
```
- [ ] Make the `if` condition parser able to evaluate some conditions at compile time, so we can use the `compile-error` feature with the `if` condition.
example:
```rust
   todo!("Remove this secret", if: !cfg!(debug_assertions));
   let my_little_secret = "i love you";
```

## Contributing
I'm happy to accept any contributions, just consider reading the [CONTRIBUTING.md](https://github.com/0x61nas/todo2/blob/aurora/CONTRIBUTING.md) guide first. to avoid waste waste our time on some unnecessary things.

> the main keywords are: **signed commits**, **conventional commits**, **no emojis**, **the PR shouldn't have more then tree commits most of the time**

## License
This project is licensed under the MIT license. [Read more](https://github.com/0x61nas/todo2/blob/aurora/LICENSE)
And you can use it under the Unlicense license if you want. [Read more](https://github.com/0x61nas/todo2/blob/aurora/LICENSE-UNLICENSE)

