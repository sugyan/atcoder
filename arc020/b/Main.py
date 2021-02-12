n, c = map(int, input().split())
a = [int(input()) - 1 for _ in range(n)]
mincost = n
for i in range(10):
    for j in range(10):
        if i == j:
            continue
        b = [i if k % 2 == 0 else j for k in range(n)]
        mincost = min(mincost, len(list(filter(lambda x: x[0] != x[1], zip(a, b)))))
print(mincost * c)
