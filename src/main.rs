use std::collections::VecDeque;
use std::collections::HashMap;

const HASH: &str = "# ";
const APPLE: &str = "$ ";
const HEAD: &str = "@ ";
const BODY: &str = "o ";
const EMPTY: &str = "  ";

//Top left playable square is (0, 0)
//Bottom right playable square is (w-2, h-2) 
#[derive(PartialEq, Eq, Hash)]
struct Coord(usize, usize);

enum Square{
    Empty,
    Apple,
    Head,
    Body,
}

enum Heading{
    Down,
    Up,
    Right,
    Left,
}

struct GameInfo{
    apple: Coord,
    head: Coord,
    body: VecDeque<Coord>,
    pop: HashMap<Coord, Square>,
    facing: Heading,  
}

//main game loop.
fn main() {
    let h: usize = 10;
    let w: usize = 10;
    
    let mut game = generate_initial_board(h, w);

    print_board(h, w, game);
}

//Initial board generation.
//Positions the player in the top left and the apple in the bottom right.
fn generate_initial_board(h: usize, w: usize) -> GameInfo { 
    let mut game = GameInfo{
        apple: Coord(w - 2, h - 2),
        head: Coord(1, 1),
        body: VecDeque::new(),
        pop: HashMap::new(),
        facing: Heading::Right,
    };

    game.pop.insert(Coord(1,1), Square::Head);
    game.pop.insert(Coord(w-2, h-2), Square::Apple);

    return game;
}

//Given the information about the game, prints the board in ascii to stdout.
fn print_board(height: usize, width: usize, game: GameInfo){
    let top_bound = HASH.repeat(width);

    for i in 0..height {
        if i == 0 || i == (height-1) {
            println!("{}", top_bound);
        } else {
            let mut lin = "".to_owned();

            for j in 0..width {
                let sq_char: &str;

                if j == 0 || j == (width - 1){
                    sq_char = HASH;
                } else {
                    let c = Coord(j,i);

                    match game.pop.get(&c){
                        Some(Square::Empty) => sq_char = EMPTY,
                        Some(Square::Body) => sq_char = BODY,
                        Some(Square::Head) => sq_char = HEAD,
                        Some(Square::Apple) => sq_char = APPLE,
                        None => sq_char = EMPTY,
                    }
                }

                lin.push_str(sq_char);
            }

            println!("{}", lin);
        }
    }
}