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
   - [ ] Write denom in Scientific format (sample denom = 4 * 16^524, what currently is saved as 523 times 0x00 and 0x04 LittleEndian)
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

## Header documentation

Everything is Little Endian

| Offset				  | Length					| Data            |
|-----------------|-----------------|-----------------|
| 0 	(0x00)			|	8						  	|	\<nom size\>    |
| 8 	(0x08)			|	8								| \<denom size\>  |
| 16 	(0x10)			|	8								| \<data size\>   |
| 24 	(0x18)			|	\<nom size\>	  | \<nominator\>   |
| 24+\<nom size\> |	\<denom size\>  | \<denominator\> |