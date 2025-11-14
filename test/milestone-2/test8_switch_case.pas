program TestKasus;
variabel
    pilihan : char;
mulai
    pilihan := 'A';
    kasus pilihan dari
        'A', 'a' : writeln('Anda memilih A');
        'B', 'b' : writeln('Anda memilih B');
        'C'      : 
        mulai
            writeln('Anda memilih C');
            writeln('Pilihan C spesial');
        selesai;
    selain_itu
        writeln('Pilihan tidak valid');
    selesai;
selesai.