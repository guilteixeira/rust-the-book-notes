# 2. Programming a Guessing Game

## Conhecendo melhor os tipos de variáveis

Este capítulo do livro foi sensacional aprendemos um monte de recursos comuns em qualquer linguagem como declarar uma variável ou capturar um input do usuário.
Porém muita coisa tem um jeitinho `Rust` de fazer, além disso, conhecemos melhor da syntax e do porque de alguns recursos que vimos no capítulo 1.

Nesse capítulo aprendemos a importar nossa primeira biblioteca, ou `crate`, como é chamada no `Rust`. A crate `std` na qual iremos utilizar a função `io`, bem semelhante ao que é visto em outras linguagens como `C` e `C++`, mas com uma sintaxe mais amigável como no `PHP`. Para importar utilizar uma `crate`, nós adicionamos em nosso código `use crate::function;`, nesse capítulo, chamamos `use std::io;`.

Outra coisa bem legal nesse capítulo foi ver a forma com o `Rust`descreve uma variável, e explicando exatamente como fazemos para declarar variáveis que por padrão são imutáveis com `let` ou mutáveis com `let mut`. O `Rust` descreve variáveis como links para informações, fazendo uma analogia que nós conectamos uma informação a elas, e isso está super alinhado com como uma variável funciona em baixo nível, servindo basicamente como uma etiqueta para um espaço de memória.

**Observações:** Por padrão variáveis, vetores e entre outros recursos no Rust são imutáveis, é sempre importante evitar o uso de `mut` caso você não saiba por que exatamente está fazendo isso, por questões de segurança quase sempre vamos preferir variáveis que não fique mudando o tempo todo, apesar do nome ser justamente variável.

## Macros ainda não

Mais uma vez não tivemos muita explicação sobre o que é uma macro e como ela funciona, e pelo índice do livro, vimos que isso poderá entrar em detalhes apenas no capítulo 20. Me pergunto o por que ter apresentado o `println!` ao invés de utilizar a biblioteca `std` nesse início, visto que não havia intenção de demonstrar as macros nessa fase do livro.

## Referências são ponteiros seguros?

Outra coisa muito interessante foi o uso de referencia com o `&`, podendo ser uma referência a princípio imutável ou mutável, porém sem alterar o conteúdo de uma variável, o que torna tudo mais seguro, imagine isso como um ponteiro de `C` só que muito seguro. A syntax é bem simples podemos usar `&var` para uma referencia imutável, ou `&mut var` para uma referência mutável.

## Tratamento de erros com `expect`

Um tópico absurdamente bacana nesse livro e que aqui eu tenho que bater palmas com as mãos e com os pés, pois não imaginei ver algo assim tão cedo num livro de programação. O livro nos apresenta já nese segundo capítulo, sobre tratamento básico de erros, usando `expect`, não confunda com `except` do `Python`, apesar de ter uma lógica semelhante. Ao receber um input de um usuário, todo tipo de patifaria pode acontecer, o compilador sabe, nós sabemos, você deve saber também.

Quando recebemos um input, seja qual for, é retornado um `result`, que pode ter a principio duas variações, `ok` quando operação ocorre com sucesso, ou `err` quando a operação falhou. A função `expect` é chamada sempre que `results` retorna `err`, portanto DEVE ser usada para tratar erros de forma elegante. O compilador te avisa, caso não utilize `expect` para tratar um erro. Você pode utilizar `expect` de forma bem simples passando uma mensagem de erro como argumento com `expect('Error...')`.

## Formatando texto com `placeholders`

Neste capítulo também conhecemos os famosos `placeholders`, que são utilizados na macro `println!` para formatação de strings mais complexas, por exemplo, no jogo de adivinhação nós utilizamos o `placeholder`, para demonstrar o valor que o usuário digita, da seguinte maneira:

```rust
println!("You guessed: {}", guess);
```

Nesse exemplo o valor de `guess` será exibido no lugar do `placeholder`, ou seja `{}`. Um outro exemplo legal que o livro trás, é que caso existam mais valores a serem exibidos, podemos passar a variável desejada, dentro do `placeholder`, semelhante ao que fazemos no `Python`, algo como:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

## `Cargo` é amor, `Cargo` é vida

No capítulo 1 fomos apresentados ao `Cargo` o gerenciador de dependencias e agora que sabemos, de `crates` do `Rust`. Também conhecemos o `Cargo.toml`, aquele arquivo bacana que guarda informações do nosso projeto, e tinha uma sessão lá que ainda não haviamos visto, que são as dependencias. Felizmente o `Cargo.lock`não precisa ser gerenciado manualmente e o livro explica o porque disso.

Existe uma forma muito fácil de gerenciar dependencias no `Cargo`, basta adicionar a `crate`desejada no `Cargo.toml` a baixo da sessão `[dependencies]`, da seguinte forma:

```rust
[dependencies]
rand = "0.8.5"
```

Agora vem a mágica, olhe seu `Cargo.lock` e ele não vai ter nada além da sessão `[package]` com o pacote que estamos criando aqui o `guessing_game`. Mas ao adicionar a `crate` `rand` no `Cargo.toml`, podemos rodar `cargo build` e WooW, nós temos download da `crate` e automaticamante o `Cargo.lock` foi configurado. Perceba que muito mais coisa foi baixada além do `rand` isso são dependencias de build, necessárias para que a `crate` funcione.

O mais legal disso tudo, é que caso tenha alguma correção urgente no `rand`, você sabe exatamente de onde vieram essas dependencias, caso exista uma CVE (Common Vulnerabilities and Exposures) que é um tipo de identificador para vulnerabilidades em softwares largamente implementados. Podemos atualizar para a versão que corrige isso, por exemplo, poderiamos atualizar a `crate` `rand` da seguinte maneira:

```rust
[dependencies]
rand = "0.9.0"
```

Agora ao rodar `cargo update`, vemos que além da `rand` todas as dependencias de build também foram atualizadas e o `Cargo.toml` foi atualizado também, isso facilita demais a gestão de dependencias, utilizando ferramentas de SAST (Static Application Security Test) e SCA (Software Composition Analisys) como SonarQube, Veracode, Aikido e Github DependaBot.

Observações: O livro diz que o `cargo` utiliza versionamento semantico `Semantic Versioning` ou `SemVer` o que pode ajudar no versionamento automático de updates menores que não quebram a aplicação, para mais informações vale uma lida na [documentação oficial do SemVer](http://semver.org/).
