beginnings of CUDD bindings for Rust,
the ROBDD library from Colorado.
Reduced Ordered Binary Decision Diagrams that is.

http://vlsi.colorado.edu/~fabio/CUDD/

Also, CUDD is built as a dynamic library.

TODO:
* Reference Counting
* more functions
* How to use C macros from Rust? Well, the simple ones :/
* How to make the pointers to DdManager and DdNode opaque?

[![Build Status](https://travis-ci.org/malie/cudd_rust.svg?branch=master)](https://travis-ci.org/malie/cudd_rust)
