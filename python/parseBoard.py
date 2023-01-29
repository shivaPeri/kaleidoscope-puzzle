import cv2 as cv
import numpy as np

colors = np.array([[0,0,0],         # black
                    [255,0,0],      # red
                    [255,255,0],    # yellow
                    [0,0,255]])     # blue

# TODO given cropped board image, converts to bitstring
# TODO writes new boards to board folder
def im2board(im, name="new-board"):
    
    w, h, _ = im.shape
    square = w // 8

    out = ""

    for i in range(8):
        for j in range(8):
            y = i * square + square // 2
            x = j * square + square // 2

            pix = im[x,y]
            similarity = colors - pix
            scores = np.linalg.norm(similarity, axis=1)
            out += str(scores.argmin())

    return out

if __name__ == "__main__":

    path = "../boards/images/"
    fname = "emu.png"
    img = cv.imread(path + fname)
    img = cv.cvtColor(img, cv.COLOR_BGR2RGB)

    print(im2board(img))