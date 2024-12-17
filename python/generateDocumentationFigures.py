import json
import numpy as np
from PIL import Image
from tqdm import tqdm

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
        if color == "1": return np.array([33, 33, 33, 255])        # black
        if color == "2": return np.array([218, 56, 50, 255])    # red
        if color == "3": return np.array([255, 222, 36, 255])   # yellow
        if color == "4": return np.array([32, 89, 168, 255])    # blue
        
        # 2-bit colors
        if color == "00": return np.array([33, 33, 33, 255])       # black
        if color == "01": return np.array([218, 56, 50, 255])   # red
        if color == "10": return np.array([255, 222, 36, 255])  # yellow
        if color == "11": return np.array([32, 89, 168, 255])   # blue
        
        # default case
        return np.array([0, 0, 0, 0])                           # transparent
    
    def _apply_mask_value(self, color):
        return color == "1" or color == "11"
    
    def _get_mask_color(self, color):
        
        # 1 bit colors
        if color == "0": return np.array([0, 0, 0, 255])        # black
        if color == "1": return np.array([255, 255, 255, 255])  # white
        
        # 2-bit colors
        if color == "00": return np.array([0, 0, 0, 255])       # black
        if color == "11": return np.array([255, 255, 255, 255]) # white
        
        # default case
        return np.array([0, 0, 0, 0])                           # transparent
    
    def _get_board_array(self, board):
        colors = []
        if len(board) == 64:
            colors = [board[i] for i in range(len(board))]
        elif len(board) == 64 * 2:
            colors = [board[i:i+2] for i in range(0, len(board), 2)]
        
        assert len(colors) == 64
        return colors
    
    # clear a given section of the data array 
    def _clear_area(self, x, y, w, h):
        for i in range(x, x+w):
            for j in range(y, y+h):
                if self._in_bounds(i,j):
                    self.data[i,j,:] = 0
                    
    # change opacity of a given section of the data array 
    def set_area_opacity(self, x, y, w, h, opacity=50):
        for i in range(x, x+w):
            for j in range(y, y+h):
                if self._in_bounds(i,j):
                    self.data[i,j,-1] = opacity
    
    def load_board_from_file(board_title="chaos", input_path="../boards/boards.json"):
        with open(input_path, 'r') as fd:
            data = json.load(fd)
            return data[board_title]
    
    # adds 64x1 line to data array from color array
    # guarenteed len(colors) == 64
    def draw_bitvector(self, board, x=0, y=0, draw_mask=False):
        colors = self._get_board_array(board)
        self._clear_area(x, y, 1, 64)
        for i in range(64):    
            cell = colors[i]
            x_pos, y_pos = x, y + i
            if self._in_bounds(x_pos, y_pos):
                if draw_mask:
                    self.data[x_pos][y_pos] = self._get_mask_color(cell)
                else:
                    self.data[x_pos][y_pos] = self._get_cell_color(cell)
        
    # adds 8x8 square to data array from color array
    # guarenteed len(colors) == 64
    def draw_board(self, board, x=0, y=0, mask=None, draw_mask=False):
        colors = self._get_board_array(board)

        if mask is not None:
            mask_values = self._get_board_array(mask)
        
        self._clear_area(x, y, 8, 8)
        for i in range(8):    
            for j in range(8):
                flat_idx = i * 8 + j
                cell = colors[flat_idx]
                mask_value = self._apply_mask_value(mask_values[flat_idx]) if mask is not None else True
                
                if mask_value:
                    x_pos, y_pos = x + i, y + j
                    if self._in_bounds(x_pos, y_pos):
                        if draw_mask:
                            self.data[x_pos][y_pos] = self._get_mask_color(cell)
                        else:
                            self.data[x_pos][y_pos] = self._get_cell_color(cell)
    
    # return a PIL image frame
    def generate_frame(self):
        w, h = self.width, self.height
        img = Image.new("RGBA", (h,w))
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
            optimize=False,
            compress_level=0          # No compression (fully lossless)
        )
        
        print(f"Saved animation to: {out_path}")
    
def test():
    animation = PixelAnimation()
    canvas = CanvasFrame(20,20)

    for i in range(64):
        board_string = str(1+ i % 4) * i + "0" * (64-i)
        canvas.draw_board(board_string)
        canvas.draw_board(board_string, 3, 10)
        frame = canvas.generate_frame()
        animation.add_frame(frame)
        
    animation.export()
    
def get_piece_placements(index=-1):
    with open("./pieces3.txt", "r") as fd:
        file_content = fd.read()
        pieces = file_content.split("\n\n")
        pieces = [ piece for piece in pieces if len(piece) > 0 ]
        
        # array of all possible configurations
        selected_piece = pieces[index].split("\n")
        return selected_piece

def generate_figure_1():
    # this figure should show the process of pieces are converted to bit strings
    
    board_size = 8
    bitvector_size = board_size * board_size
    horizontal_spacing = 2
    vertical_spacing = 1
    row_items = [board_size, board_size, bitvector_size, bitvector_size]
    
    piece_idx = 17
    piece_placements = get_piece_placements(piece_idx)
    canvas_height = sum(row_items) + (len(row_items) - 1) * horizontal_spacing
    canvas_width = max(8, len(piece_placements) * vertical_spacing)
    
    animation = PixelAnimation()
    canvas = CanvasFrame(canvas_width, canvas_height)
    
    start_x = 0
    print(f"{len(piece_placements)} total frames")
    for frame_idx, placement in tqdm(enumerate(piece_placements)):
        mask, board = placement.split(",")
        
        start_y = 0
        canvas.draw_board(board, 0, start_y, mask=mask)
        start_y += board_size + horizontal_spacing
        
        canvas.draw_board(mask, 0, start_y, draw_mask=True)
        start_y += board_size + horizontal_spacing
        
        canvas.set_area_opacity(start_x-vertical_spacing, start_y, 1, 64)
        canvas.draw_bitvector(board, start_x, start_y)
        start_y += bitvector_size + horizontal_spacing
        
        canvas.set_area_opacity(start_x-vertical_spacing, start_y, 1, 64)
        canvas.draw_bitvector(mask, start_x, start_y, draw_mask=True)
        start_x += vertical_spacing
        
        frame = canvas.generate_frame()
        animation.add_frame(frame)
        
    animation.export(out_path=f"piece_{piece_idx}_placements.png")
    
    
def generate_figure_2(piece_idx=17):
    # this figure should show the process of pieces are converted to bit strings
    
    board_size = 8
    bitvector_size = board_size * board_size
    horizontal_spacing = 2
    
    piece_placements = get_piece_placements(piece_idx)
    canvas_height = board_size + horizontal_spacing + board_size
    canvas_width = board_size
    
    animation = PixelAnimation()
    canvas = CanvasFrame(canvas_width, canvas_height)
    
    print(f"{len(piece_placements)} total frames")
    for frame_idx, placement in tqdm(enumerate(piece_placements)):
        mask, board = placement.split(",")
        
        start_y = 0
        canvas.draw_board(board, 0, start_y, mask=mask)
        start_y += board_size + horizontal_spacing
        
        canvas.draw_board(mask, 0, start_y, draw_mask=True)
        start_y += board_size + horizontal_spacing
        
        frame = canvas.generate_frame()
        animation.add_frame(frame)
        
    animation.export(out_path=f"piece_{piece_idx}_placements_small.png")

def generate_figure_3():
    pass

if __name__ == "__main__":
    print("started")
    # generate_figure_1()
    
    # for i in range(18):
    #     generate_figure_2(i)
    
    generate_figure_3()