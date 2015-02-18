

sumofsquares = sum [x * x | x <- [1..100]]

squareofsum = (sum [1..100]) ^ 2

result =  squareofsum - sumofsquares

main = print result
