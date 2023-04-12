"""
[g g g]   [w w w]
[g g g]   [w w w]
[g g g]   [w w w]
                 
          [r r r]
          [r r r]
          [r r r]
                 
          [y y y]
          [y y y]
          [y y y]
                 
          [o o o]   [b b b]
          [o o o]   [b b b]
          [o o o]   [b b b]
"""

"""face_idx:
    0 -> 왼쪽
    1 -> 위쪽
    2 -> 앞쪽
    3 -> 아래쪽
    4 -> 뒤쪽
    5 -> 오른쪽
"""
from copy import deepcopy

class BaseCube:
    def __init__(self) -> None:
        self.cube = [[[color for _ in range(3)] for _ in range(3)] for color in ("g", "w", "r", "y", "o", "b")]
    
    def rotate_clockwise(self, face_idx: int):
        """시계 방향으로 리스트를 90도 회전"""
        self.cube[face_idx] = [[self.cube[face_idx][j][i] for j in range(2, -1, -1)] for i in range(3)]
    
    def get_cube_row(self, face_idx: int, row_idx: int, reverse: bool = False) -> list[str]:
        if reverse:
            return self.cube[face_idx][row_idx][::-1]
        return self.cube[face_idx][row_idx]

    def get_cube_column(self, face_idx: int, column_idx: int, reverse: bool = False) -> list[str]:
        if reverse:
            return [self.cube[face_idx][idx][column_idx] for idx in range(3)][::-1]
        return [self.cube[face_idx][idx][column_idx] for idx in range(3)]
        

class Cube(BaseCube):
    def up_clockwise(self):
        self.rotate_clockwise(1)
        temp_cube = deepcopy(self.cube)
        
        for n, val in enumerate(self.get_cube_row(2, 0)):
            temp_cube[0][n][2] = val
        
        for n, val in enumerate(self.get_cube_column(0, 2, True)):
            temp_cube[4][2][n] = val
        
        for n, val in enumerate(self.get_cube_row(4, 2)):
            temp_cube[5][2][n] = val
        
        for n, val in enumerate(self.get_cube_row(5, 2, True)):
            temp_cube[2][0][n] = val
        
        self.cube = temp_cube
        
    def up_inverted(self):
        for _ in range(3):
            self.up_clockwise()
    

    def down_clockwise(self):
        self.rotate_clockwise(3)
        temp_cube = deepcopy(self.cube)
        
        for n, val in enumerate(self.get_cube_row(5, 0)):
            temp_cube[4][0][n] = val
        
        for n, val in enumerate(self.get_cube_row(4, 0, True)):
            temp_cube[0][n][0] = val
        
        for n, val in enumerate(self.get_cube_column(0, 0)):
            temp_cube[2][2][n] = val
        
        for n, val in enumerate(self.get_cube_row(2, 2, True)):
            temp_cube[5][0][n] = val
        
        self.cube = temp_cube
        
    def down_inverted(self):
        for _ in range(3):
            self.down_clockwise()
    

    def front_clockwise(self):
        self.rotate_clockwise(2)
        temp_cube = deepcopy(self.cube)

        for n, val in enumerate(self.get_cube_row(3, 0, True)):
            temp_cube[0][2][n] = val
        
        for n, val in enumerate(self.get_cube_row(0, 2)):
            temp_cube[1][2][n] = val
        
        for n, val in enumerate(self.get_cube_row(1, 2, True)):
            temp_cube[5][n][2] = val
        
        for n, val in enumerate(self.get_cube_column(5, 2)):
            temp_cube[3][0][n] = val
        
        self.cube = temp_cube
        
    def front_inverted(self):
        for _ in range(3):
            self.front_clockwise()
    

    def bottom_clockwise(self):
        self.rotate_clockwise(4)
        temp_cube = deepcopy(self.cube)
        
        for n, val in enumerate(self.get_cube_row(1, 0)):
            temp_cube[0][0][n] = val
        
        for n, val in enumerate(self.get_cube_row(0, 0, True)):
            temp_cube[3][2][n] = val
        
        for n, val in enumerate(self.get_cube_row(3, 2)):
            temp_cube[5][n][0] = val
        
        for n, val in enumerate(self.get_cube_column(5, 0, True)):
            temp_cube[1][0][n] = val
        
        self.cube = temp_cube
        
    def bottom_inverted(self):
        for _ in range(3):
            self.bottom_clockwise()
    

    def left_clockwise(self):
        self.rotate_clockwise(0)
        temp_cube = deepcopy(self.cube)
        
        for n, val in enumerate(self.get_cube_column(2, 0)):
            temp_cube[3][n][0] = val
        
        for n, val in enumerate(self.get_cube_column(3, 0)):
            temp_cube[4][n][0] = val
        
        for n, val in enumerate(self.get_cube_column(4, 0)):
            temp_cube[1][n][0] = val
        
        for n, val in enumerate(self.get_cube_column(1, 0)):
            temp_cube[2][n][0] = val
        
        self.cube = temp_cube
        
    def left_inverted(self):
        for _ in range(3):
            self.left_clockwise()
    

    def right_clockwise(self):
        self.rotate_clockwise(5)
        temp_cube = deepcopy(self.cube)
        
        for n, val in enumerate(self.get_cube_column(1, 2)):
            temp_cube[4][n][2] = val
        
        for n, val in enumerate(self.get_cube_column(4, 2)):
            temp_cube[3][n][2] = val
        
        for n, val in enumerate(self.get_cube_column(3, 2)):
            temp_cube[2][n][2] = val
        
        for n, val in enumerate(self.get_cube_column(2, 2)):
            temp_cube[1][n][2] = val
        
        self.cube = temp_cube
        
    def right_inverted(self):
        for _ in range(3):
            self.right_clockwise()


n = int(input())

for _ in range(n):
    _ = input()
    cube = Cube()
    
    test_case = input().split()
    for t in test_case:
        match t[0]:
            case "U":
                if t[1] == "+":
                    cube.up_clockwise()
                else:
                    cube.up_inverted()
                
            case "D":
                if t[1] == "+":
                    cube.down_clockwise()
                else:
                    cube.down_inverted()
            
            case "F":
                if t[1] == "+":
                    cube.front_clockwise()
                else:
                    cube.front_inverted()
            
            case "B":
                if t[1] == "+":
                    cube.bottom_clockwise()
                else:
                    cube.bottom_inverted()
            
            case "L":
                if t[1] == "+":
                    cube.left_clockwise()
                else:
                    cube.left_inverted()
            
            case "R":
                if t[1] == "+":
                    cube.right_clockwise()
                else:
                    cube.right_inverted()

    print(*["".join(s) for s in cube.cube[1]], sep="\n")