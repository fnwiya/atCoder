#!/usr/bin/env python
# -*- coding: utf-8 -*-
s = str(input())
ret = s[0].upper()
if len(s) > 1:
    s_lower = s.lower()
    print(ret + s_lower[1:])
else:
    print(ret)
