#!/usr/bin/python
# -*- coding: utf-8 -*-
# 600992
import sys

s = str(input())
t = str(input())


def lose():
    print("You will lose")
    sys.exit()


def check_at(ch):
    if ch != "a" and ch != "t" and ch != "c" and ch != "o" and \
       ch != "d" and ch != "e" and ch != "r" and ch != "@":
        lose()

for i in range(len(s)):
    if s[i] != t[i]:
        if s[i] == "@":
            check_at(t[i])
        elif t[i] == "@":
            check_at(s[i])
        else:
            lose()
print("You can win")
