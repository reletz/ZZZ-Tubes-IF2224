use std::fmt;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ObjectKind {
    Constant,
    Variable,
    Type,
    Procedure,
    Function,
    Program,
}

impl fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Kita singkat namanya supaya muat di kolom tabel yang kecil
        let s = match self {
            ObjectKind::Constant => "Const",
            ObjectKind::Variable => "Var",
            ObjectKind::Type => "Type",
            ObjectKind::Procedure => "Proc",
            ObjectKind::Function => "Func",
            ObjectKind::Program => "Prog",
        };
        write!(f, "{}", s)
    }
}

pub const TYP_NOTYPE: usize = 0;
pub const TYP_INT: usize = 1;
pub const TYP_REAL: usize = 2;
pub const TYP_BOOL: usize = 3;
pub const TYP_CHAR: usize = 4;
pub const TYP_STRING: usize = 5; 

/// tab
#[derive(Debug, Clone)]
pub struct TabEntry {
    pub name: String,       // 'identifiers'
    pub link: usize,        // 'link': pointer ke identifier sebelumnya (linked list)
    pub obj: ObjectKind,    // 'obj': jenis objek
    pub typ: usize,         // 'type': indeks tipe data di tab ini juga
    pub ref_idx: usize,     // 'ref': pointer ke atab (jika array) atau btab (jika proc/func)
    pub normal: bool,       // 'nrm': true = normal/value, false = reference (var)
    pub level: usize,       // 'lev': scope level (0=global, 1=main, dst)
    pub adr: usize,         // 'adr': offset memori atau value konstanta
}

/// btab
#[derive(Debug, Clone)]
pub struct BTabEntry {
    pub last: usize, // pointer ke identifier terakhir yang dideklarasikan di blok ini
    pub lpar: usize, // pointer ke parameter terakhir
    pub psze: usize, // parameter size (total ukuran parameter)
    pub vsze: usize, // variable size (total ukuran variabel lokal)
}

/// Representasi baris pada tabel 'atab' (Array Table)
#[derive(Debug, Clone)]
pub struct ATabEntry {
    pub xtyp: usize, // index type (misal: Integer)
    pub etyp: usize, // element type (pointer ke tab)
    pub eref: usize, // element ref (jika elemennya komposit/array lagi)
    pub low: i32,    // batas bawah
    pub high: i32,   // batas atas
    pub elsz: usize, // ukuran elemen
    pub size: usize, // total ukuran array
}

#[derive(Debug)]
pub struct SymbolTable {
    pub tab: Vec<TabEntry>,
    pub btab: Vec<BTabEntry>,
    pub atab: Vec<ATabEntry>,

    // Display Stack
    // display[i] menyimpan indeks btab yang aktif di level i
    pub display: Vec<usize>, 
    pub level: usize,        
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut st = SymbolTable {
            tab: Vec::new(),
            btab: Vec::new(),
            atab: Vec::new(),
            display: vec![0; 20], // Max depth 20
            level: 0,
        };
        
        // 1. Buat Blok Global (Universe) - Level 0
        st.make_block(); 
        st.display[0] = 0; 

        // 2. Inisialisasi Tipe Primitif (Index 1-5)
        st.tab.push(TabEntry {
            name: "".to_string(), link: 0, obj: ObjectKind::Type, typ: TYP_NOTYPE, 
            ref_idx: 0, normal: true, level: 0, adr: 0,
        });

        // 3. Reserved Words
        let reserved_words = vec![
            "AND", "ARRAY", "BEGIN", "CASE", "CONST", "DIV", "DOWNTO", "DO", "ELSE", "END",
            "FOR", "FUNCTION", "IF", "MOD", "NOT", "OF", "OR", "PROCEDURE", "PROGRAM",
            "RECORD", "REPEAT", "STRING", "THEN", "TO", "TYPE", "UNTIL", "VAR", "WHILE", "PACKED"
        ];

        // 4. Masukkan reserved words ke tabel
        for kw in reserved_words {
            st.tab.push(TabEntry {
                name: kw.to_string(),
                link: 0,
                obj: ObjectKind::Constant,
                typ: TYP_NOTYPE,
                ref_idx: 0,
                normal: true,
                level: 0,
                adr: 0,
            });
        }

        // 3. Inisialisasi Standard Procedures (writeln, readln, dll)
        st.init_primitives();
        st.init_standard_procedures();

        st
    }

    fn init_primitives(&mut self) {
        // Masukkan tipe dasar secara berurutan (Index 1..5)
        // Fungsi add_system_entry akan otomatis me-link ke btab[0]
        self.add_system_entry("integer", ObjectKind::Type, TYP_INT, 1);
        self.add_system_entry("real", ObjectKind::Type, TYP_REAL, 1);
        self.add_system_entry("boolean", ObjectKind::Type, TYP_BOOL, 1);
        self.add_system_entry("char", ObjectKind::Type, TYP_CHAR, 1);
        self.add_system_entry("string", ObjectKind::Type, TYP_STRING, 1);
    }

    fn init_standard_procedures(&mut self) {
        // Daftarkan writeln dll sebagai Procedure global (Level 0)
        self.add_system_entry("write", ObjectKind::Procedure, TYP_NOTYPE, 0);
        self.add_system_entry("writeln", ObjectKind::Procedure, TYP_NOTYPE, 0);
        self.add_system_entry("read", ObjectKind::Procedure, TYP_NOTYPE, 0);
        self.add_system_entry("readln", ObjectKind::Procedure, TYP_NOTYPE, 0);
    }

    fn add_system_entry(&mut self, name: &str, obj: ObjectKind, typ: usize, size: usize) {
        // Ambil identifier terakhir dari global block (btab[0])
        let last_link = self.btab[0].last;

        let entry = TabEntry {
            name: name.to_string(),
            link: last_link, // Link ke identifier sebelumnya
            obj,
            typ, // Tipe menunjuk diir sendiri sebagai representasi tipe
            ref_idx: 0,
            normal: true,
            level: 0,
            adr: size, // Ukuran tipe (misal 1 word)
        };
        
        self.tab.push(entry);
        let new_idx = self.tab.len() - 1;

        // Update identifier terakhir di global block
        self.btab[0].last = new_idx;
    }

    /// Memasukkan identifier baru ke tabel 'tab'
    pub fn enter(&mut self, name: String, obj: ObjectKind, typ: usize, adr: usize, normal: bool) -> usize {
        // 1. Ambil indeks blok aktif saat ini dari display
        let current_btab_idx = self.display[self.level];
        // 2. Ambil 'last' identifier dari blok tersebut (untuk linked list)
        let last_link = self.btab[current_btab_idx].last;

        // 3. Buat entry baru
        let entry = TabEntry {
            name,
            link: last_link, // Point ke identifier sebelumnya (Linked List terbalik)
            obj,
            typ,
            ref_idx: 0,    // Default 0, diupdate manual nanti jika perlu (misal array/proc)
            normal,
            level: self.level,
            adr,
        };

        self.tab.push(entry);
        let new_idx = self.tab.len() - 1;

        // 4. Update 'last' di btab agar menunjuk ke entry baru ini (Head baru)
        self.btab[current_btab_idx].last = new_idx;

        new_idx
    }

    /// Membuat block baru di 'btab' (Dipanggil saat masuk Prosedur/Fungsi)
    pub fn make_block(&mut self) -> usize {
        let entry = BTabEntry {
            last: 0,
            lpar: 0,
            psze: 0,
            vsze: 0,
        };
        self.btab.push(entry);
        self.btab.len() - 1
    }

    /// Menambahkan array baru ke 'atab'
    pub fn make_array(&mut self, xtyp: usize, etyp: usize, eref: usize, low: i32, high: i32, elsz: usize) -> usize {
        let size = ((high - low + 1) as usize) * elsz;
        let entry = ATabEntry {
            xtyp,
            etyp,
            eref,
            low,
            high,
            elsz,
            size,
        };
        self.atab.push(entry);
        self.atab.len() - 1
    }

    /// Mencari identifier di tabel (Lookup)
    /// Logika: Mulai dari scope level saat ini, mundur sampai level 0.
    /// Di setiap level, telusuri linked list 'link' sampai ketemu atau habis.
    pub fn find(&self, name: &str) -> Option<usize> {
        // Iterasi level dari scope sekarang (self.level) turun ke 0
        for lev in (0..=self.level).rev() {
            let btab_idx = self.display[lev];
            let mut curr_idx = self.btab[btab_idx].last;

            // Telusuri linked list di level ini
            while curr_idx != 0 {
                let entry = &self.tab[curr_idx];
                if entry.name == name {
                    return Some(curr_idx); 
                }
                curr_idx = entry.link;
            }
        }
        None
    }

    pub fn enter_scope(&mut self) {
        self.level += 1;
        if self.level >= self.display.len() {
            self.display.resize(self.level + 1, 0);
        }
        let btab_idx = self.make_block();
        self.display[self.level] = btab_idx;
    }

    pub fn exit_scope(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }
    pub fn print_tables(&self) {
        println!("\n>> Symbol Table Dump");
        
        // 1. Tabel TAB (Identifiers)
        println!("\n{:-^95}", " TAB (Identifiers) ");
        println!("| {:<5} | {:<15} | {:<5} | {:<6} | {:<5} | {:<5} | {:<5} | {:<5} | {:<5} |", 
            "Idx", "Name", "Link", "Obj", "Typ", "Ref", "Nrm", "Lev", "Adr");
        println!("|{:-<93}|", "");

        for (i, entry) in self.tab.iter().enumerate() {
            if i == 0 { continue; } 

            // Format adr based on entry type
            let adr_display = match entry.obj {
                ObjectKind::Constant => {
                    // Constants: show signed value
                    format!("{}", entry.adr as u32 as i32)
                },
                ObjectKind::Type if entry.ref_idx == 0 && (entry.typ == TYP_INT || entry.typ == TYP_CHAR) => {
                    // Subrange type: unpack and show bounds
                    let low = (entry.adr & 0xFFFF) as u16 as i16 as i32;
                    let high = ((entry.adr >> 16) & 0xFFFF) as u16 as i16 as i32;
                    if entry.adr != 0 {
                        format!("{}..{}", low, high)
                    } else {
                        "0".to_string()
                    }
                },
                _ => {
                    // Everything else: show as-is
                    format!("{}", entry.adr)
                }
            };
            
            println!("| {:<5} | {:<15} | {:<5} | {:<6} | {:<5} | {:<5} | {:<5} | {:<5} | {:<5} |",
                i,
                if entry.name.is_empty() { "<empty>" } else { &entry.name },
                entry.link,
                entry.obj,
                entry.typ,
                entry.ref_idx,
                if entry.normal { 1 } else { 0 },
                entry.level,
                adr_display
            );
        }
        println!("{:-^95}", "");

        // 2. Tabel BTAB (Blocks)
        println!("\n{:-^55}", " BTAB (Blocks) ");
        println!("| {:<5} | {:<5} | {:<5} | {:<5} | {:<5} |", 
            "Idx", "Last", "LPar", "PSze", "VSze");
        println!("|{:-<53}|", "");

        for (i, entry) in self.btab.iter().enumerate() {
            println!("| {:<5} | {:<5} | {:<5} | {:<5} | {:<5} |",
                i, entry.last, entry.lpar, entry.psze, entry.vsze
            );
        }
        println!("{:-^55}", "");

        // 3. Tabel ATAB (Arrays)
        if !self.atab.is_empty() {
            println!("\n{:-^85}", " ATAB (Arrays) ");
            println!("| {:<5} | {:<5} | {:<5} | {:<5} | {:<8} | {:<8} | {:<5} | {:<5} |", 
                "Idx", "XTyp", "ETyp", "ERef", "Low", "High", "ElSz", "Size");
            println!("|{:-<83}|", "");

            for (i, entry) in self.atab.iter().enumerate() {
                 // Convert i32 to string to handle potential negative bounds nicely
                println!("| {:<5} | {:<5} | {:<5} | {:<5} | {:<8} | {:<8} | {:<5} | {:<5} |",
                    i + 1, // ATAB biasanya 1-based index di referensi Pascal-S
                    entry.xtyp, entry.etyp, entry.eref, entry.low, entry.high, entry.elsz, entry.size
                );
            }
            println!("{:-^85}", "");
        } else {
            println!("\n[ATAB is empty]");
        }
    }

    /// Nyari identifier hanya di dalam scope (gk sampe parent)
    pub fn find_in_current_scope(&self, name: &str) -> Option<usize> {
        let current_btab = self.display[self.level];
        let mut idx = self.btab[current_btab].last;
        
        while idx != 0 {
            if self.tab[idx].name.to_lowercase() == name.to_lowercase() {
                return Some(idx);
            }
            idx = self.tab[idx].link;
        }
        
        None
    }
}