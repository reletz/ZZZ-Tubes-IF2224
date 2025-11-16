# Pascal-S Compiler

## Identitas Kelompok

- **Nama Kelompok**: JadiApaArtiHidup?
- **Kode Kelompok**: ZZZ
- **Anggota**:
  1. Ahmad Syafiq - 13523135
  2. Frederiko Eldad Mugiyono - 13523147
  3. Naufarrel Zhafif Abhista - 13523149
  4. Hasri Fayadh Muqaffa - 13523156
  5. I Made Wiweka Putera - 13523160

## Deskripsi Program

Compiler Pascal-S adalah implementasi compiler untuk subset bahasa Pascal yang dikembangkan sebagai tugas besar mata kuliah IF2224 Teori Bahasa Formal dan Otomata.

Compiler ini terdiri dari beberapa tahapan:

1. **Lexical Analysis (Lexer)** - Mengubah source code menjadi token

   Lexer diimplementasikan menggunakan Deterministic Finite Automata (DFA) dengan fitur:

   - Tokenisasi keywords Pascal-S (program, var, begin, end, dll)
   - Pengenalan identifier (variabel dan nama fungsi/prosedur)
   - Operator aritmatika (+, -, \*, /, div, mod)
   - Operator relasional (=, <>, <, <=, >, >=)
   - Operator boolean (and, or, not)
   - Tipe data (integer, real, boolean, string)
   - Literal konstanta (angka, string dalam quotes)
   - Delimiter dan separator (, ; . () [])
   - Penanganan whitespace
   - Error handling untuk karakter tidak valid

   Token yang dihasilkan memiliki format:

   ```
   TOKEN_TYPE(value)
   ```

   Contoh:

   ```
   KEYWORD(program)
   IDENTIFIER(main)
   SEMICOLON(;)
   ```

2. **Syntax Analysis (Parser)** - Membangun Concrete Syntax Tree (CST/Parse Tree)

   Parser diimplementasikan menggunakan Recursive Descent Parser dengan fitur:

   - Parsing struktur program Pascal-S (header, deklarasi, body)
   - Deklarasi variabel dengan tipe data
   - Statement parsing (compound statements, expressions)
   - Error handling dengan line/column tracking
   - Parse Tree/CST generation untuk representasi struktural program

   **Parser Call Tree**:

   ```
   main.rs
     └─> parser.parse()
          └─> parse_program()
               ├─> consume_keyword("program")
               ├─> consume_token(Identifier)    # Program name
               ├─> consume_token(Semicolon)
               ├─> parse_declaration_part()
               │    └─> parse_variable_declaration_block()
               │         └─> parse_variable_group()
               │              ├─> consume_token(Identifier)
               │              ├─> consume_token(Colon)
               │              └─> parse_type_spec()
               ├─> parse_compound_statement()
               │    ├─> consume_keyword("mulai")
               │    ├─> parse_statement_list()
               │    │    └─> parse_statement()
               │    │         └─> parse_expression()
               │    │              └─> parse_primary()
               │    └─> consume_keyword("selesai")
               └─> consume_token(Dot)

          Returns: Ok(Program { ... }) or Err(SyntaxError)
   ```

3. **Semantic Analysis** - ???
4. **Intermediate Code Generation** - ???
5. **Interpreter** - ???

## Requirements

- Rust 1.70+ (with Cargo)
- (Dependencies akan otomatis diinstall melalui Cargo)

## Cara Instalasi

```bash
git clone https://github.com/reletz/ZZZ-Tubes-IF2224.git
cd ZZZ-Tubes-IF2224
cargo build --release
```

## Cara Penggunaan Program

### Milestone 1 - Lexer

```bash
cargo run -- program.pas
# atau untuk development
cargo run --bin compiler -- input.pas
```

#### Format Input

File Pascal-S dengan ekstensi `.pas`

#### Format Output

List token dalam format `TOKEN_TYPE(value)`

### Milestone 2 - Parser

**Menguji parser dengan output Parse Tree/CST:**

```bash
cargo run -- test/milestone-2/test4_array_func.pas
```

#### **Format Input**

File Pascal-S dengan ekstensi `.pas` menggunakan keyword Bahasa Indonesia.

#### **Format Output**

Parser menghasilkan **dua representasi Parse Tree**:

##### **1. Tree-Style Parse Tree (Default Output)**

```
<program>
├── <program-header>
│   └── KEYWORD(program) IDENTIFIER(TestArrayFunc) ;
├── <declaration-part>
│   └── <var-declaration>: arr : array[...]
├── <compound-statement>
│   └── <statement-list>
│       └── arr[5] := 42
└── DOT(.)
```

##### **2. Raw CST (Debug Format)**

```rust
Program {
    name: "TestArrayFunc",
    declarations: [Variable(...)],
    body: CompoundStatement {
        statements: [Assignment(...)]
    }
}
```

**Untuk menampilkan raw CST**, uncomment di [`main.rs`](src/main.rs):

```rust
println!("{:#?}", parse_tree);
```

## Struktur Project

```
ZZZ-Tubes-IF2224
├── Cargo.lock
├── Cargo.toml
├── config
│   └── dfa.json
├── doc
│   ├── Diagram-1-ZZZ.png
│   ├── Laporan-1-ZZZ.pdf
│   └── Laporan-2-ZZZ.pdf
├── examples
│   ├── comment_test.pas
│   ├── comprehensive_test.pas
│   └── hello.pas
├── LICENSE
├── README.md
├── scripts
├── src
│   ├── code_generator
│   ├── interpreter
│   ├── lexer
│   │   ├── dfa.rs
│   │   ├── lexer.rs
│   │   ├── mod.rs
│   │   └── token_types.rs
│   ├── main.rs
│   ├── parser
│   │   ├── declarations.rs
│   │   ├── error.rs
│   │   ├── expressions.rs
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   ├── parse_tree.rs
│   │   ├── statements.rs
│   │   └── tree_printer.rs
│   ├── semantic_analyzer
│   └── utils
└── test
    ├── integration
    ├── milestone-1
    │   ├── expected_output_hello.txt
    │   ├── test1_simple.pas
    │   ├── test2_operators.pas
    │   ├── test3_strings_chars.pas
    │   ├── test4_comments.pas
    │   ├── test5_arrays_range.pas
    │   ├── test_hello.pas
    │   └── test_testLexer.pas
    ├── milestone-2
    │   ├── expected_output_expression.txt
    │   ├── test1_expression.pas
    │   ├── test2_literals.pas
    │   ├── test3_all_ops.pas
    │   ├── test4_array_func.pas
    │   ├── test5_procedure.pas
    │   ├── test6_error.pas
    │   ├── test7_nested_loop.pas
    │   ├── test8_switch_case.pas
    │   └── test9_edge.pas
    ├── milestone-3
    ├── milestone-4
    └── milestone-5
```

## Pembagian Tugas

### Milestone 1

| Nama                     | NIM      | Tugas                                           |
| ------------------------ | -------- | ----------------------------------------------- |
| Ahmad Syafiq             | 13523135 | README & License                                |
| Frederiko Eldad Mugiyono | 13523147 | Test Case dan Pengujian                         |
| Naufarrel Zhafif Abhista | 13523149 | Implementasi DFA (Kode dan Aturannya) dan Lexer |
| Hasri Fayadh Muqaffa     | 13523160 | Laporan                                         |
| I Made Wiweka Putera     | 13523160 | Diagram DFA                                     |

### Milestone 2

| Nama                     | NIM      | Tugas                                           |
| ------------------------ | -------- | ----------------------------------------------- |
| Ahmad Syafiq             | 13523135 | Declarations                               |
| Frederiko Eldad Mugiyono | 13523147 | Statements                      |
| Naufarrel Zhafif Abhista | 13523149 | Parse Tree (Node), Parser |
| Hasri Fayadh Muqaffa     | 13523156 | Tree Printer                                         |
| I Made Wiweka Putera     | 13523160 | Expressions, Statements                                     |

## Milestone Progress

- [x] Project Structure Setup
- [x] **Milestone 1**: Lexer Implementation (Deadline: 19 Oktober 2025)
- [x] **Milestone 2**: Parser Implementation
- [ ] **Milestone 3**: Semantic Analysis
- [ ] **Milestone 4**: Intermediate Code Generation
- [ ] **Milestone 5**: Interpreter

## Links

- **Repository**: https://github.com/reletz/ZZZ-Tubes-IF2224
- **Release**: https://github.com/reletz/ZZZ-Tubes-IF2224/releases
- **DFA Diagram Workspace**: [doc/dfa_diagram.png](doc/Diagram-1-ZZZ.png)
