import json
import numpy as np
from PIL import Image

def load_board_from_file(input_path):
    with open(input_path, 'r') as fd:
        data = json.load(fd)
        return data


class ImageFrame():
    def __init__(self, width, height):
        self.width = width
        self.height = height
        
        # 4 channels for rgba
        self.data = np.zeros((width, height, 4), dtype=np.int32) 
    
    def _get_cell_color(self, color):
        if color == "1": return np.array([0, 0, 0, 255])        # black
        if color == "2": return np.array([218, 56, 50, 255])    # red
        if color == "3": return np.array([255, 222, 36, 255])   # yellow
        if color == "4": return np.array([32, 89, 168, 255])    # blue
        return np.array([0, 0, 0, 0])                           # transparent
    
    def _get_cell_color_2_bit(self, color):
        if color == "00": return np.array([0, 0, 0, 255])        # black
        if color == "01": return np.array([218, 56, 50, 255])    # red
        if color == "10": return np.array([255, 222, 36, 255])   # yellow
        if color == "11": return np.array([32, 89, 168, 255])    # blue
        return np.array([0, 0, 0, 0])                            # transparent
    
    def _draw_board_2_bit(self, board, x, y):
        colors = [board[i:i+2] for i in range(0, len(board), 2)]
        for i in range(8):    
            for j in range(8):     
                color_idx = i * 8 + j
                self.data[x + j][y + i] = self._get_cell_color_2_bit(colors[color_idx])
    
    def _draw_board(self, board, x, y):
        colors = [board[i] for i in range(len(board))]
        for i in range(8):    
            for j in range(8):     
                color_idx = i * 8 + j
                self.data[x + j][y + i] = self._get_cell_color(colors[color_idx])
    
    def draw_board(self, board, x=0, y=0):
        if len(board) == 64:
            self._draw_board(board, x, y)
        elif len(board) == 64 * 2:
            self._draw_board_2_bit(board, x, y)
    
    def generate_frame(self):
        w, h = self.width, self.height
        img = Image.new("RGBA", (w, h))
        pixels = self.data.reshape(w*h, 4)
        pixels = [(item[0], item[1], item[2], item[3]) for item in pixels]
        img.putdata(pixels)
        return img
    
def test():
    frames = []
    for i in range(64):
        img = ImageFrame(20,20)
        board_string = "1" * i + "0" * (64-i)
        img.draw_board(board_string)
        img.draw_board(board_string, 10, 10)
        frame = img.generate_frame()
        frames.append(frame)


    # # Define custom durations for each frame
    # durations = [255, 200, 50, 300, 150, 100, 200, 50, 300, 150]  # Milliseconds

    frames[0].save(
        "animation.png",
        save_all=True,
        append_images=frames[1:],
        duration=100,
        # duration=durations,
        loop=0,
        format="PNG",
    )

if __name__ == "__main__":
    print("started")
    test()
    print("finished")