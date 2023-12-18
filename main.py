#TODO:
# fn that generates a random board and solves it
# add dfs algo
# fn that uses another a* algo
# add "zero" field to the class instead of searching for it every time
# make the board flexible to take any no. of rows and cols and adjust the goal list based on that
# take user input to form a board and check if it is solvable
# print the path in a better way
# simple ui using pygame or tkinter or on web
# playable board
# fn to add and undo move to pass the same board instead of deep copying it
from collections import deque
import copy

class BoardState:
    def __init__(self, board, parent=None):
        self.board = board
        self.parent = parent

def get_neighbors(board, m, n):
    neighbors = []
    for i in range(m):
        for j in range(n):
            if board[i][j] == 0:
                for di, dj in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                    ni, nj = i + di, j + dj

                    if 0 <= ni < m and 0 <= nj < n:
                        new_board = copy.deepcopy(board)
                        new_board[i][j] = new_board[ni][nj]
                        new_board[ni][nj] = 0
                        neighbors.append(new_board)

    return neighbors

def sliding_puzzle_bfs(start):
    m, n = len(start), len(start[0])
    goal = [[1, 2, 3], [4, 5, 6], [7, 8, 0]]

    if start == goal:
        return 0

    start_state = BoardState(start)
    q = deque([start_state])
    visited = set([tuple(map(tuple, start))])

    lvl = 0
    while q:
        size = len(q)
        lvl += 1
        for _ in range(size):
            current_state = q.popleft()

            for neighbor in get_neighbors(current_state.board, m, n):
                if neighbor == goal:
                    print_path(neighbor, current_state)
                    return lvl

                if tuple(map(tuple, neighbor)) not in visited:
                    visited.add(tuple(map(tuple, neighbor)))
                    q.append(BoardState(neighbor, current_state))
    return None

def print_path(current_board, final_state):
    path = []
    lvl = -1
    while final_state is not None:
        path.append(final_state.board)
        final_state = final_state.parent

    for board in reversed(path):
        lvl += 1
        print_board(board, lvl)
        print()

    print_board(current_board, lvl)

def print_board(board, lvl):
    print(lvl)
    for row in board:
        print(row)
    print()

# Example usage
start_board = [[1, 8, 3], [6, 4, 7], [5, 2, 0]]
result = sliding_puzzle_bfs(start_board)
print(result)
