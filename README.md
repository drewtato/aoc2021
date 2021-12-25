# Advent of Code 2021

These are my solutions for Advent of Code 2021, this time in Rust.

These run in under 3 seconds on my machine, when run sequentially and piped input from input files. Days 19, 20, and 25 are slowest. 20 and 25 are because of HashMap, while day 19 is just doing a lot of unnecessary computation. Besides those (probably about 2 seconds), the time is mostly overhead for piping the inputs. Most of the actual solutions run in about 10ms.

## Stats

```text
      -------Part 1--------   -------Part 2--------
Day       Time  Rank  Score       Time  Rank  Score
 25   00:23:39  1065      0   00:23:42   856      0
 24   20:08:28  5724      0   20:12:52  5578      0
 23   02:33:01  2384      0   03:54:03  1506      0
 22   00:20:27  1724      0   02:20:03  1410      0
 21   00:19:22  2001      0   01:32:26  2148      0
 20   01:05:06  2323      0   01:05:40  2041      0
 19   02:45:22  1224      0   02:50:06  1110      0
 18   01:58:41  1524      0   02:05:19  1474      0
 17   00:28:17  1748      0   00:51:52  2554      0
 16   01:02:49  1867      0   03:00:03  4535      0
 15   00:17:09   939      0   00:25:14   371      0
 14   00:17:33  2680      0   01:36:02  4716      0
 13   00:24:36  2473      0   00:25:12  1495      0
 12   00:41:39  3770      0   00:46:32  2468      0
 11   00:37:01  3359      0   00:40:16  3199      0
 10   00:10:00  1549      0   00:16:10  1094      0
  9   00:11:27  2126      0   00:25:40  1348      0
  8   00:14:55  3964      0   02:10:07  6554      0
  7   00:09:33  4192      0   00:16:03  3932      0
  6   00:06:46  1440      0   00:19:32  1904      0
  5   00:29:47  4080      0   00:35:33  2646      0
  4   00:35:09  3269      0   00:37:58  2341      0
  3   00:22:15  7826      0   00:46:22  4887      0
  2   00:05:58  4006      0   00:09:22  3717      0
  1   00:02:10   750      0   00:05:11   676      0
```

## Timestamps of each puzzle unlock

 Day    | Timestamp  | Discord code
--------|------------|--------------------
 Day 1  | 1638334800 | `<t:1638334800:R>`
 Day 2  | 1638421200 | `<t:1638421200:R>`
 Day 3  | 1638507600 | `<t:1638507600:R>`
 Day 4  | 1638594000 | `<t:1638594000:R>`
 Day 5  | 1638680400 | `<t:1638680400:R>`
 Day 6  | 1638766800 | `<t:1638766800:R>`
 Day 7  | 1638853200 | `<t:1638853200:R>`
 Day 8  | 1638939600 | `<t:1638939600:R>`
 Day 9  | 1639026000 | `<t:1639026000:R>`
 Day 10 | 1639112400 | `<t:1639112400:R>`
 Day 11 | 1639198800 | `<t:1639198800:R>`
 Day 12 | 1639285200 | `<t:1639285200:R>`
 Day 13 | 1639371600 | `<t:1639371600:R>`
 Day 14 | 1639458000 | `<t:1639458000:R>`
 Day 15 | 1639544400 | `<t:1639544400:R>`
 Day 16 | 1639630800 | `<t:1639630800:R>`
 Day 17 | 1639717200 | `<t:1639717200:R>`
 Day 18 | 1639803600 | `<t:1639803600:R>`
 Day 19 | 1639890000 | `<t:1639890000:R>`
 Day 20 | 1639976400 | `<t:1639976400:R>`
 Day 21 | 1640062800 | `<t:1640062800:R>`
 Day 22 | 1640149200 | `<t:1640149200:R>`
 Day 23 | 1640235600 | `<t:1640235600:R>`
 Day 24 | 1640322000 | `<t:1640322000:R>`
 Day 25 | 1640408400 | `<t:1640408400:R>`

Merry Christmas 🎄🦀
