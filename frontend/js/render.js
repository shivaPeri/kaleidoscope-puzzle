
const SQSIZE = 60
const MINI_SQSIZE = 10

const alpha = "aa"
const RED_FILL = "#db3939"
const BLACK_FILL = "#212121"
const BLUE_FILL = "#2059a8"
const YELLOW_FILL = "#ffde24"

let gamepieces = []
let refboard, gameboard
let board
let games
let game_name = "australian-emu"
// let game_name = "girl-wearing-cowboy-hat"

let selector
let selected_piece = null

/* ***************** INIT + MAIN LOOP ******************* */

function setup() {

  // canvas HTML object
  var cnv = createCanvas(window.innerWidth, window.innerHeight);
  cnv.position(0, 0, 'fixed')

  // game selector dropdown
  selector_size = 200;
  selector = createSelect();
  selector.size(selector_size);
  selector.position((width - selector_size) / 2, 70)
  selector.changed(parseBoard);

  let start_x = (width - (8 * SQSIZE)) / 2
  // refboard = new Board(puzzles["new"], start_x, 100, true)
  // gameboard = new Board(puzzles["new2"], start_x, 100, false)

  fetch('../../boards/scraped-boards.json')
    .then(response => response.json())
    .then(data => {
      games = data

      for (const [key, _] of Object.entries(games)) {
        selector.option(key);
      }
      selector.selected(game_name)
      board = new Board2(start_x, 100)
      console.log(board)

      // parseBoard()
      draw()
    })
    .catch(error => console.log(error))

  for (var i = 0; i < pieces.length; i++)
    gamepieces.push(new Piece(pieces[i], i))

  noLoop()
}

function draw() {
  background(255)

  if (board != undefined) {
    console.log("called")
    board.draw()
  }

  for (var piece of gamepieces) piece.draw()

  // if (selected_piece != null) {
  //   let piece = gamepieces[selected_piece]

  //   noFill()
  //   strokeWeight(2)
  //   stroke(0, 255, 0)
  //   // circle(piece.x, piece.y, SQSIZE)

  //   let dx = piece.arr.shape[1] * SQSIZE / 2
  //   let dy = piece.arr.shape[0] * SQSIZE / 2

  //   let x = piece.x - dx + SQSIZE / 2
  //   let y = piece.y - dy + SQSIZE / 2
  //   circle(x, y, SQSIZE)

  //   let min_d = SQSIZE
  //   let min_x = 0
  //   let min_y = 0
  //   let idx = null
  //   for (let i = 0; i < 8; i++) {
  //     for (let j = 0; j < 8; j++) {
  //       let bx = gameboard.x + SQSIZE * i + SQSIZE / 2
  //       let by = gameboard.y + SQSIZE * j + SQSIZE / 2
  //       circle(bx, by, 10)
  //       if (dist(x, y, bx, by) < min_d) {
  //         min_d = dist(x, y, bx, by)
  //         min_x = bx
  //         min_y = by
  //         idx = [i, j]
  //       }
  //     }
  //   }

  //   console.log(idx)
  //   circle(min_x, min_y, 30)
  // }
}

/* ***************** CLASSES ******************* */

class Piece {
  constructor(ndarray, idx) {
    this.id = idx
    this.arr = nj.array(ndarray)
    this.placed = false

    // position (spawn based on index)
    var mag = 10
    let spawn_x = (idx % 6) * (width - mag) * 0.07 + (450)
    let spawn_y = (~~(idx / 6)) * (height - mag) * 0.05 + (height / 4 * 3)
    this.spawn_x = spawn_x
    this.spawn_y = spawn_y
    this.x = this.spawn_x
    this.y = this.spawn_y

    this.spawn_pos = createVector(spawn_x, spawn_y)
    this.pos = this.spawn_pos.copy()
  }

  // this is a method because shape changes after rotation operations
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

  // returns how big to draw squares
  getSqSize() {
    let size = selected_piece == this.id ? SQSIZE : MINI_SQSIZE
    size = this.placed ? SQSIZE : size
    return size
  }

  // returns a vector for center of drawn piece
  getCenter() {
    let size = this.getSqSize()
    let dx = this.arr.shape[1] * size / 2
    let dy = this.arr.shape[0] * size / 2
    return createVector(this.pos.x - dx, this.pos.y - dy)
  }

  draw() {

    noStroke()
    let size = this.getSqSize()
    let center = this.getCenter()

    for (var i = 0; i < this.arr.shape[0]; i++) {
      for (var j = 0; j < this.arr.shape[1]; j++) {

        switch (this.arr.get(i, j, 0)) {
          case RED: fill(RED_FILL); break;
          case BLACK: fill(BLACK_FILL); break;
          case BLUE: fill(BLUE_FILL); break;
          case YELLOW: fill(YELLOW_FILL); break;
          default: continue;
        }

        rect(center.x + j * size, center.y + i * size, size, size)

        if (this.mouseOver()) {
          fill(255, 255, 255, 50)
          rect(center.x + j * size, center.y + i * size, size, size)
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
      let min_d = SQSIZE
      let min_x = gameboard.x
      let min_y = gameboard.y
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

class Board2 {
  constructor(x, y) {
    this.pos = createVector(x, y)

    this.sol = nj.zeros([8, 8]).subtract(1)
    this.ref = nj.zeros([8, 8])
    this.parseBoard()
    this.notes = notes

    this.width = 8 * SQSIZE
    this.height = 8 * SQSIZE
  }

  draw() {
    noStroke()
    for (var i = 0; i < 8; i++) {
      for (var j = 0; j < 8; j++) {
        switch (this.ref.get(i, j)) {
          case RED: fill(RED_FILL + alpha); break;
          case BLACK: fill(BLACK_FILL + alpha); break;
          case BLUE: fill(BLUE_FILL + alpha); break;
          case YELLOW: fill(YELLOW_FILL + alpha); break;
        }
        rect(this.x + j * SQSIZE, this.y + i * SQSIZE, SQSIZE, SQSIZE)
      }
    }
  }

  // rotates board by 90 degrees
  rotate() {
    this.sol = nj.rot90(this.sol)
    this.ref = nj.rot90(this.ref)
  }

  // looks up selected game -> populates this.ref ndarray
  parseBoard() {
    let str = games[selector.value()]
    for (let i = 0; i < str.length; i++) {
      let row = Math.floor(i / 8);
      let col = i % 8;
      this.ref.set(row, col, parseInt(str[i]))
    }
  }

  mouseOver() {
    return (mouseX > this.x && mouseX < this.x + this.width &&
      mouseY > this.y && mouseY < this.y + this.height)
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
      let min_d = SQSIZE
      let min_x = gameboard.x
      let min_y = gameboard.y
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
