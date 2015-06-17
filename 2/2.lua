
local function fib(x, y)
   return x + y
end


last1, last2 = 1,2
result = last2

while last2 < 4000000 do
   newlast = fib(last1, last2)
   last1 = last2
   last2 = newlast
   if newlast % 2 == 0 then
   result = newlast + result
   end
end

print(result)
