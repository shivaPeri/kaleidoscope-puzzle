import json
import cv2 as cv
import numpy as np
from PIL import Image

# Read the JSON file
with open(input_path, 'r') as fd:
    data = json.load(fd)


class ImageFrame():
    
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.data = np.array(width, height, 4) # 4 channels for rgba
    
    def _get_cell_color(self, color):
        if color == "1": return np.array([0, 0, 0, 100])        # black
        if color == "2": return np.array([218, 56, 50, 100])    # red
        if color == "3": return np.array([255, 222, 36, 100])   # yellow
        if color == "4": return np.array([32, 89, 168, 100])    # blue
        return np.array([0, 0, 0, 0])                           # transparent
    
    def _get_cell_color_2_bit(self, color):
        if color == "00": return np.array([0, 0, 0, 100])        # black
        if color == "01": return np.array([218, 56, 50, 100])    # red
        if color == "10": return np.array([255, 222, 36, 100])   # yellow
        if color == "11": return np.array([32, 89, 168, 100])    # blue
        return np.array([0, 0, 0, 0])                            # transparent
    
    def _draw_board_2_bit(self, board, x, y):
        pass
    
    def _draw_board(self, board, x, y):
        pass
    
    def draw_board(self, board, x, y):
        if len(board) == 64:
            self._draw_board(board, x, y)
        elif len(board) == 64 * 2:
            self._draw_board_2_bit(board, x, y)
    
    def generate_frame(self):
        w, h = self.width, self.height
        img = Image.new("RGBA", (w, h))
        img.putdata(self.data.reshape(w*h, 4))
        return img


def map2color(i: str):
    if int(i) == 1: return "0 0 0\n"        # black
    if int(i) == 2: return "218 56 50\n"    # red
    if int(i) == 3: return "255 222 36\n"   # yellow
    if int(i) == 4: return "32 89 168\n"    # blue
    return ""

for title, board in data.items():

    with open(output_dir + 'tmp.ppm', 'w') as f:
        f.write("P3\n")
        f.write("8 8\n")
        f.write("256\n")

        for c in board:
            f.write(map2color(c))

    im = Image.open(output_dir + "tmp.ppm")
    im.convert("RGB").save(output_dir + title + ".png")

if __name__ == "__main__":
    print("started")
    print("finished")