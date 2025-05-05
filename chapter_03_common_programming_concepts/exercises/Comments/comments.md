# Comments

Infelizmente comentários no Rust existem apenas linha a linha, o que isso quer dizer? Para adicionar um comentário, adicione `//` ao início da linha, sendo assim:

```rust
// Isso é um comentário em rust
```

No entanto... para fazer um bloco de comentários teriamos que fazer algo como:

```rust
// Então, estamos fazendo algo complicado aqui, longo o suficiente para precisarmos de
// várias linhas de comentários para fazer isso! Ufa! Espero que este comentário
// explique o que está acontecendo.
```

> Sim, horrível. Mas, é o que temos, nada é perfeito.

Podemos comentar o código também, dessa forma:


```rust
fn main() {
    println!("Hello, world!"); // printa um hello, world! óbvio
}
```

```rust
fn main() {
    // essa é a função principal do programa, que até o momento apenas printa
    // um hello, world!
    println!("Hello, world!");
}
```