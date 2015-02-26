
factorial 1 = 1
factorial n = product [n,n-1..1]

sumStrings xs = foldl sumHelper 0 xs
    where sumHelper acc x =
              let
                  num = read [x] ::Integer
              in
                acc + num

result = sumStrings $ show $ factorial 100

main = print result
