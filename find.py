def traverse(starting_node, n, max_round):
    visited = set()
    visited.add((tuple(starting_node[0]), starting_node[1]))
    stack = []
    neigh = neighbours(starting_node, n, max_round, visited)
    stack.append([neigh, 0])
    i = 0
    while(True):
        top = stack[i]
        if top[1] == len(top[0]):
            stack.pop()
            if i == 0:
                return False
            i -= 1
            continue

        new_current = top[0][top[1]]
        if len(new_current[0]) == 0:
            return True
        new_neigh = neighbours(new_current, n, max_round, visited)
        stack.append([new_neigh, 0])
        top[1] += 1
        i += 1


def neighbours(node, n, max_round, visited):
    out = []
    for i in range(2**n):
        flipped = move(node[0], i)

        next_round = node[1] + 1
        if next_round == max_round:
            flipped = mix_around(flipped, n)
            next_round = 0
        nex = (tuple(flipped), next_round)
        if nex not in visited:
            visited.add(nex)
            out.append(nex)
    return out

def mix_around(possibilities, n):
    out = set()
    for p in possibilities:
        out.add(p)
        for _ in range(n):
            p = ((p << 1) & ((1 << n) - 1)) | (p >> (n-1))
            out.add(p)
    return out

def move(possibilities, flip):
    out = set()
    for p in possibilities:
        out.add(p ^ flip)

    out.discard(0)
    return out

def start(n):
    out = set()
    for i in range(1, 2**n):
        out.add(i)
    return out

for n in range(1, 100):
    for phi in range(1, n+1):
        print("Trying", n, ":", phi)
        if traverse([start(n), 0], n, phi):
            print(n, ":", phi)
            break