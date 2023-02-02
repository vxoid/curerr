# Curerr
- [x] A crate for error handling

## Documentation
- [x] rust docs - https://docs.rs/curerr

## Example
```rust
fn devide(a: i32, b: i32) -> Result<i32, CursedErrorHandle> {
    if b == 0 {
        return Err(CursedErrorHandle::new(
            CursedError::InvalidArgument,
            "0 devision!!!".to_string()
        ))
    }

    Ok(a/b)
}

let result = devide(6, 3).expect("devision error");
```