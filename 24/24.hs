
import Data.List


result = (sort $ permutations "0123456789") !! (1000000 - 1)

main = print result
