
const SQSIZE = 60
const MINI_SQSIZE = 10
let alpha = 100
let REDFILL, BLACKFILL, BLUEFILL, YELLOWFILL
let gamepieces = []
let refboard, gameboard
let games
let game_name = "australian-emu"
// let game_name = "girl-wearing-cowboy-hat"

let selector
// let slider

/* ***************** INIT + MAIN LOOP ******************* */

function setup() {

  REDCLEAR = color(219, 57, 57, alpha)
  BLACKCLEAR = color(33, 33, 33, alpha)
  BLUECLEAR = color(32, 89, 168, alpha)
  YELLOWCLEAR = color(255, 222, 36, alpha)

  REDFILL = color(219, 57, 57)
  BLACKFILL = color(33, 33, 33)
  BLUEFILL = color(32, 89, 168)
  YELLOWFILL = color(255, 222, 36)

  angleMode(DEGREES)

  var cnv = createCanvas(window.innerWidth, window.innerHeight);
  cnv.position(0, 0, 'fixed')

  selector_size = 200;
  selector = createSelect();
  selector.size(selector_size);
  selector.position((width - selector_size) / 2, 70)
  selector.changed(parseBoard);

  // slider = createSlider(0, 255, 100);
  // slider.position(10, 10)
  // slider.changed(() => {
  //   alpha = slider.value()
  //   REDCLEAR = color(219, 57, 57, alpha)
  //   BLACKCLEAR = color(33, 33, 33, alpha)
  //   BLUECLEAR = color(32, 89, 168, alpha)
  //   YELLOWCLEAR = color(255, 222, 36, alpha)
  // })

  let start = (width - (8 * SQSIZE)) / 2
  refboard = new Board(puzzles["new"], start, 100, true)
  gameboard = new Board(puzzles["new2"], start, 100, false)

  fetch('../../boards/scraped-boards.json')
    // fetch('https://github.com/shivaPeri/kaleidoscope-puzzle/blob/main/boards/' + 'scraped-boards.json')
    .then(response => response.json())
    .then(data => {
      games = data

      for (const [key, _] of Object.entries(games)) {
        selector.option(key);
      }
      selector.selected(game_name)
      parseBoard()
      redraw()
    })
    .catch(error => console.log(error))


  for (var i = 0; i < pieces.length; i++)
    gamepieces.push(new Piece(pieces[i], i))
}

function draw() {
  background(255)
  gameboard.draw()
  refboard.draw()
  for (var piece of gamepieces) piece.draw()

  noLoop()
}

/* ***************** CLASSES ******************* */

// class Cell {
//   constructor(x, y, color) {
//     this.x = x
//     this.y = y
//     this.color = color
//   }

//   draw() {
//     noStroke()
//     fill(this.color)
//     rect(this.x + j * SQSIZE, this.y + i * SQSIZE, SQSIZE, SQSIZE)
//   }
// }

class Piece {
  constructor(ndarray, idx) {
    this.arr = nj.array(ndarray)

    // translate flag, animation flag
    this.selected = false
    this.transition = false
    this.progress = 0.0       // transition progress

    // position (random spawn) (spawn based on index)
    var mag = 10
    this.x = (idx % 6) * (width - mag) * 0.07 + (450)
    this.y = (~~(idx / 6)) * (height - mag) * 0.05 + (height / 4 * 3)
    // this.x = random(mag, width - mag)
    // this.y = random(height / 4 * 3, height - mag)

    // relative mouse position
    this.dx = 0
    this.dy = 0
  }

  rotate() {
    if (this.selected) {
      this.arr = nj.rot90(this.arr)
      this.x = mouseX + (this.y - mouseY)
      this.y = mouseY + (this.x - mouseX)
      this.transition = true
      this.rotateAnimation()
      // redraw()
      this.draw()
    }
  }

  rotateAnimation() {
    if (this.transition) {
      console.log("called")
      push()
      translate(-(this.x + this.dx), -(this.y + this.dy))
      while (this.progress < 1) {
        rotate(9)
        this.draw()
        this.progress += 1
      }
      this.transition = false
      this.progress = 0
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

    let size = this.selected ? SQSIZE : MINI_SQSIZE

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


        rect(this.x + j * size, this.y + i * size, size, size)


        if (this.mouseOver()) {
          fill(255, 255, 255, 50)
          rect(this.x + j * size, this.y + i * size, size, size)
        }

      }
    }

    stroke(0, 255, 0)
    noFill()
    circle(this.x, this.y, size)
    circle(this.x - this.dx, this.y - this.dy, size * 0.5)
  }

  mouseOver() {
    for (var i = 0; i < this.arr.shape[0]; i++) {
      for (var j = 0; j < this.arr.shape[1]; j++) {

        let size = this.selected ? SQSIZE : MINI_SQSIZE;
        if (this.arr.get(i, j, 0) != EMPTY) {

          if (mouseX > this.x + j * size &&
            mouseX < this.x + (j + 1) * size &&
            mouseY > this.y + i * size &&
            mouseY < this.y + (i + 1) * size)
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
          case RED: this.ref ? fill(REDCLEAR) : fill(REDFILL); break;
          case BLACK: this.ref ? fill(BLACKCLEAR) : fill(BLACKFILL); break;
          case BLUE: this.ref ? fill(BLUECLEAR) : fill(BLUEFILL); break;
          case YELLOW: this.ref ? fill(YELLOWCLEAR) : fill(YELLOWFILL); break;
          default: stroke(100); strokeWeight(2); noFill();
        }

        rect(this.x + j * SQSIZE, this.y + i * SQSIZE, SQSIZE, SQSIZE)
      }
    }

    if (this.ref) {
      noStroke()
      fill(0, 50)
      for (var i = 0; i < this.arr.length; i++) {
        for (var j = 0; j < this.arr[0].length; j++) {
          circle(this.x + (j + .5) * SQSIZE, this.y + (i + .5) * SQSIZE, SQSIZE / 2)
        }
      }
    }

    // if (this.ref) this.drawNotes()
  }

  mouseOver() {
    return (mouseX > this.x && mouseX < this.x + this.w &&
      mouseY > this.y && mouseY < this.y + this.h)
  }

  // snap piece location to closest legal board placement
  placePiece(piece, where) {
    if (isRef) return;

    // TODO

  }
}

/* ***************** HELPER FUNCTIONS ******************* */

const parseBoard = () => {
  let str = games[selector.value()]
  for (let i = 0; i < str.length; i++) {
    let row = Math.floor(i / 8);
    let col = i % 8;
    refboard.arr[row][col] = parseInt(str[i])
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
