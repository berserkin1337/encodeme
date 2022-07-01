# ENCODEME

A cli written in rust to hide secret messages inside png files


Below on the left is the original image and on the right is the image with a secret message embedded inside it.

![original](/img/wing.png)
![embedded_img](/img/secret.png)



## Building
```bash
cargo build --release
```
## Usage
```bash
./target/release/encodeme -h
USAGE:
    encodeme [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    decode    Decodes a PNG image into a message
    encode    Encodes a message into a PNG image
    help      Print this message or the help of the given subcommand(s)
    remove    Removes the secret message from the PNG image
```

## Encoding a message into the file

### Encoding a message into the original file

```bash
$ ./target/release/encodeme encode --help
encodeme-encode 
Encodes a message into a PNG image

USAGE:
    encodeme encode [OPTIONS] -p <path> -t <chunk_type> -m <message>

OPTIONS:
    -h, --help             Print help information
    -m <message>           specify the message to encode
    -o <output>            specify the path of the output png image
    -p <path>              specify the path of the png image
    -t <chunk_type>        specify the chunk type of the message
```

```bash
$ ./target/release/encodeme encode  -p  img/wing.png  -t ruSt -m "My secret message"
```
### Encoding a message into a different file
```bash
$ ./target/release/encodeme encode  -p  img/wing.png  -t ruSt -m "My secret message" -o img/secret.png
```


## Decoding a message from the file
```bash
./target/release/encodeme decode  --help
encodeme-decode 
Decodes a PNG image into a message

USAGE:
    encodeme decode -p <chunk_type> -t <type>

OPTIONS:
    -h, --help             Print help information
    -p <chunk_type>        specify the path of the png image
    -t <type>              specify the chunk type of the message
```

```
$ ./target/release/encodeme decode  -p img/secret.png -t  ruSt
My secret message
```

## Removing a message from a file

```bash
./target/release/encodeme remove -h
encodeme-remove 
Removes the secret message from the PNG image

USAGE:
    encodeme remove -p <path> -t <type>

OPTIONS:
    -h, --help       Print help information
    -p <path>        specify the path of the png image
    -t <type>        specify the chunk type of the message
```

```bash
$ ./target/release/encodeme remove  -p img/secret.png -t  ruSt
My secret message
```