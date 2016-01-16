#!/usr/bin/python
# -*- coding: utf-8 -*-
# 594525
import math
xa, ya, xb, yb, xc, yc = map(int, input().split())
print(math.fabs((xa * yb + xb * yc + xc * ya - ya * xb - yb * xc - yc * xa)/2))
