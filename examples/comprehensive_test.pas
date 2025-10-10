program TestLexer;

const
  MAX_SIZE = 100;
  PI = 3.14159;

type
  intArray = array [1..MAX_SIZE] of integer;

var
  x, y, z: integer;
  result: real;
  flag: boolean;
  message: char;
  numbers: intArray;

procedure TestProcedure(a, b: integer);
begin
  if a > b then
    writeln('a is greater')
  else
    writeln('b is greater or equal');
end;

function Calculate(x, y: real): real;
begin
  Calculate := x * y + 2.5;
end;

begin
  { Main program }
  x := 10;
  y := 20;
  z := x + y * 2;
  
  if x <> y then
  begin
    for z := 1 to 10 do
      numbers[z] := z * z;
    
    while x <= 100 do
    begin
      x := x + 1;
      if x mod 10 = 0 then
        writeln('x is now ', x);
    end;
  end;
  
  result := Calculate(3.14, 2.0);
  writeln('Result: ', result);
end.