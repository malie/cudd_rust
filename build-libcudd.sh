#!/bin/sh

cd cudd-2.5.1

[[ -f libcudd.so ]] && exit 0;

./setup.sh

find epd/ mtr/ st/ util/ cudd/ -name '*.c' \
    | grep -v saveimage.c \
    | grep -v test-sav.c \
    | grep -v test-res.c \
    | grep -v testcudd.c \
    | grep -v testmtr.c \
    | xargs gcc -g -Iinclude/ -O3 -shared \
	   -fpic -mtune=native \
	   -DHAVE_IEEE_754 -DBSD \
	   -DSIZEOF_VOID_P=8 -DSIZEOF_LONG=8 \
	   -o libcudd.so \
    && cp -v libcudd.so $OUT_DIR
