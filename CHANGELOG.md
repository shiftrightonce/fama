# Change logs

## Version 0.3.1

- Add the ability to return Option<T>. When T is none, the pipe flow will stop. The returned value is also stored in the pipeline
- Add the ability to return and store Result<T, E>. When result is `err` the pipe flow will be stopped

## Version 0.3.0

Pipe handlers can now specify returned value. This allows you to use general function or struct instead
of specific ones. 

## Version 0.2.0

The entire crate was reimplemented. `Busybody` is use for dependency injecting.