
var result: int
for i in 1..999:
  if i mod 3 == 0 or i mod 5 == 0:
    result += i

echo(result)
