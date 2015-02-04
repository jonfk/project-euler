#lang racket

(define numbers
  (for/list ([i 1000]) i))

(define matching (filter (lambda (x)
                           (or (equal? (remainder x 3) 0) (equal? (remainder x 5) 0)))
                         numbers))
(print matching)
(foldl + 0 matching)
