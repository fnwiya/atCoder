#!/usr/bin/python
# -*- coding: utf-8 -*-
# 601416
n = int(input())
cards = [1, 2, 3, 4, 5, 6]
for i in range(n % 60):
    cards[i % 5], cards[i % 5 + 1] = cards[i % 5 + 1], cards[i % 5]

print(''.join([str(x) for x in cards]))
