
// Namespace
const EMPTY = 0
const BLACK = 1
const RED = 2
const YELLOW = 3
const BLUE = 4

// Shorthand Namespace
const E = EMPTY
const B = BLACK
const R = RED
const Y = YELLOW
const b = BLUE

const pieces = [

    // Monominoes
    [[[RED, BLACK]]],
    [[[BLACK, BLUE]]],

    // Dominoes
    [[[RED, BLACK], [BLACK, BLUE]]],

    // Trominoes
    [[[RED, YELLOW], [BLACK, BLACK], [RED, BLUE]]],
    [[[BLACK, BLACK], [RED, YELLOW], [BLACK, BLACK]]],
    [[[RED, BLUE], [EMPTY, EMPTY]], [[BLACK, BLACK], [RED, YELLOW]]],
    [[[BLACK, BLACK], [EMPTY, EMPTY]], [[RED, YELLOW], [BLACK, BLACK]]],

    // Tetrominoes
    [[[RED, BLUE], [BLACK, BLACK], [RED, YELLOW], [BLACK, BLACK]]],
    [[[RED, BLACK], [BLACK, BLUE]], [[BLACK, YELLOW], [RED, BLACK]]],
    [[[BLACK, BLACK], [RED, YELLOW], [EMPTY, EMPTY]], [[EMPTY, EMPTY], [BLACK, BLACK], [RED, BLUE]]],
    [[[EMPTY, EMPTY], [RED, BLACK], [BLACK, YELLOW]], [[RED, BLACK], [BLACK, BLUE], [EMPTY, EMPTY]]],
    [[[EMPTY, EMPTY], [RED, BLACK], [EMPTY, EMPTY]], [[RED, BLACK], [BLACK, BLUE], [RED, BLACK]]],
    [[[EMPTY, EMPTY], [BLACK, YELLOW], [EMPTY, EMPTY]], [[BLACK, YELLOW], [RED, BLACK], [BLACK, BLUE]]],
    [[[BLACK, BLUE], [EMPTY, EMPTY], [EMPTY, EMPTY]], [[RED, BLACK], [BLACK, YELLOW], [RED, BLACK]]],
    [[[RED, YELLOW], [EMPTY, EMPTY], [EMPTY, EMPTY]], [[BLACK, BLACK], [RED, BLUE], [BLACK, BLACK]]],
    [[[EMPTY, EMPTY], [EMPTY, EMPTY], [BLACK, BLACK]], [[RED, YELLOW], [BLACK, BLACK], [RED, BLUE]]],
    [[[EMPTY, EMPTY], [EMPTY, EMPTY], [RED, BLACK]], [[BLACK, BLUE], [RED, BLACK], [BLACK, YELLOW]]],

    // Octominoes
    [[[RED, BLACK], [BLACK, YELLOW], [RED, BLACK], [BLACK, BLUE], [RED, BLACK], [BLACK, YELLOW], [RED, BLACK], [BLACK, BLUE]]]
]

const puzzles = {

    "new":
        [[0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0]],

    "classic":
        [[R, B, R, B, R, B, R, B],
        [B, R, B, R, B, R, B, R],
        [R, B, R, B, R, B, R, B],
        [B, R, B, R, B, R, B, R],
        [R, B, R, B, R, B, R, B],
        [B, R, B, R, B, R, B, R],
        [R, B, R, B, R, B, R, B],
        [B, R, B, R, B, R, B, R]],

    "sun-shower":
        [[B, b, B, Y, B, b, B, Y],
        [R, B, Y, B, b, B, Y, B],
        [B, R, B, b, B, Y, B, b],
        [R, B, R, B, Y, B, b, B],
        [B, R, B, R, B, b, B, Y],
        [R, B, R, B, R, B, Y, B],
        [B, R, B, R, B, R, B, b],
        [R, B, R, B, R, B, R, B]],

    "magic":
        [[B, B, R, B, R, B, R, B],
        [R, B, R, R, B, R, B, R],
        [B, R, B, B, R, B, R, B],
        [R, B, R, R, B, R, B, R],
        [B, R, B, R, R, B, R, B],
        [R, B, R, B, R, B, R, R],
        [B, R, B, R, B, R, B, B],
        [R, B, R, B, R, B, R, B]],

    "elephant":
        [[B, R, B, R, B, R, B, R],
        [R, B, R, B, R, B, R, B],
        [B, R, B, R, B, R, B, R],
        [R, B, R, R, R, B, R, B],
        [R, B, B, B, R, R, B, R],
        [B, R, B, B, B, B, R, B],
        [R, R, R, B, B, B, R, R],
        [B, R, R, B, R, B, R, B]],

    "chaos":
        [[B, B, Y, b, B, B, R, B],
        [Y, B, B, B, Y, R, Y, R],
        [R, b, R, Y, B, b, B, B],
        [B, R, B, B, b, B, b, R],
        [R, B, R, B, B, Y, B, Y],
        [b, B, b, B, Y, b, Y, B],
        [B, B, R, Y, B, B, R, b],
        [Y, R, B, B, b, R, B, R]],
}