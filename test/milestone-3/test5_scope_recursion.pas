program ScopeRecursion;
konstanta
    LIMIT = 10;

variabel
    global_val : integer;
    counter : integer;
    hasil_fib : integer;

fungsi Fibonacci(n: integer): integer;
mulai
    jika n <= 1 maka
        Fibonacci := n
    selain_itu
        Fibonacci := Fibonacci(n - 1) + Fibonacci(n - 2);
selesai;

prosedur OuterProc(var x: integer);
variabel
    local_outer : integer;

    prosedur InnerProc(y: integer);
    variabel
        global_val : integer;
    mulai
        global_val := 999;
        
        local_outer := y + global_val; 
        
        x := x + local_outer;
    selesai;

mulai
    local_outer := 0;
    InnerProc(5);
selesai;

mulai
    global_val := 100;
    counter := 10;

    hasil_fib := Fibonacci(6);
    writeln('Fibonacci(6) = ', hasil_fib);

    OuterProc(counter);

    jika (counter = 1014) dan (global_val = 100) maka
        writeln('Scope & Shadowing Test: SUKSES')
    selain_itu
        writeln('Scope & Shadowing Test: GAGAL');

selesai.