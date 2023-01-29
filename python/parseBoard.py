import cv2 as cv
import numpy as np

colors = np.array([[0,0,0],         # black
                    [255,0,0],      # red
                    [255,255,0],    # yellow
                    [0,0,255]])     # blue

# given cropped board image, converts to bitstring
def im2board(im):
    
    out = ""
    square = im.shape[0] // 8

    for i in range(8):
        for j in range(8):
            x = i * square + square // 2
            y = j * square + square // 2

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