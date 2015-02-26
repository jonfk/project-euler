
candidates = show (2^1000)

sumStrings acc x =
    let
        num = read [x] ::Integer
    in
      acc + num

result = foldl sumStrings 0 candidates

main = print result
