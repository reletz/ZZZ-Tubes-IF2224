program TestArrayFunc;
variabel
    arr : larik[1..10] dari integer;
    x : integer;
mulai
    arr[5] := 42;
    x := arr[5] + max(10, 20);
    writeln('Result: ', x);
selesai.