
const SQSIZE = 60
const MINI_SQSIZE = 10

const alpha = "88"
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
  selector.changed(() => {
    board.init()
    for (var piece of gamepieces) piece.respawn()
  });

  let start_x = (width - (8 * SQSIZE)) / 2

  fetch('../../boards/scraped-boards.json')
    .then(response => response.json())
    .then(data => {
      games = data

      for (const [key, _] of Object.entries(games)) {
        selector.option(key);
      }
      selector.selected(game_name)
      board = new Board(start_x, 100)
      console.log(board)

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
    board.draw()
  }

  for (var piece of gamepieces) piece.draw()

  // if (board != undefined) {
  //   board.drawDebugView()
  // }

  // if (selected_piece != null) {
  //   let piece = gamepieces[selected_piece]
  //   piece.drawDebugView()
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
    this.spawn_pos = createVector(spawn_x, spawn_y)

    // centered position of piece
    this.pos = this.spawn_pos.copy()
  }

  // this is a method because shape changes after rotation operations
  dims() {
    return [this.arr.shape[0], this.arr.shape[1]]
  }

  rotate() {
    if (selected_piece == this.id) {
      this.arr = nj.rot90(this.arr)
      this.pos.x = mouseX + (this.pos.y - mouseY)
      this.pos.y = mouseY + (this.pos.x - mouseX)
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

  // returns a vector for top left pixel of where to start drawing piece
  // this vector acts as the anchor which we use to try to snap to board
  getAnchor() {
    let size = this.getSqSize()
    let dx = this.arr.shape[1] * size / 2
    let dy = this.arr.shape[0] * size / 2
    return createVector(this.pos.x - dx, this.pos.y - dy)
  }

  // given anchor point, match pieces anchor point and recompute new center position
  snapTo(anchor) {
    let size = this.getSqSize()
    let dx = this.arr.shape[1] * size / 2
    let dy = this.arr.shape[0] * size / 2

    this.pos.x = anchor.x + dx
    this.pos.y = anchor.y + dy
  }

  respawn() {
    this.pos = this.spawn_pos.copy()
    this.placed = false
  }

  draw() {
    noStroke()
    let size = this.getSqSize()
    let start = this.getAnchor()

    for (var i = 0; i < this.arr.shape[0]; i++) {
      for (var j = 0; j < this.arr.shape[1]; j++) {

        switch (this.arr.get(i, j, 0)) {
          case RED: fill(RED_FILL); break;
          case BLACK: fill(BLACK_FILL); break;
          case BLUE: fill(BLUE_FILL); break;
          case YELLOW: fill(YELLOW_FILL); break;
          default: continue;
        }

        rect(start.x + j * size, start.y + i * size, size, size)

        if (this.mouseOver()) {
          fill(255, 255, 255, 50)
          rect(start.x + j * size, start.y + i * size, size, size)
        }
      }
    }
  }

  drawDebugView() {
    noFill()
    strokeWeight(2)
    stroke(0, 255, 0)
    circle(this.pos.x, this.pos.y, SQSIZE)

    let anchor = this.getAnchor()
    circle(anchor.x, anchor.y, SQSIZE)

    // find min distance cell
    let min_d = SQSIZE
    let min_x = board.pos.x
    let min_y = board.pos.y
    let idx = null

    for (let i = 0; i < 8; i++) {
      for (let j = 0; j < 8; j++) {
        let bx = board.pos.x + SQSIZE * i
        let by = board.pos.y + SQSIZE * j
        circle(bx, by, 10)
        let d = dist(anchor.x, anchor.y, bx, by)

        if (d < min_d) {
          min_d = d
          min_x = bx
          min_y = by
          idx = [i, j]
        }
      }
    }

    // console.log(idx)
    circle(min_x, min_y, 30)
  }

  mouseOver() {
    for (var i = 0; i < this.arr.shape[0]; i++) {
      for (var j = 0; j < this.arr.shape[1]; j++) {

        let size = this.getSqSize()
        let start = this.getAnchor()
        if (this.arr.get(i, j, 0) != EMPTY) {

          if (mouseX > start.x + j * size &&
            mouseX < start.x + (j + 1) * size &&
            mouseY > start.y + i * size &&
            mouseY < start.y + (i + 1) * size)
            return true;
        }
      }
    }
    return false;
  }

}

class Board {
  constructor(x, y) {
    this.pos = createVector(x, y)

    this.sol = nj.zeros([8, 8]).subtract(1)
    this.ref = nj.zeros([8, 8])
    this.parseBoardString()
    // this.notes = notes

    this.width = 8 * SQSIZE
    this.height = 8 * SQSIZE
  }

  init() {
    this.sol = nj.zeros([8, 8]).subtract(1)
    this.ref = nj.zeros([8, 8])
    this.parseBoardString()
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
        rect(this.pos.x + j * SQSIZE, this.pos.y + i * SQSIZE, SQSIZE, SQSIZE)
      }
    }
  }

  drawDebugView() {
    for (var i = 0; i < 8; i++) {
      for (var j = 0; j < 8; j++) {
        let tmp = `(${i},${j})=${this.sol.get(i, j)}`
        fill(0)

        text(tmp, this.pos.x + j * SQSIZE, this.pos.y + i * SQSIZE)
      }
    }
  }

  // rotates board state by 90 degrees
  rotate() {
    this.sol = nj.rot90(this.sol)
    this.ref = nj.rot90(this.ref)
  }

  // looks up selected game -> populates this.ref ndarray
  parseBoardString() {
    let str = games[selector.value()]
    for (let i = 0; i < str.length; i++) {
      let row = Math.floor(i / 8);
      let col = i % 8;
      this.ref.set(row, col, parseInt(str[i]))
    }
  }

  mouseOver() {
    return (mouseX > this.pos.x && mouseX < this.pos.x + this.width &&
      mouseY > this.pos.y && mouseY < this.pos.y + this.height)
  }

  // unmarks current selected piece from solution board
  undo() {
    for (let i = 0; i < 8; i++) {
      for (let j = 0; j < 8; j++) {
        if (this.sol.get(i, j) == selected_piece) {
          this.sol.set(i, j, -1)
        }
      }
    }
  }

  // snap piece location to closest legal board placement
  // returns boolean value on sucess or failure
  place() {

    // safety check
    if (selected_piece != null) {
      let piece = gamepieces[selected_piece]

      // only try to place piece if mouse is on the board
      if (this.mouseOver()) {
        let anchor = piece.getAnchor()

        // find min distance cell
        let min_d = SQSIZE
        let min_x = this.pos.x
        let min_y = this.pos.y
        let min_i = null
        let min_j = null

        for (let i = 0; i < 8; i++) {
          for (let j = 0; j < 8; j++) {
            let bx = this.pos.x + SQSIZE * i
            let by = this.pos.y + SQSIZE * j
            let d = dist(anchor.x, anchor.y, bx, by)

            if (d < min_d) {
              min_d = d
              min_x = bx
              min_y = by
              min_i = j
              min_j = i
            }
          }
        }

        // bounds check
        let [w, h] = piece.dims()

        let piece_fits = true

        if (min_i + w <= 8 || min_j + h <= 8) {
          // check all cells are empty
          for (let i = min_i; i < min_i + w; i++) {
            for (let j = min_j; j < min_j + h; j++) {
              if (piece.arr.get(i - min_i, j - min_j, 0) != 0) {
                if (this.sol.get(i, j) != -1) {
                  piece_fits = false
                }
              }

            }
          }
        }

        if (!piece_fits) {
          console.log(selected_piece, " does not fit")
          console.log(this.sol)
        }

        // place piece onto board's solution
        if (piece_fits) {
          for (let i = min_i; i < min_i + w; i++) {
            for (let j = min_j; j < min_j + h; j++) {
              if (piece.arr.get(i - min_i, j - min_j, 0) != 0) {
                this.sol.set(i, j, selected_piece)
              }
            }
          }

          piece.placed = true
          piece.snapTo(createVector(min_x, min_y))
          console.log("placed piece ", selected_piece)
          return true
        }

      }
      piece.respawn()
      board.undo()
    }
    return false
  }
}

/* ***************** HELPER FUNCTIONS ******************* */

const dist = (x1, y1, x2, y2) => {
  return Math.sqrt((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2))
}

/* ***************** EVENT HANDLERS ******************* */

function windowResized() {
  resizeCanvas(windowWidth, windowHeight);
}

function mouseDragged() {
  if (selected_piece != null) {
    let piece = gamepieces[selected_piece]
    piece.pos.x = mouseX
    piece.pos.y = mouseY
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
  if (selected_piece != null) {
    board.place()
    selected_piece = null
    redraw()
  }
}
