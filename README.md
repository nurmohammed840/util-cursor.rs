A simple cursor implementation for reading data from array.
It's similar to `std::io::Cursor` from std library.

## Usage

add to your `Cargo.toml`:

```toml
[dependencies]
util-cursor = "0.1"
```

```rust
use util_cursor::Cursor;

let data = [1, 2, 3, 4, 5];
let mut cursor = Cursor::new(data.as_ref());

assert_eq!(cursor.read_slice(3), Some(&[1, 2, 3][..]));

assert_eq!(cursor.offset, 3);
assert_eq!(cursor.remaining_slice(), [4, 5].as_ref());
```