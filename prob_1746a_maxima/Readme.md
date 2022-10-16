# A. Maxmina (Codeforces Global Round 23)
- time limit per test:  1 second
- memory limit per test 256 megabytes
- input standard input
- output standard output

You have an array $a$ of size $n$ consisting only of zeroes and ones and an integer $k$. In one operation you can do one of the following:
- Select 2 consecutive elements of a and replace them with their minimum (that is, let $a:=[a_1,a_2,…,a_{i−1},min(a_i,a_{i+1}),a_{i+2},…,a_n]$ for some $1\le i\le n−1$). This operation decreases the size of a by 1.
- Select $k$ consecutive elements of $a$ and replace them with their maximum (that is, let $a:=[a_1,a_2,…,a_{i−1},max(a_i,a_{i+1},…,a_{i+k−1}),a_{i+k},…,a_n]$ for some $1 \le i \le n−k+1$). This operation decreases the size of a by $k−1$. 

Determine if it's possible to turn $a$ into $[1]$ after several (possibly zero) operations.

## Input
Each test contains multiple test cases. The first line contains the number of test cases $t$ $(1 \le t \le 1000)$. The description of the test cases follows.

The first line of each test case contains two integers $n$ and $k$ $(2 \le k \le n \le 50)$, the size of array $a$ and the length of segments that you can perform second type operation on.

The second line contains $n$ integers $a_1,a_2,…,a_n$ ($a_i$ is $0$ or $1$), elements of array $a$.

# Output
For each test case, if it is possible to turn a into $[1]$, print "YES", otherwise print "NO".