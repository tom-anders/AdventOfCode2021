import ast 
import math
import copy

class Node:
    def __init__(self, l, parent = None):
        self.parent = parent

        if type(l) == list:
            self.left = Node(l[0], self)
            self.right = Node(l[1], self)

            self.value = None
        else:
            self.value = l

            self.left = None
            self.right = None
    
    def __repr__(self, level = 0):
        return self.to_list().__repr__()
    
    def to_list(self):
        if self.value != None:
            return self.value
        return [self.left.to_list(), self.right.to_list()]

    def find_left_neighbor(self):
        n = self

        while True:
            parent = n.parent

            if parent.left == n:

                if parent.parent == None:
                    return None

                n = parent
            else:
                break

        n = n.parent.left

        if n.value != None:
            return n

        while True:
            n = n.right

            if n.value != None:
                return n

    def find_right_neighbor(self):
        n = self

        while True:
            parent = n.parent

            if parent.right == n:

                if parent.parent == None:
                    return None

                n = parent
            else:
                break

        n = n.parent.right

        if n.value != None:
            return n

        while True:
            n = n.left

            if n.value != None:
                return n

    def explode(self, level = 0):
        if self.value == None:
            if level == 4 and self.left.value != None and self.right.value != None: # This is a "leaf-pair"
                
                ln = self.find_left_neighbor()
                if ln:
                    ln.value += self.left.value

                rn = self.find_right_neighbor()
                if rn:
                    rn.value += self.right.value
                 
                # Replace this pair with 0
                self.left = None
                self.right = None
                self.value = 0
                
                return True
            else:
                return self.left.explode(level + 1) or self.right.explode(level + 1)
        else:
            return False
    
    def split(self):
        if self.value == None:
            return self.left.split() or self.right.split()

        if self.value >= 10:
            self.left = Node(math.floor(self.value / 2))
            self.left.parent = self

            self.right = Node(math.ceil(self.value / 2))
            self.right.parent = self

            self.value = None

            return True

        return False

    def reduce(self):
        while True:
            if not self.explode() and not self.split():
                break

    def magnitude(self):
        if self.value != None:
            return self.value

        return 3 * self.left.magnitude() + 2 * self.right.magnitude()

def add_numbers(lhs, rhs):
    sum = Node([0, 0])
    sum.left = lhs
    sum.left.parent = sum

    sum.right = rhs
    sum.right.parent = sum
    return sum

numbers = [Node(ast.literal_eval(s)) for s in open('input/18.in').readlines()]

sum = numbers[0]
for n in numbers[1:]:
    sum = add_numbers(sum, n)
    sum.reduce()

print("Part 1:", sum.magnitude())

numbers = [Node(ast.literal_eval(s)) for s in open('input/18.in').readlines()]

max = 0
for i in range(0, len(numbers)):
    for j in range(i+1, len(numbers)):
        for lhs, rhs in [(numbers[i], numbers[j]), (numbers[j], numbers[i])]:
            sum = add_numbers(copy.deepcopy(lhs), copy.deepcopy(rhs))
            sum.reduce()

            mag = sum.magnitude()
            if mag > max:
                max = mag

print("Part 2:", max)
