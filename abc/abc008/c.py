#!/usr/bin/env python
# -*- coding: utf-8 -*-
# 614120
n = int(input())
coins = [int(input()) for _ in range(n)]
exp_val = 0
for i in range(n):
    divisor_cnt = 0
    for j in range(n):
        if coins[i] % coins[j] == 0:
            divisor_cnt += 1
    if divisor_cnt % 2 == 0:
        exp_val += 1/2
    else:
        exp_val += (divisor_cnt+1)/(2*divisor_cnt)

print(exp_val)
