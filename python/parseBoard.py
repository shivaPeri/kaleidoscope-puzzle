import numpy as np
import cv2, requests, imageio, urllib.request
from bs4 import BeautifulSoup
from tqdm import tqdm

url = "http://www.users.on.net/~mikegatley/kaleidoscope/all1.html"
url2 = "http://www.users.on.net/~mikegatley/kaleidoscope/all2.html"

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

def scrapeBoardsFromURL(url, filepath, include_comma=True):
    response = requests.get(url)

    ignore = [ "e-mail me", "web site hit counter" ]

    soup = BeautifulSoup(response.content, 'html.parser')
    images = soup.find_all('img')

    with open(filepath, "a") as fd:

        for i, im in tqdm(enumerate(images)):

            title = im.get('alt')
            if title not in ignore:

                src = im.get('src')
                imdata = urllib.request.urlopen(src).read()
                imbytes = bytearray(imdata)
                open("tmp.gif","wb+").write(imbytes)    # write to tmp file

                gif = imageio.mimread("tmp.gif")
                name = title.replace(' ', '-').lower()
                img = cv2.cvtColor(gif[0], cv2.COLOR_RGBA2RGB)
                add_comma = "," if include_comma or (i != len(images) - 1) else ""
                fd.write('\t"{}": "{}"{}\n'.format(name, im2board(img), add_comma))

    print('done scraping')

if __name__ == "__main__":

    # testing im2board function
    path = "../boards/images/"
    fname = "kitten-playing.png"
    img = cv2.imread(path + fname)
    img = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)
    print(im2board(img))

    # testing web scraping
    filepath = '../boards/scraped-boards.json'
    with open(filepath, "w") as fd:
        fd.write("{\n")

    scrapeBoardsFromURL(url, filepath)
    scrapeBoardsFromURL(url2, filepath, False)

    with open(filepath, "a") as fd:
        fd.write("}")