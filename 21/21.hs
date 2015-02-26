
import Data.List
import qualified Data.Set as Set

properDivisors x = [a | a <- [1..x-1], x `mod` a == 0]

d x = sum $ properDivisors x

isAmicable a =
    let
        da = d a
        db = d da
    in db == a && da /= a

check acc x =
    if not (Set.member x acc) && isAmicable x then
        let
            s = Set.insert x acc
        in Set.insert (d x) s
    else
        acc

amicables = Set.toList $ foldl' check Set.empty [1..9999]

result = sum amicables

main = print result
