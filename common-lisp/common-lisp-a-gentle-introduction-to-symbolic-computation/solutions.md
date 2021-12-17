# 1.1

- 13
- 3
- /
- 3
- 3

- -48
- 5/3
- 0
- -
- 2/3
- 2

# 1.2

S, I, S, S, I, N, N, S, S, I, I, S

# 1.3

T, NIL, NIL, NIL, T, T, NIL

# 1.4

```
          SUB2
   +----------------+
   |                |
   |         +---+  |
---+-------->|   |  |
   |         | - +--+-->
   |  2 ---->|   |  |
   |         +---+  |
   |                |
   +----------------+
```

# 1.4 - 1.7

(solved on paper)

# 1.8

The function negates its input.

# 1.9 - 1.12

(solved on paper)

# 1.13

T

# 1.14

T, NIL, NIL

# 1.15 - 1.17

(solved on paper)

# 1.18

When input is `-2`.

# 1.19

- NIL → NIL
- T → T
- RUTABAGA → T

# 1.20

(solved on paper)

# 1.21

- Wrong type input.
- Too few inputs.
- Too many inputs.

# 1.22

Yes and no, respectively.

# 1.23

EQUAL, <, >, NOT

# 1.24

No and yes, respectively.

# 1.25

Because it's a symbol different than NIL.

# 1.26

No and yes, respectively.

# 1.27


- (EVENP T)
- (EVENP 2 2)

# 2.1

(solved on paper)

# 2.2

2, 5, 6

# 2.3

(solved on paper)

# 2.4

((BOWS ARROWS) (FLOWERS CHOCOLATES))

# 2.5

6, 3, 4, 4, 5, 6

# 2.6

() => NIL
(()) => (NIL)
((())) => ((NIL))
(() ()) => (NIL NIL)
(() (())) => (NIL (NIL))

# 2.7

(HONK IF YOU LIKE GEESE)
(IF YOU LIKE GEESE)
IF

# 2.8

REST -> REST -> FIRST

# 2.9

REST -> SECOND

# 4.1

```common-lisp
(defun make-even (n) (if (evenp n) n (+ n 1)))
```

# 4.2

```common-lisp
(defun further (n) (if (< n 0) (- n 1) (+ n 1)))
```

# 4.3

```common-lisp
(defun my-not (x) (if x nil t))
```

# 4.4

```common-lisp
(defun ordered (x y) (if (< x y) (list x y) (list y x)))
```

# 4.5

3, 2, 1

# 4.6

```common-lisp
(defun my-abs (x) (cond ((< x 0) (- x)) (t x)))
```

# 4.7

- Incorrect, `(symbolp x)` is not a correct clause.
- Correct.
- Incorrect, `('symbol)` should have no parenthesis.
- Incorrent, `((t 'not-a-symbol))` has too many parenthesis.

# 4.8

```common-lisp
(defun emphasize3 (x)
  (cond ((equal (first x) 'good) (cons 'great (rest x)))
	((equal (first x) 'bad)  (cons 'awful (rest x)))
	(t (cons 'very x))))
```

# 4.9

In the original version the order of test-and-consequent clauses was wrong.
Fixed version:

```common-lisp
(defun make-odd (x)
      (cond ((not (oddp x)) (+ x 1))
            (t x)))
```

# 4.10

Using `COND`:

```common-lisp
(defun constrain (x max min)
  (cond ((< x min) min)
	((> x max) max)
	(t x)))
```

Using nested `IF`s:

```common-lisp
(defun constrain (x max min)
  (if (< x min)
    min
    (if (> x max)
      max
      x)))
```

# 4.11

```common-lisp
(defun firstzero (l)
  (cond ((equal 0 (first l)) 'first)
        ((equal 0 (second l)) 'second)
        ((equal 0 (third l)) 'third)
        (t 'none)))
```

# 4.12

```common-lisp
(defun cycle (x)
  (cond ((equal 99 x) 1)
	(t (+ 1 x))))
```

# 4.13

```common-lisp
(defun howcompute (x y z)
  (cond ((and (equal (+ x y) z) (equal (* x y) z)) 'sum-of-or-product-of)
        ((equal (+ x y) z) 'sum-of)
        ((equal (* x y) z) 'product-of)
        (t '(beats me))))
```

# 4.14

`foe`, `fee`, `foe`, `nil`, `yes`, `t`

# 4.15

```common-lisp
(defun geq (x y)
  (or (equal x y) (> x y)))
```

# 4.16

```common-lisp
(defun fun (x)
  (cond ((and (oddp x) (> x 0)) (* x x))
        ((and (oddp x) (< x 0)) (* x 2))
        (t (/ x 2))))
```

# 4.17

```common-lisp
(defun check (a b)
  (or (and (equal b 'child) (or (equal a 'boy) (equal a 'girl)))
      (and (equal b 'adult) (or (equal a 'man) (equal a 'woman)))))
```
