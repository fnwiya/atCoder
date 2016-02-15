#!/usr/bin/env python
# -*- coding: utf-8 -*-
N = int(input())
NG1 = int(input())
NG2 = int(input())
NG3 = int(input())


def subtract123(N):
    count = 0
    while count < 100:
        if N-3 in [NG1, NG2, NG3]:
            if N-2 in [NG1, NG2, NG3]:
                if N-1 in [NG1, NG2, NG3]:
                    return False
                else:
                    N -= 1
            else:
                N -= 2
        else:
            N -= 3
        count += 1
    if N > 0:
        return False
    else:
        return True

if N in [NG1, NG2, NG3]:
    print("NO")
else:
    if subtract123(N):
        print("YES")
    else:
        print("NO")
