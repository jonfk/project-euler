#lang racket

(define numbers
  (for/list ([i 1001]) i))

(define matching (filter (lambda (x)
                           (or (equal? (remainder x 3) 0) (equal? (remainder x 5) 0)))
                         numbers))
(foldl + 0 matching)
