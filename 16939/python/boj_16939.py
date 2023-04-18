from copy import deepcopy

class BaseCube:
    def __init__(self, colors: list[int]) -> None:
        self.default_cube = [[colors[4*i:4*(i+1)][:2], colors[4*i:4*(i+1)][2:]] for i in range(0, len(colors)//4)]
        self.init_cube()

    def init_cube(self):
        self.cube = deepcopy(self.default_cube)

    def rotate_clockwise(self, face_idx: int):
        self.cube[face_idx] = [
            [self.cube[face_idx][1][0], self.cube[face_idx][0][0]],
            [self.cube[face_idx][1][1], self.cube[face_idx][0][1]]
        ]
    
    def get_row(self, face_idx: int, row_idx: int, reverse: bool = False) -> list[int]:
        if reverse:
            return self.cube[face_idx][row_idx][::-1]
        return self.cube[face_idx][row_idx]
    
    def get_colomn(self, face_idx: int, colomn_idx: int, reverse: bool = False) -> list[int]:
        if reverse:
            return [face[colomn_idx] for face in self.cube[face_idx]][::-1]
        return [face[colomn_idx] for face in self.cube[face_idx]]
    
    def is_same_numbers(self, face_idx: int) -> bool:
        first_num = self.cube[face_idx][0][0]
        for i in range(2):
            for j in self.get_row(face_idx, i):
                if first_num != j:
                    return False
        return True
    
    def is_all_same_numbers(self) -> bool:
        return all(cube.is_same_numbers(i) for i in range(6))

class Cube(BaseCube):
    def up_clockwise(self):
        self.rotate_clockwise(0)

        temp_cube = deepcopy(self.cube)
        
        temp_cube[3][0] = self.get_row(1, 0)
        temp_cube[5][0] = self.get_row(3, 0)
        temp_cube[4][0] = self.get_row(5, 0)
        temp_cube[1][0] = self.get_row(4, 0)
        
        self.cube = temp_cube
    
    def up_invert(self):
        for _ in range(3):
            self.up_clockwise()


    def down_clockwise(self):
        for _ in range(3):
            self.down_invert()
        
        self.rotate_clockwise(2)
        self.rotate_clockwise(2)

    def down_invert(self):
        self.rotate_clockwise(2)
        
        temp_cube = deepcopy(self.cube)
        
        temp_cube[3][1] = self.get_row(1, 1)
        temp_cube[5][1] = self.get_row(3, 1)
        temp_cube[4][1] = self.get_row(5, 1)
        temp_cube[1][1] = self.get_row(4, 1)
        
        self.cube = temp_cube
    
    
    def front_clockwise(self):
        self.rotate_clockwise(1)
        
        temp_cube = deepcopy(self.cube)
        
        temp_cube[4][0][0], \
        temp_cube[4][1][0] = self.get_row(0, 1)
        temp_cube[2][0] = self.get_colomn(4, 0, True)
        temp_cube[3][0][1], \
        temp_cube[3][1][1] = self.get_row(2, 0)
        temp_cube[0][1] = self.get_colomn(3, 1, True)
        
        self.cube = temp_cube
    
    def front_invert(self):
        for _ in range(3):
            self.front_clockwise()


    def bottom_clockwise(self):
        self.rotate_clockwise(5)
        
        temp_cube = deepcopy(self.cube)

        temp_cube[4][0][1], \
        temp_cube[4][1][1] = self.get_row(2, 1, True)
        temp_cube[0][0] = self.get_colomn(4, 1)
        temp_cube[3][0][0], \
        temp_cube[3][1][0] = self.get_row(0, 0, True)
        temp_cube[2][1] = self.get_colomn(3, 0)
        
        self.cube = temp_cube
    
    def bottom_invert(self):
        for _ in range(3):
            self.bottom_clockwise()
    
    
    def left_clockwise(self):
        self.rotate_clockwise(3)

        temp_cube = deepcopy(self.cube)
        
        temp_cube[1][0][0], \
        temp_cube[1][1][0] = self.get_colomn(0, 0)
        temp_cube[2][0][0], \
        temp_cube[2][1][0] = self.get_colomn(1, 0)
        temp_cube[5][0][1], \
        temp_cube[5][1][1] = self.get_colomn(2, 0, True)
        temp_cube[0][0][0], \
        temp_cube[0][1][0] = self.get_colomn(5, 1, True)
        
        self.cube = temp_cube
    
    def left_invert(self):
        for _ in range(3):
            self.left_clockwise()
    
    
    def right_clockwise(self):
        for _ in range(3):
            self.right_invert()
        self.rotate_clockwise(4)
        self.rotate_clockwise(4)

    def right_invert(self):
        self.rotate_clockwise(4)

        temp_cube = deepcopy(self.cube)
        
        temp_cube[1][0][1], \
        temp_cube[1][1][1] = self.get_colomn(0, 1)
        temp_cube[2][0][1], \
        temp_cube[2][1][1] = self.get_colomn(1, 1)
        temp_cube[5][0][0], \
        temp_cube[5][1][0] = self.get_colomn(2, 1, True)
        temp_cube[0][0][1], \
        temp_cube[0][1][1] = self.get_colomn(5, 0, True)
        
        self.cube = temp_cube


cube = Cube(list(map(int, input().split())))

work = [
    cube.up_clockwise,
    cube.up_invert,
    cube.down_clockwise,
    cube.down_invert,
    cube.front_clockwise,
    cube.front_invert,
    cube.left_clockwise,
    cube.left_invert,
    cube.right_clockwise,
    cube.right_invert,
]

for w in work:
    w()
    if cube.is_all_same_numbers():
        print(1)
        break
    
    cube.init_cube()
else:
    print(0)
