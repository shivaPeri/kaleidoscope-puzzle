import json
import cv2 as cv
from PIL import Image

# input_path = '../boards/scraped-boards.json'
input_path = '../boards/boards.json'
output_dir = '../doc/'

# Read the JSON file
with open(input_path, 'r') as fd:
    data = json.load(fd)

def map2color(i: str):
    if int(i) == 1: return "0 0 0\n"        # black
    if int(i) == 2: return "218 56 50\n"    # red
    if int(i) == 3: return "255 222 36\n"   # yellow
    if int(i) == 4: return "32 89 168\n"    # blue
    return ""

for title, board in data.items():

    with open(output_dir + 'tmp.ppm', 'w') as f:
    # with open(output_dir + title + '.ppm', 'w') as f:
        f.write("P3\n")
        f.write("8 8\n")
        f.write("256\n")

        for c in board:
            f.write(map2color(c))


    # i = cv.imread(output_dir + "tmp.ppm")
    # cv.imwrite(output_dir + title + ".jpg",i)
    im = Image.open(output_dir + "tmp.ppm")
    im.convert("RGB").save(output_dir + title + ".png")
