#!/usr/bin/python
# -*- coding: utf-8 -*-
# 594521
w = input()
output = ""
for char in w:
    if char != "a" and char != "i" and char != "u" and char != "e" and char != "o":
        output += char
print(output)
