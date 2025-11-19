#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ObjectKind {
    Constant,
    Variable,
    Type,
    Procedure,
    Function,
    Program,
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
/// Sesuai Spesifikasi M3 Halaman 9
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
        
        // Inisialisasi Tipe Primitif agar index 1..5 terisi
        st.init_primitives();
        st
    }

    fn init_primitives(&mut self) {
        // Dummy entry 0 (Gak kepake/error)
        self.add_system_entry("", ObjectKind::Type, TYP_NOTYPE, 0); 
        
        // 1. Integer
        self.add_system_entry("integer", ObjectKind::Type, TYP_INT, 1);
        // 2. Real
        self.add_system_entry("real", ObjectKind::Type, TYP_REAL, 1);
        // 3. Boolean
        self.add_system_entry("boolean", ObjectKind::Type, TYP_BOOL, 1);
        // 4. Char
        self.add_system_entry("char", ObjectKind::Type, TYP_CHAR, 1);
        // 5. String (Extension)
        self.add_system_entry("string", ObjectKind::Type, TYP_STRING, 1);
    }

    fn add_system_entry(&mut self, name: &str, obj: ObjectKind, typ: usize, size: usize) {
        let entry = TabEntry {
            name: name.to_string(),
            link: 0,
            obj,
            typ, // Tipe menunjuk diir sendiri sebagai representasi tipe
            ref_idx: 0,
            normal: true,
            level: 0,
            adr: size, // Ukuran tipe (misal 1 word)
        };
        self.tab.push(entry);
    }

    /// Memasukkan identifier baru ke tabel 'tab'
    pub fn enter(&mut self, name: String, obj: ObjectKind, typ: usize, adr: usize) -> usize {
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
            normal: true,  // Default true, diubah nanti jika parameter 'var'
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
                    return Some(curr_idx); // Ketemu!
                }
                // Pindah ke node sebelumnya
                curr_idx = entry.link;
            }
            
            // Cek juga entri ke-0 (biasanya dummy, tapi kadang system types ada di awal)
            // Di implementasi kita, system types ada di index 1..5.
            // Linked list akan berhenti di 0. Kita perlu cek apakah index 1..5 bisa diakses?
            // System types (integer, dll) biasanya global, jadi akan ditemukan saat lev=0.
        }
        
        // Cek Tipe Primitif (Global/System) secara manual jika belum ketemu di link
        // Karena init_primitives link-nya 0, mereka tidak tersambung ke rantai utama
        for i in 1..=5 {
             if i < self.tab.len() && self.tab[i].name == name {
                 return Some(i);
             }
        }

        None
    }

    // Panggil kalau Analyzer masuk proses prosedur/fungsi (masuk scope)
    pub fn enter_scope(&mut self) {
        self.level += 1;

        if self.level >= self.display.len() {
            self.display.resize(self.level + 1, 0);
        }
        
        let btab_idx = self.make_block();
        // Update display stack
        self.display[self.level] = btab_idx;
    }

    // Panggil kalau Analyzer udah selesai proses prosedur/fungsi (keluar scope)
    pub fn exit_scope(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }
}