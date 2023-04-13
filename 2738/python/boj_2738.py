class Matrix:
    def __init__(self, array: list) -> None:
        self.array = array
    
    def __add__(self, other):
        new_matrix = self.array
        
        for row_idx in range(len(new_matrix)):
            for colomn_idx in range(len(new_matrix[row_idx])):
                new_matrix[row_idx][colomn_idx] += other.array[row_idx][colomn_idx]
        
        return Matrix(new_matrix)
    
    def __repr__(self) -> str:
        temp = []
        for row in self.array:
            temp.append(" ".join(map(str, row)))
        
        return "\n".join(temp)

n, _ = map(int, input().split())
matrix_a = Matrix([list(map(int, input().split())) for _ in range(n)])
matrix_b = Matrix([list(map(int, input().split())) for _ in range(n)])

print(matrix_a + matrix_b)