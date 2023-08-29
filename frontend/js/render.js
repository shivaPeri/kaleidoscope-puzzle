
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
let selected_piece = null

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

  // canvas HTML object
  var cnv = createCanvas(window.innerWidth, window.innerHeight);
  cnv.position(0, 0, 'fixed')

  // game selector dropdown
  selector_size = 200;
  selector = createSelect();
  selector.size(selector_size);
  selector.position((width - selector_size) / 2, 70)
  selector.changed(parseBoard);

  let start = (width - (8 * SQSIZE)) / 2
  refboard = new Board(puzzles["new"], start, 100, true)
  gameboard = new Board(puzzles["new2"], start, 100, false)

  console.log(gameboard)

  fetch('../../boards/scraped-boards.json')
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

  noLoop()
}

function draw() {
  background(255)
  gameboard.draw()
  refboard.draw()
  for (var piece of gamepieces) piece.draw()

  if (selected_piece != null) {
    let piece = gamepieces[selected_piece]

    noFill()
    strokeWeight(2)
    stroke(0, 255, 0)
    // circle(piece.x, piece.y, SQSIZE)

    let dx = piece.arr.shape[1] * SQSIZE / 2
    let dy = piece.arr.shape[0] * SQSIZE / 2

    let x = piece.x - dx + SQSIZE / 2
    let y = piece.y - dy + SQSIZE / 2
    circle(x, y, SQSIZE)

    let min_d = 100
    let min_x = 0
    let min_y = 0
    let idx = null
    for (let i = 0; i < 8; i++) {
      for (let j = 0; j < 8; j++) {
        let bx = gameboard.x + SQSIZE * i + SQSIZE / 2
        let by = gameboard.y + SQSIZE * j + SQSIZE / 2
        circle(bx, by, 10)
        if (dist(x, y, bx, by) < min_d) {
          min_d = dist(x, y, bx, by)
          min_x = bx
          min_y = by
          idx = [i, j]
        }
      }
    }

    console.log(idx)
    circle(min_x, min_y, 30)
  }
}

/* ***************** CLASSES ******************* */

class Piece {
  constructor(ndarray, idx) {
    this.id = idx
    this.arr = nj.array(ndarray)
    this.placed = false

    console.log(this.arr)

    // position (spawn based on index)
    var mag = 10
    this.spawn_x = (idx % 6) * (width - mag) * 0.07 + (450)
    this.spawn_y = (~~(idx / 6)) * (height - mag) * 0.05 + (height / 4 * 3)
    this.x = this.spawn_x
    this.y = this.spawn_y
  }

  dims() {
    return [this.arr.shape[0], this.arr.shape[1]]
  }

  rotate() {
    if (selected_piece == this.id) {
      this.arr = nj.rot90(this.arr)
      this.x = mouseX + (this.y - mouseY)
      this.y = mouseY + (this.x - mouseX)
      redraw()
    }
  }

  flip() {
    if (selected_piece == this.id) {
      this.arr = nj.flip(this.arr, 0)
      this.arr = nj.flip(this.arr, 2)
      redraw()
    }
  }

  draw() {

    noStroke()
    let size = selected_piece == this.id ? SQSIZE : MINI_SQSIZE
    size = this.placed ? SQSIZE : size

    let dx = this.arr.shape[1] * size / 2
    let dy = this.arr.shape[0] * size / 2

    for (var i = 0; i < this.arr.shape[0]; i++) {
      for (var j = 0; j < this.arr.shape[1]; j++) {

        switch (this.arr.get(i, j, 0)) {
          case RED: fill(REDFILL); break;
          case BLACK: fill(BLACKFILL); break;
          case BLUE: fill(BLUEFILL); break;
          case YELLOW: fill(YELLOWFILL); break;
          default: continue;
        }

        rect(this.x + j * size - dx, this.y + i * size - dy, size, size)

        if (this.mouseOver()) {
          fill(255, 255, 255, 50)
          rect(this.x + j * size - dx, this.y + i * size - dy, size, size)
        }
      }
    }
  }

  mouseOver() {
    for (var i = 0; i < this.arr.shape[0]; i++) {
      for (var j = 0; j < this.arr.shape[1]; j++) {

        let size = selected_piece == this.id ? SQSIZE : MINI_SQSIZE;
        size = this.placed ? SQSIZE : size
        let dx = this.arr.shape[1] * size / 2
        let dy = this.arr.shape[0] * size / 2
        if (this.arr.get(i, j, 0) != EMPTY) {

          if (mouseX > this.x + j * size - dx &&
            mouseX < this.x + (j + 1) * size - dx &&
            mouseY > this.y + i * size - dy &&
            mouseY < this.y + (i + 1) * size - dy)
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

  // drawNotes() {
  //   for (let i = 0; i < this.arr.length; i++) {
  //     for (let j = 0; j < this.arr[0].length; j++) {

  //       let offset = 3
  //       let sx = this.x + i * SQSIZE
  //       let sy = this.y + j * SQSIZE
  //       stroke(255)
  //       strokeWeight(5)
  //       strokeCap(ROUND)
  //       line(sx + offset, sy, sx + SQSIZE - offset, sy)
  //       line(sx, sy + offset, sx, sy + SQSIZE - offset)
  //     }
  //   }
  // }

  draw() {
    noStroke()
    for (var i = 0; i < this.arr.length; i++) {
      for (var j = 0; j < this.arr[0].length; j++) {

        switch (this.arr[i][j]) {
          case RED: this.ref ? fill(REDCLEAR) : fill(REDFILL); break;
          case BLACK: this.ref ? fill(BLACKCLEAR) : fill(BLACKFILL); break;
          case BLUE: this.ref ? fill(BLUECLEAR) : fill(BLUEFILL); break;
          case YELLOW: this.ref ? fill(YELLOWCLEAR) : fill(YELLOWFILL); break;
          default: fill(255, 10)
          // default: stroke(100); strokeWeight(2); noFill();
        }

        rect(this.x + j * SQSIZE, this.y + i * SQSIZE, SQSIZE, SQSIZE)
      }
    }
  }

  mouseOver() {
    return (mouseX > this.x && mouseX < this.x + this.w &&
      mouseY > this.y && mouseY < this.y + this.h)
  }

  // snap piece location to closest legal board placement
  // returns boolean value on sucess or failure
  place() {

    if (selected_piece != null) {
      let piece = gamepieces[selected_piece]

      let dx = piece.arr.shape[1] * SQSIZE / 2
      let dy = piece.arr.shape[0] * SQSIZE / 2

      // x and y of first cell in piece
      let x = piece.x - dx + SQSIZE / 2
      let y = piece.y - dy + SQSIZE / 2

      // find min distance cell
      let min_d = 100
      let min_x = 0
      let min_y = 0
      let min_i = 0
      let min_j = 0

      for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
          let bx = gameboard.x + SQSIZE * i + SQSIZE / 2
          let by = gameboard.y + SQSIZE * j + SQSIZE / 2

          if (dist(x, y, bx, by) < min_d) {
            min_d = dist(x, y, bx, by)
            min_x = bx
            min_y = by
            min_i = i
            min_j = j
          }
        }
      }

      // bounds check
      let [w, h] = piece.dims()
      let piece_fits = true
      if (min_i + w <= 8 || min_h + h <= 8) {
        // check all cells are empty
        for (let i = min_i; i < min_i + w; i++) {
          for (let j = min_j; j < min_j + h; j++) {
            if (gameboard.arr[i][j] != -1) {
              piece_fits = false
            }
          }
        }

      }

      // actually place piece onto board
      if (piece_fits) {
        for (let i = min_i; i < min_i + w; i++) {
          for (let j = min_j; j < min_j + h; j++) {
            if (piece.arr.get(i - min_i, j - min_j, 0) != 0) {
              gameboard.arr[i][j] = selected_piece
            }
          }
        }

        piece.placed = true
        piece.x = min_x + SQSIZE / 2
        piece.y = min_y + SQSIZE / 2
        selected_piece = null
        return true
      }

      // unmark board
      for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
          if (gameboard.arr[i][j] == selected_piece) {
            gameboard.arr[i][j] = -1
          }
        }
      }
      piece.placed = false
      piece.x = piece.spawn_x
      piece.y = piece.spawn_y
    }

    return false
  }
}

/* ***************** HELPER FUNCTIONS ******************* */

const dist = (x1, y1, x2, y2) => {
  return Math.sqrt((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2))
}

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

function mouseDragged() {
  if (selected_piece != null) {
    let piece = gamepieces[selected_piece]
    piece.x = mouseX
    piece.y = mouseY
    redraw()
  }
}

function mouseMoved() {
  redraw()
}

function keyPressed() {
  if (selected_piece != null) {
    let piece = gamepieces[selected_piece]
    switch (keyCode) {
      case (UP_ARROW): piece.flip(); break;
      case (DOWN_ARROW): piece.flip(); break;
      case (RIGHT_ARROW): piece.rotate(); break;
      case (LEFT_ARROW): piece.rotate(); break;
    }
  }
}

function mousePressed() {
  if (selected_piece == null) {
    for (var piece of gamepieces) {
      if (piece.mouseOver()) {
        selected_piece = piece.id
      }
    }
    redraw()
  }
}

function mouseReleased() {
  console.log(gameboard)
  if (selected_piece != null) {
    gameboard.place()
    selected_piece = null
    redraw()
  }

  console.log(gameboard)
}
