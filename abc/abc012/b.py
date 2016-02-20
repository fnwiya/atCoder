#!/usr/bin/env python
# -*- coding: utf-8 -*-
n = int(input())
h = n // (60*60)
m = (n % (60*60)) // 60
s = n % 60
print("{0:02d}:{1:02d}:{2:02d}".format(h, m, s))
