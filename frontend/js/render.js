
const SQSIZE = 40
const MINI_SQSIZE = 10

const alpha = "55"
const RED_FILL = "#db3939"
const BLACK_FILL = "#212121"
const BLUE_FILL = "#2059a8"
const YELLOW_FILL = "#ffde24"

let cnv

let gamepieces = []
let board
let games
let game_name = "australian-emu"

let selector
let selector_size = 200;
let selected_piece = null

let rotateRegion
let flipRegion

let mobile_offset

/* ***************** INIT + MAIN LOOP ******************* */

function setup() {

  // canvas HTML object
  cnv = createCanvas(windowWidth, windowHeight);
  cnv.position(0, 0, 'fixed')

  // game selector dropdown

  selector = createSelect();
  selector.size(selector_size);
  selector.position((width - selector_size) / 2, 70, 'fixed')
  selector.changed(() => {
    board.init()
    for (var piece of gamepieces) piece.respawn()
    redraw()
  });

  let h = 100
  rotateRegion = new HoverRegion(width / 2, height - h, width / 2, h, 'rotate')
  flipRegion = new HoverRegion(0, height - h, width / 2, h, 'flip')

  let start_x = (width - (8 * SQSIZE)) / 2
  mobile_offset = isMobile() ? -80 : 0

  fetch('./boards.json')
    .then(response => response.json())
    .then(data => {
      games = data

      for (const [key, _] of Object.entries(games)) {
        selector.option(key);
      }
      selector.selected(game_name)
      board = new Board(start_x, 100)

      draw()
    })
    .catch(error => console.log(error))

  for (var i = 0; i < pieces.length; i++)
    gamepieces.push(new Piece(pieces[i], i))

  noLoop()
}

function init() {
  let start_x = (width - (8 * SQSIZE)) / 2
  board.pos.x = start_x
  selector.position((width - selector_size) / 2, 70, 'fixed')
}

function draw() {
  background(255)

  if (board != undefined) {
    board.draw()
  }

  rotateRegion.draw()
  flipRegion.draw()

  for (var piece of gamepieces) piece.draw()

  //   if (board != undefined) {
  //     board.drawDebugView()
  //   }

  //   if (selected_piece != null) {
  //     let piece = gamepieces[selected_piece]
  //     piece.drawDebugView()
  //   }
}


/* ***************** CLASSES ******************* */

class Piece {
  constructor(ndarray, idx) {
    this.id = idx
    this.arr = nj.array(ndarray)
    this.placed = false

    // position (spawn based on index)
    var mag = 50
    let spawn_x = mag + (idx % 6) * (width - mag) / 6
    let spawn_y = (~~(idx / 6)) * (height - mag) * 0.05 + (height * 0.70)
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
    redraw()
  }

  mouseOver() {
    let x = mouseX
    let y = mouseY + mobile_offset
    return (x > this.pos.x && x < this.pos.x + this.width &&
      y > this.pos.y && y < this.pos.y + this.height)
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
        console.log(min_i + w, min_j + h)

        if (min_i + w <= 8 && min_j + h <= 8) {
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
        } else {
          piece_fits = false
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

class HoverRegion {
  constructor(x, y, w, h, fn) {
    this.pos = createVector(x, y)
    this.w = w
    this.h = h

    this.start = millis()
    this.delay = 500
    this.fn = fn
  }

  draw() {
    noStroke()
    fill(240)
    rect(this.pos.x, this.pos.y, this.w, this.h)
    fill(0)
    textAlign(CENTER)
    text(`hover to ${this.fn}`, this.pos.x + this.w / 2, this.pos.y + this.h / 2)
  }

  mouseOver() {
    return (mouseX > this.pos.x && mouseX < this.pos.x + this.w &&
      mouseY > this.pos.y && mouseY < this.pos.y + this.h)
  }

  poll() {
    if (this.mouseOver() && millis() >= this.start + this.delay) {
      let piece = gamepieces[selected_piece]
      if (this.fn == 'rotate') {
        piece.rotate()
      } else {
        piece.flip()
      }
      this.start = millis()
    }
  }
}

/* ***************** HELPER FUNCTIONS ******************* */

const dist = (x1, y1, x2, y2) => {
  return Math.sqrt((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2))
}

const isMobile = () => {
  var check = false;
  (function(a) {
    if (/(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|iris|kindle|lge |maemo|midp|mmp|mobile.+firefox|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows ce|xda|xiino/i.test(a) || /1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s\-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|\-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw\-(n|u)|c55\/|capi|ccwa|cdm\-|cell|chtm|cldc|cmd\-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc\-s|devi|dica|dmob|do(c|p)o|ds(12|\-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(\-|_)|g1 u|g560|gene|gf\-5|g\-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd\-(m|p|t)|hei\-|hi(pt|ta)|hp( i|ip)|hs\-c|ht(c(\-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i\-(20|go|ma)|i230|iac( |\-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc\-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|\-[a-w])|libw|lynx|m1\-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m\-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(\-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)\-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|\-([1-8]|c))|phil|pire|pl(ay|uc)|pn\-2|po(ck|rt|se)|prox|psio|pt\-g|qa\-a|qc(07|12|21|32|60|\-[2-7]|i\-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h\-|oo|p\-)|sdk\/|se(c(\-|0|1)|47|mc|nd|ri)|sgh\-|shar|sie(\-|m)|sk\-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h\-|v\-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl\-|tdg\-|tel(i|m)|tim\-|t\-mo|to(pl|sh)|ts(70|m\-|m3|m5)|tx\-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|\-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(\-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas\-|your|zeto|zte\-/i.test(a.substr(0, 4)))
      check = true;
  })(navigator.userAgent || navigator.vendor || window.opera);
  return check;
};

/* ***************** EVENT HANDLERS ******************* */

function windowResized() {
  console.log("called")
  resizeCanvas(windowWidth, windowHeight);
  init()
}

function mouseDragged() {
  if (selected_piece != null) {
    let piece = gamepieces[selected_piece]

    piece.pos.x = mouseX
    piece.pos.y = mouseY + mobile_offset

    rotateRegion.poll()
    flipRegion.poll()
  }
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
    let piece_ = gamepieces[selected_piece]
    piece_.pos.x = mouseX
    piece_.pos.y = mouseY + mobile_offset
  }
  redraw()
}

function mouseReleased() {
  if (selected_piece != null) {
    board.place()
    selected_piece = null
  }
  redraw()
}

document.addEventListener('contextmenu', event => event.preventDefault());
