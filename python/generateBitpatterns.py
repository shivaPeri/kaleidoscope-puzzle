# generate all bitpatterns for all pieces
# writes to output file

mono_1 = [
    [[2]],
    [[1]]
]

mono_2 = [
    [[4]],
    [[1]]
]

domo_1 = [
    [[2,1]],
    [[1,2]],
    [[4,1]],
    [[1,4]],
    [[2],[1]],
    [[1],[2]],
    [[4],[1]],
    [[1],[4]]
]

trom_1 = [
    [[2,1,2]],
    [[3,1,4]],
    [[4,1,3]],
    [[2],[1],[2]],
    [[3],[1],[4]],
    [[4],[1],[3]]
]

trom_2 = [
    [[1,2,1]],
    [[1,3,1]],
    [[1],[2],[1]],
    [[1],[3],[1]]
]

trom_3 = [
    [[0,1],[1,2]],
    [[1,0],[2,1]],
    [[1,2],[0,1]],
    [[2,1],[1,0]],
    [[0,1],[1,3]],
    [[1,0],[3,1]],
    [[1,3],[0,1]],
    [[3,1],[1,0]]
]

trom_4 = [
    [[0,2],[2,1]],
    [[2,0],[1,2]],
    [[2,1],[0,2]],
    [[1,2],[2,0]],
    [[0,4],[3,1]],
    [[3,0],[1,4]],
    [[4,1],[0,3]],
    [[1,3],[4,0]]
]

tetr_1 = [
    [[2,1,2,1]],
    [[1,2,1,2]],
    [[4,1,3,1]],
    [[1,3,1,4]],
    [[2],[1],[2],[1]],
    [[1],[2],[1],[2]],
    [[4],[1],[3],[1]],
    [[1],[3],[1],[4]]
]

tetr_2 = [
    [[2,1],[1,2]],
    [[1,2],[2,1]],
    [[4,1],[1,3]],
    [[3,1],[1,4]],
    [[1,3],[4,1]],
    [[1,4],[3,1]]
]

tetr_3 = [
    [[0,0,2],[1,2,1]],
    [[2,1],[0,2],[0,1]],
    [[1,2,1],[2,0,0]],
    [[1,0],[2,0],[1,2]],
    [[1,0,0],[3,1,4]],
    [[3,1],[1,0],[4,0]],
    [[4,1,3],[0,0,1]],
    [[0,4],[0,1],[1,3]]
]

tetr_4 = [
    [[2,0,0],[1,2,1]],
    [[1,2],[2,0],[1,0]],
    [[1,2,1],[0,0,2]],
    [[0,1],[0,2],[2,1]],
    [[0,0,3],[1,4,1]],
    [[3,1],[0,4],[0,1]],
    [[1,4,1],[3,0,0]],
    [[1,0],[4,0],[1,3]],
]

tetr_5 = [
    [[0,0,1],[2,1,2]],
    [[1,2],[0,1],[0,2]],
    [[2,1,2],[1,0,0]],
    [[2,0],[1,0],[2,1]],
    [[1,0,0],[4,1,3]],
    [[4,1],[1,0],[3,0]],
    [[3,1,4],[0,0,1]],
    [[0,3],[0,1],[1,4]],
]

tetr_6 = [
    [[1,0,0],[2,1,2]],
    [[2,1],[1,0],[2,0]],
    [[2,1,2],[0,0,1]],
    [[0,2],[0,1],[1,2]],
    [[0,0,4],[1,3,1]],
    [[4,1],[0,3],[0,1]],
    [[1,3,1],[4,0,0]],
    [[1,0],[3,0],[1,4]],
]

tetr_7 = [
    [[0,1,0],[1,2,1]],
    [[1,0],[2,1],[1,0]],
    [[1,2,1],[0,1,0]],
    [[0,1],[1,2],[0,1]],
    [[0,3,0],[4,1,3]],
    [[4,0],[1,3],[3,0]],
    [[3,1,4],[0,3,0]],
    [[0,3],[3,1],[0,4]],
]

tetr_8 = [
    [[0,2,0],[2,1,2]],
    [[2,0],[1,2],[2,0]],
    [[2,1,2],[0,2,0]],
    [[0,2],[2,1],[0,2]],
    [[0,1,0],[1,4,1]],
    [[1,0],[4,1],[1,0]],
    [[1,4,1],[0,1,0]],
    [[0,1],[1,4],[0,1]],
]

tetr_9 = [
    [[0,1,2],[1,2,0]],
    [[2,0],[1,2],[0,1]],
    [[0,2,1],[2,1,0]],
    [[1,0],[2,1],[0,2]],
    [[1,4,0],[0,1,3]],
    [[0,1],[1,4],[3,0]],
    [[3,1,0],[0,4,1]],
    [[0,3],[4,1],[1,0]],
]

tetr_10 = [
    [[1,2,0],[0,1,2]],
    [[0,1],[1,2],[2,0]],
    [[2,1,0],[0,2,1]],
    [[0,2],[2,1],[1,0]],
    [[0,3,1],[4,1,0]],
    [[4,0],[1,3],[0,1]],
    [[0,1,4],[1,3,0]],
    [[1,0],[3,1],[0,4]],
]

oct_1 = [
    [[1,2,1,2,1,2,1,2]],
    [[2,1,2,1,2,1,2,1]],
    [[1],[2],[1],[2],[1],[2],[1],[2]],
    [[2],[1],[2],[1],[2],[1],[2],[1]],
    [[1,3,1,4,1,3,1,4]],
    [[4,1,3,1,4,1,3,1]],
    [[1],[3],[1],[4],[1],[3],[1],[4]],
    [[4],[1],[3],[1],[4],[1],[3],[1]],
]

pieces = [
    mono_1, mono_2, domo_1, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4,
    tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1,
]

def int2TwoBits(x: int) -> str:
    if x == 1: return "00"
    if x == 2: return "01"
    if x == 3: return "10"
    if x == 4: return "11"
    return "00"

def encode2Ascii(s: str) -> str:
    res = ""
    assert len(s) // 16 == 8 or len(s) // 16 == 4
    for i in range(len(s) // 16):
        sub = s[8*i:8*(i+1)]
        res = chr(int(sub,2)) + res
    return res

class Board():
    def __init__(self) -> None:
        self.board = [ [0]*8 for i in range(8) ]
        self.mask = [ [0]*8 for i in range(8) ]

    def clear(self):
        for i in range(8):
            for j in range(8):
                self.board[i][j] = 0
                self.mask[i][j] = 0

    def getBitpattern128(self) -> str:
        res = ""
        for row in self.board:
            for cell in row:
                res += int2TwoBits(cell)
        return res


    def getMask64(self) -> str:
        res = ""
        for row in self.mask:
            for cell in row:
                res += str(cell)
        return res

    def getMask128(self) -> str:
        res = ""
        for row in self.mask:
            for cell in row:
                res += str(cell) * 2
        return res

    def placePiece(self, piece, fd):
        for i, config in enumerate(piece):                      # for all configurations
            width, height = len(config), len(config[0])
            for pos_x in range(8-(width-1)):                    # for each position
                for pos_y in range(8-(height-1)):
                    for x in range(width):
                        for y in range(height):
                            board_x = pos_x + x
                            board_y = pos_y + y
                            self.board[board_x][board_y] = config[x][y]
                            if config[x][y] != 0:
                                self.mask[board_x][board_y] = 1

                    mask64 = self.getMask64()
                    mask128 = self.getMask128()
                    pattern = self.getBitpattern128()

                    # binary_integer = int(mask64, 2)
                    # hex_string = format(binary_integer, '032x')
                    # print("{}\t{}\n".format(binary_integer, hex_string))

                    fd.write("{},{}\n".format(mask64, pattern))
                    self.clear()
        fd.write("\n")

with open("pieces2.txt", "w") as fd:
    board = Board()
    for piece in pieces:
        board.placePiece(piece, fd)
