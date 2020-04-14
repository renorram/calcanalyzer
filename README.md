# CalcAnalyzer

A simples analisador léxico de uma calculadora feito para cadeira de compiladores.

# Requerimentos

Para executar o buildar o projeto e executar basta ter o [Rust](https://www.rust-lang.org/tools/install) instalado.

# Como executar

## Docker

Caso tenha o docker instalado basta executar via terminal o comando:

```shell script
 docker run -ti renorram/calcanalyzer
```

## Regular

Após ter o Rust instalado basta utilizar o cargo do rust para executar em um terminal o comando:

```shell script
cargo run --release
``` 

OBS.: Se certificar de que o cargo está no PATH para que possa ser chamado na linha de comando.
