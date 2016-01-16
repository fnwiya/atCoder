#!/usr/bin/python
# -*- coding: utf-8 -*-
# 594086
deg, dis = map(int, input().split())

deg /= 10
if deg >= 11.25 and deg < 33.75:
    h = "NNE"
elif deg >= 33.75 and deg < 56.25:
    h = "NE"
elif deg >= 56.25 and deg < 78.75:
    h = "ENE"
elif deg >= 78.75 and deg < 101.25:
    h = "E"
elif deg >= 101.25 and deg < 123.75:
    h = "ESE"
elif deg >= 123.75 and deg < 146.25:
    h = "SE"
elif deg >= 146.25 and deg < 168.75:
    h = "SSE"
elif deg >= 168.75 and deg < 191.25:
    h = "S"
elif deg >= 191.25 and deg < 213.75:
    h = "SSW"
elif deg >= 213.75 and deg < 236.25:
    h = "SW"
elif deg >= 236.25 and deg < 258.75:
    h = "WSW"
elif deg >= 258.75 and deg < 281.25:
    h = "W"
elif deg >= 281.25 and deg < 303.75:
    h = "WNW"
elif deg >= 303.75 and deg < 326.25:
    h = "NW"
elif deg >= 326.25 and deg < 348.75:
    h = "NNW"
else:
    h = "N"

dis = round((dis + 0.5) / 60.0, 1)
if dis <= 0.2:
    wind = "0"
elif dis <= 1.5:
    wind = "1"
elif dis <= 3.3:
    wind = "2"
elif dis <= 5.4:
    wind = "3"
elif dis <= 7.9:
    wind = "4"
elif dis <= 10.7:
    wind = "5"
elif dis <= 13.8:
    wind = "6"
elif dis <= 17.1:
    wind = "7"
elif dis <= 20.7:
    wind = "8"
elif dis <= 24.4:
    wind = "9"
elif dis <= 28.4:
    wind = "10"
elif dis <= 32.6:
    wind = "11"
elif dis >= 32.7:
    wind = "12"


if wind == "0":
    h = "C"

print("%s %s" % (h, wind))
