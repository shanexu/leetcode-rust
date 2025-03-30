#!/usr/bin/env python3

from typing import List


def construct_lps(s: str) -> List[int]:
    n = len(s)
    lps = [0] * n
    lps[0] = 0
    l = 0
    i = 1
    while i < n:
        if s[i] == s[l]:
            l += 1
            lps[i] = l
            i += 1
        else:
            if l > 0:
                l -= 1
            else:
                i += 1
    return lps


def search(txt: str, pat: str) -> List[int]:
    lps = construct_lps(pat)
    ans = []
    i = 0
    j = 0
    n = len(txt)
    m = len(pat)
    while i < n:
        if txt[i] == txt[j]:
            i += 1
            j += 1
            if j == m:
                ans.append(i - j)
                j = lps[j - 1]
        else:
            if j == 0:
                i += 1
            else:
                j = lps[j - 1]
    return ans


if __name__ == '__main__':
    print(search("aabaacaadaabaaba", "aaba"))
