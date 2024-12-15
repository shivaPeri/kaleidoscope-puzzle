import json
import numpy as np
from PIL import Image

class CanvasFrame():
    def __init__(self, width, height):
        self.width = width
        self.height = height
        
        # 4 channels for rgba
        self.data = np.zeros((width, height, 4), dtype=np.int32) 
        
    def _in_bounds(self, x, y):
        return 0 <= x and x < self.width and 0 <= y and y < self.height
    
    def _get_cell_color(self, color):
        # 1 bit colors
        if color == "1": return np.array([0, 0, 0, 255])        # black
        if color == "2": return np.array([218, 56, 50, 255])    # red
        if color == "3": return np.array([255, 222, 36, 255])   # yellow
        if color == "4": return np.array([32, 89, 168, 255])    # blue
        
        # 2-bit colors
        if color == "00": return np.array([0, 0, 0, 255])       # black
        if color == "01": return np.array([218, 56, 50, 255])   # red
        if color == "10": return np.array([255, 222, 36, 255])  # yellow
        if color == "11": return np.array([32, 89, 168, 255])   # blue
        
        # default case
        return np.array([0, 0, 0, 0])                           # transparent
    
    def _get_mask_color(self, color):
        if color == "0": return np.array([0, 0, 0, 255])   # black
        return np.array([255, 255, 255, 255])              # white
    
    def _get_board_array(self, board):
        colors = []
        if len(board) == 64:
            colors = [board[i] for i in range(len(board))]
        elif len(board) == 64 * 2:
            colors = [board[i:i+2] for i in range(0, len(board), 2)]
        
        assert len(colors) == 64
        return colors
    
    def load_board_from_file(board_title="chaos", input_path="../boards/boards.json"):
        with open(input_path, 'r') as fd:
            data = json.load(fd)
            return data[board_title]
    
    # adds 64x1 line to data array from color array
    # guarenteed len(colors) == 64
    def draw_bitvector(self, board, x=0, y=0, is_mask=False):
        colors = self._get_board_array(board)
        for i in range(64):    
            cell = colors[i]
            if is_mask:
                self.data[x][y + i] = self._get_mask_color(cell)
            else:
                self.data[x][y + i] = self._get_cell_color(cell)
        
    # adds 8x8 square to data array from color array
    # guarenteed len(colors) == 64
    def draw_board(self, board, x=0, y=0, is_mask=False):
        colors = self._get_board_array(board)
        for i in range(8):    
            for j in range(8):
                if self._in_bounds(i,j):
                    cell = colors[i * 8 + j]
                    if is_mask:
                        self.data[x + i][y + j] = self._get_mask_color(cell)
                    else:
                        self.data[x + i][y + j] = self._get_cell_color(cell)
    
    # clear a given section of the data array 
    def clear_area(self, x, y, w, h):
        for i in range(x, x+w):
            for j in range(y, y+h):
                if self._in_bounds(i,j):
                    self.data[i,j,:] = 0
    
    # return a PIL image frame
    def generate_frame(self):
        w, h = self.width, self.height
        img = Image.new("RGBA", (w, h))
        pixels = self.data.reshape(w*h, 4)
        pixels = [(item[0], item[1], item[2], item[3]) for item in pixels]
        img.putdata(pixels)
        return img

class PixelAnimation():
    def __init__(self):
        self.frames = []
        self.durations = []
        
    def add_frame(self, frame, duration=10):
        self.frames.append(frame)
        self.durations.append(duration)
    
    def export(self, out_path="animation.png"):
        self.frames[0].save(
            out_path,
            save_all=True,
            append_images=self.frames[1:],
            duration=self.durations,
            loop=0,
            format="PNG",
        )
        
        print(f"Saved animation to: {out_path}")
    
def test():
    animation = PixelAnimation()
    canvas = CanvasFrame(20,20)

    for i in range(64):
        board_string = str(1+ i % 4) * i + "0" * (64-i)
        canvas.draw_board(board_string)
        canvas.draw_board(board_string, 10, 10)
        frame = canvas.generate_frame()
        animation.add_frame(frame)
        
    animation.export()

def generate_figure_1():
    # this figure should show the process of pieces are converted to bit strings
    
    board_size = 8
    bitvector_size = board_size * board_size
    spacing = 2
    row_items = [board_size, board_size, bitvector_size, bitvector_size]
    
    canvas_width = sum(row_items) + (len(row_items) - 1) * spacing
    canvas_height = 1000
    

if __name__ == "__main__":
    print("started")
    test()