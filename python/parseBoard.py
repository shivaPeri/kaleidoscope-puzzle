import cv2 as cv
import numpy as np

import time
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support.expected_conditions import presence_of_element_located

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

def scrapeBoardsFromURL(url):
    with webdriver.Chrome(executable_path='/Users/shivaperi/bin/chromedriver') as driver:

        driver.get(url)
        time.sleep(30)

        i = 0
        while i < 200:

            wait = WebDriverWait(driver, 2)

            try:
                button = wait.until(presence_of_element_located((By.ID, 'download-button-' + model_id)))
                button.click()

                button = wait.until(presence_of_element_located((By.ID, 'download-option-zip')))
                button.click()
            except:
                continue
            
            print(i)
            i += 1

    print('done scraping')

if __name__ == "__main__":

    path = "../boards/images/"
    fname = "kitten-playing.png"
    img = cv.imread(path + fname)
    img = cv.cvtColor(img, cv.COLOR_BGR2RGB)

    print(im2board(img))