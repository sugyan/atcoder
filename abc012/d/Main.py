import heapq
from collections import defaultdict

n, m = map(int, input().split())
graph = defaultdict(list)
for _ in range(int(m)):
    a, b, t = [int(x) for x in input().split()]
    graph[a - 1].append((t, b - 1))
    graph[b - 1].append((t, a - 1))


def max_dist(start: int) -> int:
    dists = [float("inf")] * n
    dists[start] = 0
    candidates = [(0, start)]
    while candidates:
        mind, mini = heapq.heappop(candidates)
        if mind > dists[mini]:
            continue
        for d, i in graph[mini]:
            if mind + d < dists[i]:
                dists[i] = mind + d
                heapq.heappush(candidates, (mind + d, i))
    return int(max(dists))


print(min([max_dist(i) for i in range(n)]))
