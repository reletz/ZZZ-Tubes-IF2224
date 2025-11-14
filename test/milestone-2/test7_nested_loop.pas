program NestedControl;

variabel
    i, j, total : integer;
    is_valid : boolean;

mulai
    total := 0;
    
    untuk i := 10 turun_ke 1 lakukan
    mulai
        j := 1;
        is_valid := benar;

        selama (j <= i) dan is_valid lakukan
        mulai
            jika (i mod j) = 0 maka
            mulai
                total := total + j;
                writeln('Faktor ditemukan: ', j);
            selesai
            selain-itu
                is_valid := salah;
            
            j := j + 1;
        selesai;
    selesai;

    writeln('Total akhir: ', total);
selesai.