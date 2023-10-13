# Memdbg

Memdbg provides the `Buf` struct which implements Debug
to provide a peak into memory akin to hex readers.

The `buf_dbg` macro extends this view to any struct.

## Example

```rust
let buf = memdbg::Buf(*b"\x41 \x68 \x90 \x00 \x2f This is a test string, The Line Break will demonstrate multiline formatting!");
dbg!("{buf:?}");
```
will display
```text
| 41 20 68 20 90 20 00 20 | 2F 20 54 68 69 73 20 69 | 73 20 61 20 74 65 73 74 | 20 73 74 72 69 6E 67 2C | A . h . . . . . / . T h i s . i s . a . t e s t . s t r i n g ,
| 20 54 68 65 20 4C 69 6E | 65 20 42 72 65 61 6B 20 | 77 69 6C 6C 20 64 65 6D | 6F 6E 73 74 72 61 74 65 | . T h e . L i n e . B r e a k . w i l l . d e m o n s t r a t e
| 20 6D 75 6C 74 69 6C 69 | 6E 65 20 66 6F 72 6D 61 | 74 74 69 6E 67 21                                 | . m u l t i l i n e . f o r m a t t i n g !
```
