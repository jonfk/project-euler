#lang racket

(define (fib limit)
  (define (helper sofar limit)
    (if (empty? sofar)
        (helper (list 2 1) limit)
        (if (>= (first sofar) limit)
            (cdr sofar)
            (helper (cons (+ (first sofar) (list-ref sofar 1)) sofar) limit))))
  (reverse (helper '() limit)))

(print (fib 4000000))

(foldl (lambda (x acc)
         (if (equal? (remainder x 2) 0)
             (+ x acc)
             acc))
       0
       (fib 4000000))
