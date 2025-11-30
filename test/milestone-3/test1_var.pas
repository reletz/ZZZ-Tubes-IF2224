program CekReference;
variabel
  global: integer;

prosedur Tukar(var a: integer; b: integer);
mulai
  a := b;
  b := 0;
selesai;

mulai
  global := 10;
  Tukar(global, 5);
selesai.