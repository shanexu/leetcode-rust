#!/usr/bin/env python3

class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        n = len(s)
        i = 1
        l = 0
        lps = [0] * n
        while i < n:
            if s[i] == s[l]:
                l += 1
                lps[i] = l
                i += 1
            else:
                if l == 0:
                    i += 1
                else:
                    l = lps[l - 1]
        if l == 0:
            return False
        else:
            return i % (i - l) == 0
