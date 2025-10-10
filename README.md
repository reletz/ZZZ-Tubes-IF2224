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
├── src/                   # Source code utama (Rust)
│   ├── main.rs            # Main entry point
│   ├── lexer/             # Lexical analyzer
│   ├── parser/            # Syntax analyzer  
│   ├── semantic_analyzer/ # Semantic analyzer
│   ├── code_generator/    # Code generator
│   └── utils/             # Utility functions
├── Cargo.toml             # Rust project configuration
├── test/                  # Test cases
│   ├── milestone-1/       # Test untuk lexer
│   ├── milestone-2/       # Test untuk parser
│   └── milestone-3/       # Test untuk semantic & codegen
├── config/                # Configuration files
├── doc/                   # Dokumentasi dan laporan
├── examples/              # Contoh program Pascal-S
└── scripts/               # Script helper
```

## Pembagian Tugas
| Nama | NIM | Tugas |
|------|-----|-------|
| Ahmad Syafiq | 13523135 | |
| Frederiko Eldad Mugiyono | 13523147 | |
| Naufarrel Zhafif Abhista | 13523149 | |
| I Made Wiweka Putera | 13523156 | |
| Hasri Fayadh Muqaffa | 13523160 | |

## Milestone Progress
- [x] Project Structure Setup
- [ ] **Milestone 1**: Lexer Implementation (Deadline: 19 Oktober 2025)
- [ ] **Milestone 2**: Parser Implementation 
- [ ] **Milestone 3**: Semantic Analysis
- [ ] **Milestone 4**: Intermediate Code Generation
- [ ] **Milestone 5**: Interpreter

## Links
- **Repository**: https://github.com/username/ZZZ-Tubes-IF2224
- **Release**: https://github.com/username/ZZZ-Tubes-IF2224/releases
- **DFA Diagram Workspace**: (belum ada hehe)