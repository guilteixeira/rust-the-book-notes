# 1.2. Hello, World!

O capítulo 1 do `Rust`, sem dúvida é um dos melhores starts em programação de qualquer livro que eu tenha lido. Ele apresenta com maestria a sintaxe básica da linguagem, além de apresentar parte a parte da `commandline` necessária para execução do seu primeiro `Hello, Worldd`.

> Nesse momento estou revendo o livro, cerca de dois meses depois de ter iniciado, pois tive que fazer uma pausa por diversos motivos que não vem ao caso agora. Em uma segunda lida percebi alguns detalhes que deixei passar na primeira vez, então decidi reescrever essa nota.

Os desenvolvedores foram muito generosos na documentação ao descrever cada passo nos três principais sistemas operacionais da atualidade `OSX`, `Linux` e `Windows`. Ele descreve como executar nosso primeiro programa utilizando o utilitário `rustc`, para compilar e executar o binário do nosso `Hello, World!`. Inclusive indica uma documentação de troubleshooting, caso algo tenha dado errado.

Na seção `Anatomy of a Rust Program`, os desenvolvedores apresentam em detalhes cada parte do código, explicando não só o que é a função `main()`, mas o porque dos parênteses, e o porque das chaves e etc. No entanto, acredito que o mais importante nessa seção seja a menção a diferença entre funções e macros, onde funções necessitam de `()` após o nome, enquanto macros utilizam `!` e `()`. 

Nessa seção fica bem explícito a importância do uso de `;` ao fim de cada linha, no entanto o livro diz que isso é utilizado **na maioria das vezes**, o que significa que podem haver exceções, que iremos ver no futuro.

> Aqui pode parecer que estou tentando bajular o livro em demasia, mas não é esse o intuito, juro. É comum que em livros de programação se apresente a sintaxe de forma detalhada, no entanto é incomum ver essa abordagem desde a primeira seção do primeiro capítulo e acho que isso foi muito bem executado aqui, dando clareza para quem está lendo dos porquês de cada recurso.

O livro faz uma pequena menção ao utilitário `fmt`, que pode ser utilizado através do comando `cargo fmt`, falamos do `Cargo` ainda nesse capítulo, além do `fmt`, existem outros utilitários que o livro nos recomenda a ler através de um dos apêndices do mesmo.


Por fim na última seção do livro `Compiling and Running Are Separate Steps`, vamos nos aprofundar no processo de compilação e execução do nosso primeiro programa `Rust`.  Essa seção destrincha passo a passo, desde a chamada do compilador com `rustc main.rs` até a exibição desses arquivos via `commandline`, é bem comum que livros desse tipo se foquem em um tipo específico de sistema operacional, ou apenas na versão mais atualizada de seus utilitários, no entanto os desenvolvedores fizeram questão de documentar a exibição tanto em `shells` mais modernos como o `bash` e `zsh` que utilizam `ls` para exibir arquivos e diretórios em sistemas `Unix` ou o equivalente a isso no `CMD` do `Windows` utilizando o comando `dir /B`

> A opção /B exibe apenas nomes de arquivos.

Após compilarmos nosso código, além do nosso código fonte `main.rs`, teremos dois novos arquivos sendo esses, o binário para execução `main.exe` em ambientes `Windows`, ou apenas `main` em ambientes `Linux` ou `OSX` e o arquivo de depuração do programa `main.pdb`. Após isso podemos executar nosso programa com `./main` ou `.\main` em ambientes `Windows`.  O livro encerra esta seção do capítulo 1, salientando os leitores das diferenças entre linguagens interpretadas e compiladas, reafirmando que essas são necessidades exclusivas de linguagens compiladas como  `Rust` ou `C`, diferente de linguagens interpretadas como `Javascript` e `Python`. 