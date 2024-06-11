# Maybe a good Arithmetic Encoding Implementation used for the neuralink_challenge: May 2024

Own lossles implementation of Arithmetic Encoding [Wikipedia](https://en.wikipedia.org/wiki/Arithmetic_coding)
- Supported Files
  - [x] riff wav
    - [x] 16bit
    - [ ] other bits
  - [ ] other audio formats
-  [x] Encoding
   - [x] Arithmetic Encoding
     - [ ] Recursive
     - [ ] Efficient
     - [x] Stream Support
   - [ ] Huffman Encoding
     - [ ] Recursive
     - [ ] Efficient
     - [ ] Stream Support
-  [ ] Decoding
   - [ ] Arithmetic Encoding
     - [ ] Efficient


## Needs
Rust (Cargo)

### Create lookup-table

```./create_table <input.wav> [<table.aet>]```

### Compiling:
```cargo build``` or ```cargo build --bin <encode/decode/create_table>```

### Run without Compiling
```cargo run --bin <encode/decode> <in.rae> <out.rae> [<table.aet>]```

## Running
### Encode:
```
./encode <in.wav> <out.rae> [<table.aet>]
```
### Decode:
```
./decode <in.rae> <out.wav> [<table.aet>]
```

## compressed File Header documentation

(0-16) Little Endian
(data) 101 = .625

| Offset				  | Length              | Data              |
|-----------------|---------------------|-------------------|
| 0 	(0x00)			|	8 (u64)             |	\<num elements\>  |
| 8 	(0x08)			| \<num elements\>/8  | \<data\>          |