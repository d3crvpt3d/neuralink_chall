# Maybe a good Arithmetic Encoding Implementation used for the neuralink_challenge: May 2024

Own lossles implementation of Arithmetic Encoding

- Supported Files
  - [x] riff wav
    - [x] 16bit
    - [ ] other bits
  - [ ] other audio formats
-  [x] Encoding
   - [x] Arithmetic Encoding
     - [ ] Recursive
     - [ ] Efficient
     - [ ] Stream Support
   - [ ] Huffman Encoding
     - [ ] Recursive
     - [ ] Efficient
     - [ ] Stream Support
-  [ ] Decoding
   - [ ] Arithmetic Encoding
     - [ ] Efficient


## Needs
Rust (Cargo)

## Run without Compiling
```cargo run --bin <encode/decode> <in.rae> <out.rae>```

## Compiling:
```cargo build``` or ```cargo build --bin <encode/decode>```

## Running
### Encode:
```
./encode <in.wav> <out.rae>
```
### Decode:
```
./decode <in.rae> <out.wav>
```
