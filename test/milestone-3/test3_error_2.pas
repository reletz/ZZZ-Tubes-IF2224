program IndexIteratorConst;
konstanta
    MAX_LIMIT = 50;

variabel
    global_counter : integer;

prosedur UbahNilai(var target: integer);
mulai
    target := target + 1;
selesai;

prosedur TestLogic();
variabel
    data : larik [1..5] dari integer;
mulai
    untuk global_counter := 1 ke 3 lakukan
        data[global_counter] := global_counter * 10;

    data[6] := 999;
    data[0] := -1;
selesai;

mulai
    TestLogic();
    UbahNilai(MAX_LIMIT);
selesai.