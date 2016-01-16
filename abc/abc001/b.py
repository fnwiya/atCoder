#!/usr/bin/python
# -*- coding: utf-8 -*-
# 594080
m = int(input())
if m < 100:
    print("00")
elif 100 <= m <= 5000:
    print("{0:02d}".format(int(m/100)))
elif 6000 <= m <= 30000:
    print(50+(m//1000))
elif 35000 <= m <= 70000:
    print(int(80 + (m//1000 - 30)/5))
else:
    print(89)
