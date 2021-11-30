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
    EMPTY,
    APPLE,
    HEAD,
    BODY,
}

struct GameInfo{
    apple: Coord,
    head: Coord,
    body: VecDeque<Coord>,
    pop: HashMap<Coord, Square>,  
}

fn main() {
    let h: usize = 10;
    let w: usize = 10;
    
    let mut game = GameInfo{
        apple: Coord(w - 2, h - 2),
        head: Coord(1, 1),
        body: VecDeque::new(),
        pop: HashMap::new(),
    };

    game.pop.insert(Coord(1,1), Square::HEAD);
    game.pop.insert(Coord(w-2, h-2), Square::APPLE);

    print_board(h, w, game);
}

fn print_board(height: usize, width: usize, game: GameInfo){
    let top = HASH.repeat(width);
    
    println!("{}", top);

    let mut i = 1;
    while i < (height-1) {
        let mut j = 0;
        let mut lin = "".to_owned();

        while j < width {
            let sq_char: &str;

            if j == 0 || j == (width - 1){
                sq_char = HASH;
            } else {
                let c = Coord(j,i);

                match game.pop.get(&c){
                    Some(Square::EMPTY) => sq_char = EMPTY,
                    Some(Square::BODY) => sq_char = BODY,
                    Some(Square::HEAD) => sq_char = HEAD,
                    Some(Square::APPLE) => sq_char = APPLE,
                    None => sq_char = EMPTY,
                }

            }

            lin.push_str(sq_char);
            
            j += 1;
        }

        println!("{}", lin);

        i += 1;
    }

    println!("{}", top);
}