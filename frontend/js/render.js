
const SQSIZE = 50
let REDFILL, BLACKFILL, BLUEFILL, YELLOWFILL
let gamepieces = []
let refboard, gameboard
let games
let game_name = "australian-emu"

/* ***************** INIT + MAIN LOOP ******************* */

function setup() {

    REDFILL = color(219, 57, 57)
    BLACKFILL = color(33, 33, 33)
    BLUEFILL = color(32, 89, 168)
    YELLOWFILL = color(255, 222, 36)

    var cnv = createCanvas(window.innerWidth, window.innerHeight);
    cnv.position(0, 0, 'fixed')

    refboard = new Board(puzzles["new"], 100, 100, true)
    gameboard = new Board(puzzles["new2"], refboard.arr.length * SQSIZE + 200, 100, false)

    fetch('../../boards/scraped-boards.json')
        .then(response => response.json())
        .then(data => {
            games = data
            for (let i = 0; i < data[game_name].length; i++) {
                let row = Math.floor(i/8);
                let col = i % 8;
                refboard.arr[row][col] = parseInt(data[game_name][i])
            }

            redraw()
        })
        .catch(error => console.log(error))


    for (var i = 0; i < pieces.length; i++)
        gamepieces.push(new Piece(pieces[i]))
}

function draw() {
    background(255)
    gameboard.draw()
    refboard.draw()
    for (var piece of gamepieces) piece.draw()

    noLoop()
}

/* ***************** CLASSES ******************* */

class Piece {
    constructor(ndarray) {
        this.arr = nj.array(ndarray)

        // translate flag
        this.selected = false

        // position (random spawn)
        var mag = 100
        this.x = random(mag, width - mag)
        this.y = random(height / 4 * 3, height - mag)

        // relative mouse position
        this.dx = 0
        this.dy = 0
    }

    rotate() {
        if (this.selected) {
            this.arr = nj.rot90(this.arr)
            this.x = mouseX + (this.y - mouseY)
            this.y = mouseY + (this.x - mouseX)
            redraw()
        }
    }

    flip() {
        if (this.selected) {
            this.arr = nj.flip(this.arr, 0)
            this.arr = nj.flip(this.arr, 2)
            redraw()
        }
    }

    draw() {

        noStroke()
        for (var i = 0; i < this.arr.shape[0]; i++) {
            for (var j = 0; j < this.arr.shape[1]; j++) {

                switch (this.arr.get(i, j, 0)) {
                    case RED: fill(REDFILL); break;
                    case BLACK: fill(BLACKFILL); break;
                    case BLUE: fill(BLUEFILL); break;
                    case YELLOW: fill(YELLOWFILL); break;
                    default: continue;
                }

                rect(this.x + j * SQSIZE, this.y + i * SQSIZE, SQSIZE, SQSIZE)

                if (this.mouseOver()) {
                    fill(255, 255, 255, 50)
                    rect(this.x + j * SQSIZE, this.y + i * SQSIZE, SQSIZE, SQSIZE)
                }

            }
        }
    }

    mouseOver() {
        for (var i = 0; i < this.arr.shape[0]; i++) {
            for (var j = 0; j < this.arr.shape[1]; j++) {

                if (this.arr.get(i, j, 0) != EMPTY) {

                    if (mouseX > this.x + j * SQSIZE &&
                        mouseX < this.x + (j + 1) * SQSIZE &&
                        mouseY > this.y + i * SQSIZE &&
                        mouseY < this.y + (i + 1) * SQSIZE)
                        return true;

                }

            }
        }

        return false;
    }

}

class Board {
    constructor(ndarray, x, y, isRef) {
        this.arr = ndarray
        this.notes = notes
        this.x = x
        this.y = y
        this.ref = isRef

        this.w = ndarray[0].length * SQSIZE
        this.h = ndarray.length * SQSIZE
    }

    drawNotes() {
        for (let i = 0; i < this.arr.length; i++) {
            for (let j = 0; j < this.arr[0].length; j++) {

                let offset = 3
                let sx = this.x + i * SQSIZE
                let sy = this.y + j * SQSIZE
                stroke(255)
                strokeWeight(5)
                strokeCap(ROUND)
                line(sx + offset, sy, sx + SQSIZE - offset, sy)
                line(sx, sy + offset, sx, sy + SQSIZE - offset)
            }
        }
    }

    draw() {

        for (var i = 0; i < this.arr.length; i++) {
            for (var j = 0; j < this.arr[0].length; j++) {

                noStroke()
                switch (this.arr[i][j]) {
                    case RED: fill(REDFILL); break;
                    case BLACK: fill(BLACKFILL); break;
                    case BLUE: fill(BLUEFILL); break;
                    case YELLOW: fill(YELLOWFILL); break;
                    default: stroke(230); strokeWeight(2); noFill();
                }

                rect(this.x + j * SQSIZE, this.y + i * SQSIZE, SQSIZE, SQSIZE)
            }
        }

        // if (this.ref) this.drawNotes()
    }

    mouseOver() {
        return (mouseX > this.x && mouseX < this.x + this.w &&
            mouseY > this.y && mouseY < this.y + this.h)
    }

    placePiece(piece, where) {
        if (isRef) return;

        // TODO

    }
}

/* ***************** EVENT HANDLERS ******************* */

function windowResized() {
    resizeCanvas(windowWidth, windowHeight);
}

function mouseMoved() {
    for (var piece of gamepieces) {
        if (piece.selected) {
            piece.x = mouseX + piece.dx
            piece.y = mouseY + piece.dy
        }
    }
    redraw()
}


function keyPressed() {
    for (var piece of gamepieces) {
        if (piece.selected) {
            switch (keyCode) {
                case (UP_ARROW): piece.flip(); break;
                case (DOWN_ARROW): piece.flip(); break;
                case (RIGHT_ARROW): piece.rotate(); break;
                case (LEFT_ARROW): piece.rotate(); break;
            }
        }
    }
}

function mouseClicked() {
    for (var piece of gamepieces) {

        if (piece.selected) {
            piece.selected = !piece.selected
            return
        }

        if (piece.mouseOver()) {

            piece.selected = !piece.selected
            piece.dx = piece.x - mouseX
            piece.dy = piece.y - mouseY
        }
    }
}