# 1.3. Hello, Cargo!

Nesta aula somos apresentados ao `Cargo`, o gerenciador de pacotes do Rust. Semelhante ao NPM do Node ou ao Pip do Python.
Com o `Cargo` podemos não só instalar pacotes de terceiros, mas também somos capazes de gerenciar dependencias do nossos projetos de uma forma bem parecida com o que fazemos na linguagem `Go`.

Ao utilizarmos o comando `cargo new hello_cargo`, nós criamos um novo projeto, com isso o `Cargo` cria um novo diretório com o nome do projeto, `hello_cargo/`, neste caso e dentro dele temos o `Cargo.toml`,
contendo diversas informações pertinentes de nosso projeto, tais como nome, versão do projeto e edição da linguagem Rust utilizada para gerar o projeto.

Dentro do `Cargo.toml` também podemos listar as dependencias do projeto, bem semelhante ao que fazemos em um `requirements.txt` em projetos Python. Por padrão o `Cargo.toml` também cria um diretório `src/`
dentro do mesmo diretório do projeto, o diretório `src/` armazena todo o código fonte do projeto Rust que você criou, por padrão um arquivo `main.rs` é criado, contendo uma macro `println!` para escrever `Hello, World!`.

**Observação:**
> Repare que `println!` possui uma exclamação `!`, isso não é um erro de digitação, acontece que diferente da função `main` que possui `fn` para indicar que é uma função, `println!` é uma macro, acredito que o livro deve falar de macros em breve.

> Lendo novamente o livro vi que na seção Hello, World!, do capitulo 1, ele aborda que as macros serão apresentadas em detalhes no capítulo 20.

Eu particularmente achei isso bem legal e intuitivo, fazendo com que você desde o começo saiba como organizar minimamente o seu código. O livro deixa claro que o objetivo disto é que você entenda que todo o código do projeto deve ficar
no diretório `src/` enquanto o diretório superior, deve conter o `Cargo.toml`, `README.md`, `LICENSE` e outros recursos de suporte ou documentação para o projeto, que não sejam código Rust.

Logo após explicar sobre o que é o `Cargo` e o `Cargo.toml` e como ele pode ser usado para gerenciar projetos, o livro nos ensina como buildar nosso primeiro projeto com o `Cargo`, utilizando o comando `cargo build` na raiz do Projeto. Este comando gera alguns novos arquivos e diretórios como o diretório `target`, dentro da raiz do projeto, dentro deste diretório temos uma pasta `debug` contendo o binário `hello_cargo` que já pode ser executado.

O livro diz que por default todo build do `Cargo` é uma build debug. O `Cargo` também cria um novo arquivo na raiz do projeto chamado `Cargo.lock`, similar a um `package-lock.json` em programas Node.JS, guardando a informação exata de quais são as dependencias utilizadas no projeto e a versão exata delas.

Além do `cargo build`, podemos usar `cargo run` que magicamente executa o nosso código a partir da raiz ou qualquer pasta do Projeto que a gente esteja. Caso você não tenha usado `cargo build` o `cargo run` irá buildar na pasta `target/debug/` da mesma forma e em seguida executar o programa. Portanto, o livro diz que essa é a abordagem mais comum no dia a dia para buildar e rodar um programa em Rust, afinal mesmo que demore um pouco mais, ele já economiza o tempo de executar dois comandos em seguida.

**Lembrete:**

> Para lembrar disso com mais clareza, pense que `build` apenas contrói o pacote/projeto enquanto `run` constrói e roda o projeto.

Por fim o livro nos apresenta um último comando o `cargo check` que deve ser usado quando temos um programa maior, ou fizemos poucos ajustes e só queremos saber se o programa irá compilar, `cargo check` não gera um binário para ser executado.

Antes que o capítulo termina-se o livro nos lembrava novamente que o padrão do comando `cargo build` é gerar um build debug sem otimizações, no entanto, caso tenhamos interesse em compartilhar o binário como uma release, podemos executar `cargo build --release` com essa finalidade, dessa forma o compilador irá fazer todas as otimizações antes de entregar o binário final que será salvo em `target/release`.

O cargo é um gerenciador de pacote muito completo então ele tem um livro próprio que pode ser acessada em: https://doc.rust-lang.org/cargo/index.html.


**Observação:**
>  Estou voltando a esta nota em 15/07/2025 pois precisei me ausentar temporariamente dos estudos e deu pra refazer todos os passos perfeitamente, então acho que está bem anotado.