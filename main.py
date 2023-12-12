from queue import PriorityQueue
from collections import deque

def bfs(graph, start, goal):
    queue = [start]
    visited = set()

    while queue:
        path = queue.pop(0)
        node = path[-1]

        if node == goal:
            print(f"found path {path}")
            return

        visited.add(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                neighbor_path = path + neighbor
                queue.append(neighbor_path)

    print(f"goal was not found")

def dfs(graph, start, goal):
    stack = [start]
    visited= set()

    while stack:
        # pop() == pop(-1)
        path = stack.pop()
        node = path[-1]

        if node == goal:
            print(f"found path {path}")
            return

        visited.add(start)

        for neighbor in reversed(graph[node]):
            if neighbor not in visited:
                neighbor_path = path + neighbor
                stack.append(neighbor_path)

    print(f"goal was not found")

def uniform_cost(graph, start, goal):
    queue = PriorityQueue()
    queue.put((0, start))
    visited = set()

    while not queue.empty():
        cost, path = queue.get()
        node = path[-1]

        if node == goal:
            print(f"found path {path} with cost {cost}")
            return

        visited.add(node)

        # .items() to get neighbor and cost
        for neighbor, neighbor_cost in graph[node].items():
            if neighbor not in visited:
                neighbor_path = path + neighbor
                queue.put((cost + neighbor_cost, neighbor_path))

    print(f"goal was not found")

def greedy(graph, start, goal, h_cost):
    priority_queue = PriorityQueue()
    priority_queue.put((h_cost[start], start))
    visited = set()

    while priority_queue:
        node_h_cost, path = priority_queue.get()
        # _, path = priority_queue.get() or path = priority_queue.get()[1]  #h_cost only used for sorting

        node = path[-1]

        if node == goal:
            print(f"found path {path}")
            return

        visited.add(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                neighbor_path = path + neighbor
                priority_queue.put((h_cost[neighbor], neighbor_path))

    print(f"goal was not found")

def a_star(graph, start, goal, h_cost):
    priority_queue = PriorityQueue()
    total_cost = 0 + h_cost[start]
    priority_queue.put((total_cost, start, 0))
    visited = set()

    while priority_queue:
        total_cost, path, g_cost = priority_queue.get()

        node = path[-1]

        if node == goal:
            print(f"found path {path} with cost {total_cost}")
            return

        visited.add(node)

        for neighbor, neighbor_cost in graph[node].items():
            if neighbor not in visited:
                neighbor_path = path + neighbor
                priority_queue.put((neighbor_cost + g_cost + h_cost[neighbor], neighbor_path, neighbor_cost + g_cost))

    print(f"goal was not found")

def hill_climb(graph, start, goal, h_cost):
    # takes the first node with h_cost smaller than parent
    queue = [(h_cost[start], start)] 

    visited = set()

    while queue:
        _, path = queue.pop()

        node = path[-1]

        if node == goal:
            print(f"found path {path}")
            return

        visited.add(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                if h_cost[neighbor] < h_cost[node]:
                    neighbor_path = path + neighbor
                    queue.append((h_cost[neighbor], neighbor_path))
                    break

    print(f"goal was not found")

# other not important funcions
def bfs_path_list(graph, start, goal):
    # if start not in graph or goal not in graph:
    #     print("Start or end node not in the graph.")
    #     return
    visited = set()
    # visited = []
    queue = deque(start)

    while queue:
        path = queue.popleft()
        #last node in the path to check if it is the goal and to reach it's neighbors
        node = path[-1]

        if node == goal:
            print(f"found path {path}")
            return

        visited.add(node)
        # visited.append(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                # neighbor_path as a string
                # neighbor_path = path + neighbor
                # neighbor_path as a list
                neighbor_path = list(path)
                neighbor_path.append(neighbor)
                # 'list(path)' and not '[path]' to have a copy not a reference 
                queue.append(neighbor_path)
    print("path not found")

def bfs_multiple_goals(graph, start, goals):
    queue = deque(start)
    visited = set()

    while queue:
        path = queue.popleft()
        print(f"path={path}")
        node = path[-1]
        print(f"node={node}")

        # if node == '1' or node == '2' or node == goal:
        if node in goals:
            print(f"found path {path}")
            break

        visited.add(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                print(f"neighbor={neighbor}")
                neighbor_path = list(path)
                print(f"neighbor_path={neighbor_path}")
                neighbor_path.append(neighbor)
                queue.append(neighbor_path)

def bfs_deque(graph, start, goal):
    queue = deque(start)
    visited = set()

    while queue:
        path = queue.popleft()
        node = path[-1]

        if node == goal:
            print(f"found {path}")
            break

        visited.add(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                neighbor_path = path + neighbor
                queue.append(neighbor_path)

def dfs_recursive(graph, start, goal):
    visited = set()

    goal_found = False

    def _dfs_recursive(path):
        nonlocal goal_found

        if goal_found:
            return

        node = path[-1]

        if node == goal:
            goal_found = True
            print(f"found path {path}")
            return

        visited.add(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                neighbor_path = path + neighbor
                _dfs_recursive(neighbor_path)

    _dfs_recursive(start)

def uniform_cost_list(graph, start, goal):
    priority_queue_list = [(0, start)]
    visited = set()

    while priority_queue_list:
        priority_queue_list.sort()
        cost, path = priority_queue_list.pop(0)
        node = path[-1]

        if node == goal :
            print(f"Path found: {path}, Cost: {cost}")
            break

        visited.add(node)

        for neighbor, neighbor_cost in graph[node].items():
            if neighbor not in visited:
                neighbor_path = path + neighbor
                priority_queue_list.append((cost + neighbor_cost, neighbor_path))

import random
def hill_climb_random(graph, start, goal, h_cost):
    # takes the first node with h_cost smaller than parent
    first_time = True
    while True:
        current_node = start if first_time else random.choice(list(graph.keys()))
        first_time = False
        print(f"Trying from {current_node}")

        queue = [(h_cost[current_node], current_node)]
        visited = set()

        while queue:
            _, path = queue.pop()

            node = path[-1]

            if node == goal:
                print(f"found path {path}")
                return

            visited.add(node)

            for neighbor in graph[node]:
                if neighbor not in visited:
                    if h_cost[neighbor] < h_cost[node]:
                        neighbor_path = path + neighbor
                        queue.append((h_cost[neighbor], neighbor_path))
                        break

        print("Goal was not found in the current random start node")


graph = {
    'S': {'B': 3, 'C': 2},
    'B': {'D': 4, 'E': 1},
    'C': {'B': 1, 'F': 5},
    'D': {'E': 2, 'G': 7},
    'E': {'G': 1},
    'F': {'G': 2},
    'G': {}
}

h_costs = {
    'S': 8,
    'B': 6,
    'C': 4,
    'D': 2,
    'E': 2,
    'F': 1,
    'G': 0
}

graph_2 = {
    'S': {'A': 2, 'C': 3},
    'A': {'B': 1, 'F': 8},
    'B': {'C': 1, 'D': 1, 'G1': 4},
    'C': {'D': 1, 'G2': 5},
    'D': {'G1': 5, 'G2': 2},
    'F': {'G1': 9, 'G2': 7},
    'G1': {},
    'G2': {}
}

h_costs_2 = {
    'S': 5,
    'A': 2,
    'B': 1,
    'C': 3,
    'D': 1,
    'F': 6,
    'G1': 0,
    'G2': 0
}


bfs(graph, 'S', 'G')
dfs(graph, 'S', 'G')
dfs_recursive(graph, 'S', 'G')
uniform_cost(graph, 'S', 'G')
greedy(graph, 'S', 'G', h_costs)
a_star(graph, 'S', 'G', h_costs)
hill_climb(graph, 'S', 'G', h_costs)
bfs_path_list(graph, 'S', 'G')




# general template to use
# def search_algorithm(graph, start, goal):
#     initialize data structures (e.g., queue, stack, priority queue)
#     add start node to data structure
#     initialize visited set to track visited nodes
#
#     while data structure is not empty:
#         node = remove node from data structure
#
#         if node == goal:
#             found path to goal
#             break
#
#         visited.add(node)
#
#         for neighbor, neighbor_cost in graph[node].items():
#             if neighbor not in visited:
#                 visited.add(neighbor)
#                 neighbor_path = path + neighbor
#                 add neighbor_path and (accumlated cost or just heuristic cost or both) to data structure

#                     or

#         for neighbor in graph[node]:
#             if neighbor not in visited:
#                 visited.add(neighbor)
#                 neighbor_path = path + neighbor
#                 add neighbor_path to data structure
#
#     path to goal not found

