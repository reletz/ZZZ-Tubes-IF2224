use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticErrorKind {
    UndefinedIdentifier(String),    // Variabel 'x' belum dideklarasikan
    RedeclaredIdentifier(String),    // Variabel 'x' dideklarasikan ulang di scope sama
    
    TypeMismatch {
        expected: String,
        found: String,
    },                              // Harap integer, dapat boolean
    InvalidOperation {
        op: String,
        left_type: String,
        right_type: String,
    },                              // Tidak bisa "string" + "integer"
    
    // Kesalahan Fungsi/Prosedur
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
    },                              // Harap 2 argumen, dapat 3
    NotCallable(String),            // Mencoba memanggil variabel bukan fungsi: x()
    
    // Kesalahan Array
    NotArray(String),               // Mencoba akses indeks pada bukan array: x[1]
    IndexTypeMismatch(String),      // Indeks array bukan integer/scalar
    IndexOutOfBounds {              // Mencoba akses indeks yang di luar jangkauan
        index: i32,
        low: i32,
        high: i32
    },
    
    // Kesalahan Konstanta/Loop
    AssignmentToConstant(String),   // Mencoba assign nilai ke konstanta
    InvalidIterator(String),        // Iterator for-loop harus variabel lokal/integer
    
    // Fallback
    GenericError(String),
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub kind: SemanticErrorKind,
    pub line: usize,
    pub column: usize,
}

impl SemanticError {
    pub fn new(kind: SemanticErrorKind, line: usize, column: usize) -> Self {
        SemanticError { kind, line, column }
    }

    pub fn type_mismatch(expected: impl Into<String>, found: impl Into<String>, line: usize, col: usize) -> Self {
        Self::new(
            SemanticErrorKind::TypeMismatch { 
                expected: expected.into(), 
                found: found.into() 
            }, 
            line, 
            col
        )
    }
    
    pub fn undefined(name: impl Into<String>, line: usize, col: usize) -> Self {
        Self::new(SemanticErrorKind::UndefinedIdentifier(name.into()), line, col)
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match &self.kind {
            SemanticErrorKind::UndefinedIdentifier(id) => format!("Identifier '{}' belum dideklarasikan.", id),
            SemanticErrorKind::RedeclaredIdentifier(id) => format!("Identifier '{}' sudah dideklarasikan di scope ini.", id),
            SemanticErrorKind::TypeMismatch { expected, found } => format!("Tipe tidak cocok. Mengharapkan '{}', ditemukan '{}'.", expected, found),
            SemanticErrorKind::InvalidOperation { op, left_type, right_type } => format!("Operasi '{}' tidak valid untuk tipe '{}' dan '{}'.", op, left_type, right_type),
            SemanticErrorKind::ArgumentCountMismatch { expected, found } => format!("Jumlah argumen salah. Mengharapkan {}, ditemukan {}.", expected, found),
            SemanticErrorKind::NotCallable(id) => format!("'{}' bukan fungsi atau prosedur, tidak bisa dipanggil.", id),
            SemanticErrorKind::NotArray(id) => format!("'{}' bukan array, tidak bisa diakses menggunakan indeks.", id),
            SemanticErrorKind::IndexTypeMismatch(t) => format!("Tipe indeks array harus ordinal (integer/char), ditemukan '{}'.", t),
            SemanticErrorKind::IndexOutOfBounds { index, low, high } => format!("Array index {} di luar jangkauan [{}..{}]", index, low, high),
            SemanticErrorKind::AssignmentToConstant(id) => format!("Tidak dapat mengubah nilai konstanta '{}'.", id),
            SemanticErrorKind::InvalidIterator(id) => format!("Iterator '{}' harus berupa variabel lokal bertipe ordinal.", id),
            SemanticErrorKind::GenericError(msg) => msg.clone(),
        };

        write!(f, "SemanticError at {}:{}: {}", self.line, self.column, message)
    }
}