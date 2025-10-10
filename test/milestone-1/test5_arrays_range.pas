program ArraysAndRange;
type
  MyArray = array [1..10] of integer;
var
  arr: MyArray;
  i: integer;
begin
  for i := 1 to 10 do
    arr[i] := i;
end.