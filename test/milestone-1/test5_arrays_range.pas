program ArraysAndRange;
tipe
  MyArray = array [1..10] of integer;
variabel
  arr: MyArray;
  i: integer;
mulai
  untuk i := 1 ke 10 lakukan
    arr[i] := i;
selesai.