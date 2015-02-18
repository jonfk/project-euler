
isPythagorean (x,y,z) = x^2 + y^2 == z^2 && x < y && y < z
pythagorean = [(a,b,c) | a <- [1..500], b <- [1..500], c <- [1..500]
                  , isPythagorean(a,b,c)
                  , a + b + c == 1000 ]

myproduct [(x,y,z)] = x * y * z

result = myproduct (take 1 pythagorean)

main = print result
