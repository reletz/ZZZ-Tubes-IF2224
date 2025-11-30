program TestEdgeCases;
konstanta
    START = -10;
    MAX = START + 20;

tipe
    RealRange = 1.5 .. 10.5;
    CharRange = 'A' .. 'Z';
    
    ExprRange = (START * 2) .. (MAX - 1); 
    
    NestedArray = larik [1..5] dari larik [CharRange] dari boolean;
    
    ComplexArray = larik [ExprRange] dari integer;

variabel
    a, b, c : integer;
    flag1, flag2 : boolean;
    matrix : NestedArray;
    idx_char : char;

fungsi GetIndex(x: integer): integer;
mulai
    GetIndex := x bagi 2;
selesai;

prosedur DoNothing();
mulai
   
selesai;

mulai
    a := -5;
    b := 10 - 5;
    c := 10 - -b;
    
    flag1 := tidak (a < b) dan (c > 0) atau (a = -5); 
    
    idx_char := 'B';
    matrix[GetIndex(a + b)][idx_char] := (a < c) dan flag1;
    
    jika matrix[1]['A'] maka
        writeln('OK')
    selain_itu
        mulai 
        selesai;

    kasus a dari

    selain_itu
        writeln('Nilai A tidak ada di case');
    selesai;

    kasus b dari

    selesai;

    DoNothing();

selesai.