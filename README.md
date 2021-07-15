# p4

> A perfect solver for [Connect Four](https://en.wikipedia.org/wiki/Connect_Four).

## About

`p4` is a perfect solver for a board game "Connect Four" written in Rust, which is ported of the great [connect4](https://github.com/PascalPons/connect4) solver written in C++.
Thank you [Pasal Pons](https://github.com/PascalPons)!

The solver uses [iterative deepening depth-first search](https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search) with
[alpha-beta prunning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning). Details are explained by [Pascal Pons's articles](http://blog.gamesolver.org).

## Usage

The following command starts an interactive interface of the solver.

```
$ cargo run --example p4 --release -- -b data/7x6.book
    Finished release [optimized] target(s) in 0.23s
     Running `target/release/examples/p4 -b data/7x6.book`
>> 1234
  .   .   .   .   .   .   .
  .   .   .   .   .   .   .
  .   .   .   .   .   .   .
  .   .   .   .   .   .   .
  .   .   .   .   .   .   .
  @   O   @   O   .   .   .

score: -1

>>
```

Position's notation and meaning of position's score are described by [this article](http://blog.gamesolver.org/solving-connect-four/02-test-protocol/).

## Copyright

This software is published under AGPL license.

2021 (C) TSUYUSATO "MakeNowJust" Kitsune
