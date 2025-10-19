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
    - Operator aritmatika (+, -, *, /, div, mod)
    - Operator relasional (=, <>, <, <=, >, >=)
    - Operator boolean (and, or, not)
    - Tipe data (integer, real, boolean, string)
    - Literal konstanta (angka, string dalam quotes)
    - Delimiter dan separator (, ; . () [])
    - Penanganan whitespace dan komentar
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
2. **Syntax Analysis (Parser)** - ???
3. **Semantic Analysis** - ???
4. **Intermediate Code Generation** - ???
5. **Interpreter** - ???

## Requirements

- Rust 1.70+ (with Cargo)
- (Dependencies akan otomatis diinstall melalui Cargo)

## Cara Instalasi

```bash
git clone https://github.com/username/ZZZ-Tubes-IF2224.git
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

### Format Input

File Pascal-S dengan ekstensi `.pas`

### Format Output

List token dalam format `TOKEN_TYPE(value)`

## Struktur Project

```
├── src/                    # Source code utama (Rust)
│   ├── main.rs             # Main entry point
│   ├── lexer/             # Lexical analyzer
│   │   ├── mod.rs         # Lexer module definition
│   │   ├── lexer.rs       # Core lexer implementation
│   │   ├── dfa.rs         # DFA state machine
│   │   └── token_types.rs # Token type definitions
│   ├── parser/            # Syntax analyzer (future)
│   ├── semantic_analyzer/ # Semantic analyzer (future)
│   ├── code_generator/    # Code generator (future)
│   ├── interpreter/       # Interpreter (future)
│   └── utils/            # Utility functions
├── examples/              # Contoh program Pascal-S
│   ├── hello.pas         # Program "Hello World"
│   ├── comment_test.pas  # Test case untuk komentar
│   └── comprehensive_test.pas # Test case komprehensif
├── test/                 # Test cases
│   ├── integration/      # Integration tests
│   ├── milestone-1/      # Lexer test cases
│   │   ├── test1_simple.pas
│   │   ├── test2_operators.pas
│   │   ├── test3_strings_chars.pas
│   │   ├── test4_comments.pas
│   │   ├── test5_arrays_range.pas
│   │   └── expected_output_hello.txt
│   ├── milestone-2/      # Parser tests (future)
│   ├── milestone-3/      # Semantic tests (future)
│   ├── milestone-4/      # Code gen tests (future)
│   └── milestone-5/      # Interpreter tests (future)
├── config/               # Configuration files
│   └── dfa.json         # DFA state transition table
├── doc/                 # Documentation
├── scripts/             # Utility scripts
├── Cargo.toml           # Rust project & dependency config
├── Cargo.lock           # Dependency lock file
├── .gitignore          # Git ignore rules
└── README.md           # Project documentation
```

## Pembagian Tugas

| Nama                     | NIM      | Tugas                                           |
| ------------------------ | -------- | ----------------------------------------------- |
| Ahmad Syafiq             | 13523135 | README & License                                |
| Frederiko Eldad Mugiyono | 13523147 | Test Case dan Pengujian                         |
| Naufarrel Zhafif Abhista | 13523149 | Implementasi DFA (Kode dan Aturannya) dan Lexer |
| I Made Wiweka Putera     | 13523156 | Diagram DFA                                     |
| Hasri Fayadh Muqaffa     | 13523160 | Laporan                                         |

## Milestone Progress

- [x] Project Structure Setup
- [x] **Milestone 1**: Lexer Implementation (Deadline: 19 Oktober 2025)
- [ ] **Milestone 2**: Parser Implementation
- [ ] **Milestone 3**: Semantic Analysis
- [ ] **Milestone 4**: Intermediate Code Generation
- [ ] **Milestone 5**: Interpreter

## Links

- **Repository**: https://github.com/username/ZZZ-Tubes-IF2224
- **Release**: https://github.com/username/ZZZ-Tubes-IF2224/releases
- **DFA Diagram Workspace**: [doc/dfa_diagram.png](doc/dfa_diagram.png)
