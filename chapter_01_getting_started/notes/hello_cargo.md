# 1.3. Hello, Cargo!

Nesta aula somos apresentados ao `Cargo`, o gerenciador de pacotes do Rust. Semelhante ao NPM do Node ou ao Pip do Python.
Com o `Cargo` podemos não só instalar pacotes de terceiros, mas também somos capazes de gerenciar dependencias do nossos pacotes de uma forma bem parecida com o que fazemos na linguagem `Go`.

Ao utilizarmos o comando `cargo new hello_cargo`, nós criamos um novo pacote, com isso o `Cargo` cria um novo diretório com o nome do projeto, `hello_cargo/`, neste caso e dentro dele temos o `Cargo.toml`,
contendo diversas informações pertinentes de nosso pacote, tais como nome, versão do pacote e edição da linguagem Rust utilizada para gerar o pacote.

Dentro do `Cargo.toml` também podemos listar as dependencias do pacote, bem semelhante ao que fazemos em um `requirements.txt` em projetos Python. Por padrão o `Cargo.toml` também cria um diretório `src/`
dentro do mesmo diretório do pacote, o diretório `src/` armazena todo o código fonte do pacote Rust que você criou, por padrão um arquivo `main.rs` é criado, contendo uma macro `Println!` para escrever `Hello, World!`.
Eu particularmente achei isso bem legal e intuitivo, fazendo com que você desde o começo saiba como organizar minimamente o seu código. O livro deixa claro que o objetivo disto é que você entenda que todo o código do projeto deve ficar
no diretório `src/` enquanto o diretório superior, deve conter o `Cargo.toml`, `README.md`, `LICENSE` e outros recursos de suporte ou documentação para o pacote, que não sejam código Rust.

Logo após explicar sobre o que é o `Cargo` e o `Cargo.toml` e como ele pode ser usado para gerenciar pacotes, o livro nos ensina como buildar nosso primeiro pacote com o `Cargo`, utilizando o comando `cargo build`.

