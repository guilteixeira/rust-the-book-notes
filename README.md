# Rust: The Book - Anotações e possíveis lágrimas

Este repositório é uma coletânea de anotações e exemplos práticos feitos durante minha leitura do livro [The Rust Programming Language ("The Book")](https://doc.rust-lang.org/book/). Para este foi utilizada a versão 1.86 do Rust.

O objetivo é consolidar meu aprendizado por meio de prática ativa, testes e escrita, com foco em compreender os fundamentos da linguagem Rust e aplicá-los posteriormente na resolução de desafios no repositório [`rust-journey`](https://github.com/guilteixeira/rust-journey).


## Estrutura

Cada diretório representa um capítulo ou tópico do livro, com exemplos, testes e anotações, como por exemplo:

```
chapter_01_getting_started/
│
├── notes/
│   ├── installation.md
│   └── hello_cargo.md
│
└── exercises/
    ├── hello_world/
    │   ├── Cargo.toml
    │   └── src/
    │       └── main.rs
```

Exemplos e arquivos `.rs` terão comentários inline explicando o raciocínio por trás do código, dúvidas levantadas, ou referências externas.

## 📚 Progresso de Leitura

### ✅ Capítulo 1: Getting Started
- [x] 1.1 Installation
- [x] 1.2 Hello, World!
- [x] 1.3 Hello, Cargo!

### ⏳ Capítulo 2: Programming a Guessing Game
- [ ] Jogo da adivinhação

### 📘 Capítulo 3: Common Programming Concepts
- [ ] 3.1 Variables and Mutability
- [ ] 3.2 Data Types
- [ ] 3.3 Functions
- [ ] 3.4 Comments
- [ ] 3.5 Control Flow

### 📘 Capítulo 4: Understanding Ownership
- [ ] 4.1 What is Ownership?
- [ ] 4.2 References and Borrowing
- [ ] 4.3 The Slice Type

### 📘 Capítulo 5: Using Structs to Structure Related Data
- [ ] 5.1 Defining and Instantiating Structs
- [ ] 5.2 An Example Program Using Structs
- [ ] 5.3 Method Syntax

### 📘 Capítulo 6: Enums and Pattern Matching
- [ ] 6.1 Defining an Enum
- [ ] 6.2 The match Control Flow Construct
- [ ] 6.3 Concise Control Flow with if let and let else

### 📘 Capítulo 7: Managing Growing Projects
- [ ] 7.1 Packages and Crates
- [ ] 7.2 Modules and Privacy
- [ ] 7.3 Paths and Module Tree
- [ ] 7.4 The `use` Keyword
- [ ] 7.5 Separating Modules into Files

### 📘 Capítulo 8: Common Collections
- [ ] 8.1 Vectors
- [ ] 8.2 Strings
- [ ] 8.3 Hash Maps

### 📘 Capítulo 9: Error Handling
- [ ] 9.1 Unrecoverable Errors with `panic!`
- [ ] 9.2 Recoverable Errors with `Result`
- [ ] 9.3 To `panic!` or Not to `panic!`

### 📘 Capítulo 10: Generics, Traits, and Lifetimes
- [ ] 10.1 Generic Data Types
- [ ] 10.2 Traits
- [ ] 10.3 Lifetimes

### 📘 Capítulo 11: Writing Automated Tests
- [ ] 11.1 How to Write Tests
- [ ] 11.2 Controlling Tests
- [ ] 11.3 Test Organization

### 📘 Capítulo 12: I/O Project - Command Line Program
- [ ] 12.1 Command Line Arguments
- [ ] 12.2 Reading a File
- [ ] 12.3 Refactoring
- [ ] 12.4 TDD
- [ ] 12.5 Environment Variables
- [ ] 12.6 Writing to Stderr

### 📘 Capítulo 13: Iterators and Closures
- [ ] 13.1 Closures
- [ ] 13.2 Iterators
- [ ] 13.3 Improving the I/O Project
- [ ] 13.4 Performance Comparison

### 📘 Capítulo 14: More About Cargo and Crates.io
- [ ] 14.1 Release Profiles
- [ ] 14.2 Publishing to Crates.io
- [ ] 14.3 Workspaces
- [ ] 14.4 `cargo install`
- [ ] 14.5 Custom Commands

### 📘 Capítulo 15: Smart Pointers
- [ ] 15.1 Box<T>
- [ ] 15.2 Deref Trait
- [ ] 15.3 Drop Trait
- [ ] 15.4 Rc<T>
- [ ] 15.5 RefCell<T> and Interior Mutability
- [ ] 15.6 Reference Cycles

### 📘 Capítulo 16: Fearless Concurrency
- [ ] 16.1 Threads
- [ ] 16.2 Message Passing
- [ ] 16.3 Shared-State Concurrency
- [ ] 16.4 Sync and Send

### 📘 Capítulo 17: Async Programming
- [ ] 17.1 Futures and Async
- [ ] 17.2 Applying Concurrency with Async
- [ ] 17.3 Handling Multiple Futures
- [ ] 17.4 Streams
- [ ] 17.5 Async Traits
- [ ] 17.6 Futures, Tasks, Threads

### 📘 Capítulo 18: Object Oriented Programming
- [ ] 18.1 OO Characteristics
- [ ] 18.2 Trait Objects
- [ ] 18.3 OO Design Patterns

### 📘 Capítulo 19: Patterns and Matching
- [ ] 19.1 Pattern Uses
- [ ] 19.2 Refutability
- [ ] 19.3 Pattern Syntax

### 📘 Capítulo 20: Advanced Features
- [ ] 20.1 Unsafe Rust
- [ ] 20.2 Advanced Traits
- [ ] 20.3 Advanced Types
- [ ] 20.4 Advanced Functions and Closures
- [ ] 20.5 Macros

### 📘 Capítulo 21: Projeto Final — Web Server Multithreaded
- [ ] 21.1 Single-threaded Web Server
- [ ] 21.2 Multithreading
- [ ] 21.3 Graceful Shutdown

### 📘 Capítulo 22: Appendix
- [ ] A - Keywords
- [ ] B - Operators and Symbols
- [ ] C - Derivable Traits
- [ ] D - Development Tools
- [ ] E - Editions
- [ ] F - Translations
- [ ] G - How Rust is Made

## Progresso de exercícios
| Capítulo | Status |  
|---|---|  
| [1. Getting Started](chapter_01_getting_started) | 🧑‍💻 Fazendo |  
| [2. Programming a Guessing Game](chapter_02_programming_a_guessing_game) | ⏳ Próximo |  
| [3. Common Programming Concepts](chapter_03_common_programming_concepts) | ⏳ Próximo |  
| [4. Understanding Ownership](chapter_04_understanding_ownership) | ⏳ Próximo |  
| [5. Using Structs to Structure Related Data](chapter_05_using_structs_to_structure_related_data) | ⏳ Próximo |  
| [6. Enums and Pattern Matching](chapter_06_enums_and_pattern_matching) | ⏳ Próximo |  
| [7. Managing Growing Projects with Packages, Crates, and Modules](chapter_07_managing_growing_projects_with_packages_crates_and_modules) | ⏳ Próximo |  
| [8. Common Collections](chapter_08_common_collections) | ⏳ Próximo |  
| [9. Error Handling](chapter_09_error_handling) | ⏳ Próximo |  
| [10. Generic Types, Traits, and Lifetimes](chapter_10_generic_types_traits_and_lifetimes) | ⏳ Próximo |  
| [11. Writing Automated Tests](chapter_11_writing_automated_tests) | ⏳ Próximo |  
| [12. An I/O Project: Building a Command Line Program](chapter_12_an_io_project_building_a_command_line_program) | ⏳ Próximo |  
| [13. Functional Language Features: Iterators and Closures](chapter_13_functional_language_features_iterators_and_closures) | ⏳ Próximo |  
| [14. More about Cargo and Crates.io](chapter_14_more_about_cargo_and_crates_io) | ⏳ Próximo |  
| [15. Smart Pointers](chapter_15_smart_pointers) | ⏳ Próximo |  
| [16. Fearless Concurrency](chapter_16_fearless_concurrency) | ⏳ Próximo |  
| [17. Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams](chapter_17_fundamentals_of_asynchronous_programming_async_await_futures_and_streams) | ⏳ Próximo |  
| [18. Object Oriented Programming Features of Rust](chapter_18_object_oriented_programming_features_of_rust) | ⏳ Próximo |  
| [19. Patterns and Matching](chapter_19_patterns_and_matching) | ⏳ Próximo |  
| [20. Advanced Features](chapter_20_advanced_features) | ⏳ Próximo |  
| [21. Final Project: Building a Multithreaded Web Server](chapter_21_final_project_building_a_multithreaded_web_server) | ⏳ Próximo |  

---

## Observações

- Todos os arquivos são feitos com fins didáticos e podem conter anotações em português e inglês.
- Este repositório serve como guia pessoal e material de referência futura.

---

> _"Reading without applying is like sharpening a sword you never use."_

---

