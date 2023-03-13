# Curerr v1.0.7
- [x] A crate for error handling

## Documentation
- [x] rust docs - https://docs.rs/curerr

## Changelog
- Implemented `From<ErrorKind>` for CursedError
- Added CursedErrorType enum
- Modified CursedError enum

## v1.0.6 yank reasons
- CursedError implementation wasn't complete
- Documentation was old

## Example
```rust
fn devide(a: i32, b: i32) -> Result<i32, CursedErrorHandle> {
    if b == 0 {
        return Err(CursedErrorHandle::new(
            CursedError::Argument(CursedErrorType::Invalid),
            "0 division!!!".to_string()
        ))
    }

    Ok(a/b)
}

let result = devide(6, 3).expect("division error");
```