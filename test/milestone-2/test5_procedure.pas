program TestDeklarasiAman;

konstanta
    MAKS = 100;
    NAMA = 'Gacor King';

tipe
    Angka = integer;

variabel
    x : Angka;

prosedur Cetak(i: Angka);
variabel
    lokal_prosedur: integer;
mulai
    writeln('Angka: ', i, MAKS, NAMA);
selesai;

fungsi DapatAngka(): Angka;
variabel
    lokal_fungsi: integer;
mulai
    DapatAngka := 99;
selesai;

mulai
    x := DapatAngka();
    Cetak(x);
selesai.