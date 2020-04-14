# CalcAnalyzer

A very simple lexical analyzer that I built to learn some rust.

The analyzer must accept:

- +, -, *, /, ** as operators
- (,),{,} as symbols
- positive and negative numbers

After analyze should output a table displaying the found tokens. The analyzer should exit if it finds any illegal character.

# Requirements

You only need [Rust](https://www.rust-lang.org/tools/install) to built and run.

Optionally if you want just run the analyzer, you can execute using the docker image, see instructions bellow.

# Executing

## Docker

To execute using docker you can use the image [renorram/calcanalyzer](https://hub.docker.com/r/renorram/calcanalyzer)

Example:

```shell script
 docker run -ti renorram/calcanalyzer
```

## Built and Execute

You can built and execute from source.

```shell script
# clone the repository
git clone https://github.com/renorram/calcanalyzer

# enter in the folder
cd calcanalyzer

# built and execute release version
cargo run --release
```
