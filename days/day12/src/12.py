#!/usr/bin/env python3

class Node:
    def __init__(self, name):
        self.name = name
        self.nodes = []

    def __repr__(self):
        return f"{self.name} -> {[n.name for n in self.nodes]}"
    
    def is_big(self):
        return self.name.isupper()

    def find_paths_1(self, path = []):
        if self.name == 'end':
            return 1

        next = [n for n in self.nodes if not n.name in path or n.is_big()]

        path.append(self.name)

        return sum([n.find_paths_1(path.copy()) for n in next])

    def find_paths_2(self, allowed_twice, path = []):
        if self.name == 'end':
            return 1

        next_path = path.copy()
        next_path.append(self.name)

        next = []
        for n in self.nodes:
            if n.name == 'end':
                if next_path.count(allowed_twice) == 2:
                    next.append(n)
            elif not n.name in path or n.is_big():
                next.append(n)
            elif n.name == allowed_twice and path.count(allowed_twice) < 2:
                next.append(n)

        return sum([n.find_paths_2(allowed_twice, next_path.copy()) for n in next])


rules = [line.rstrip().split('-') for line in open("input/12.in").readlines()]

nodes = {}

for lhs, rhs in rules:
    if not lhs in nodes:
        nodes[lhs] = Node(lhs);
    if not rhs in nodes:
        nodes[rhs] = Node(rhs);

    nodes[lhs].nodes.append(nodes[rhs])
    nodes[rhs].nodes.append(nodes[lhs])

part1 = nodes['start'].find_paths_1()
print("Part 1:", part1)

part2 = sum([nodes['start'].find_paths_2(n.name) for n in nodes.values() if not n.is_big() and not n.name == 'start' and not n.name == 'end'])

print("Part 2:", part1 + part2)
