program TestLexer;

konstanta
  MAX_SIZE = 100;
  PI = 3.14159;
  EMPTY = '';

tipe
  intArray = array [123.45..MAX_SIZE] of integer;

variabel
  x, y, z: integer;
  result: real;
  flag: boolean;
  message: char;
  numbers: intArray;

prosedur TestProcedure(a, b: integer);
mulai
  jika a > b maka
    writeln('a lebih besar')
  selain-itu
    writeln('b lebih besar ''atau'' sama');
selesai;

fungsi Calculate(x, y: real): real;
mulai
  Calculate := x * y + 2.5;
selesai;

mulai
  { Program utama }
  x := 1e1;
  y := 2.0e+1;
  z := x + y * 200E-2;
  
  jika x <> y maka
  mulai
    untuk z := 1 ke 10 lakukan
      numbers[z] := z * z;
    
    selama x <= 100 lakukan
    mulai
      x := x + 1;
      jika x mod 10 = 0 maka
        writeln('x sekarang ', x);
    selesai;
  selesai;
  
  result := Calculate(3.14, 2.0);
  writeln('Hasil: ', result);
selesai.