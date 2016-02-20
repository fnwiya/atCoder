#!/usr/bin/env python
# -*- coding: utf-8 -*-
a, b = map(int, input().split())
tmp = a
a = b
b = tmp
print(a + " " + b)
