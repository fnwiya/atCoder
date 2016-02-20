#!/usr/bin/env python
# -*- coding: utf-8 -*-
n = int(input())
shortage = 2025 - n
if shortage == 1:
    print("1 x 1")
else:
    ans = []
    for i in range(1, 10):
        if shortage % i == 0 and shortage//i < 10:
            ans.append(str(i) + " x " + str(shortage//i))
    for j in range(len(ans)):
        print(ans[j])
