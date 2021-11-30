use std::collections::VecDeque;
use std::collections::HashMap;
use rand::Rng;
use std::io::{stdin, stdout, Read, Write};

const HASH: &str = "# ";
const APPLE: &str = "$ ";
const HEAD: &str = "@ ";
const BODY: &str = "o ";
const EMPTY: &str = "  ";

const RIGHT: &str = "> ";
const DOWN: &str = "v ";
const LEFT: &str = "< ";
const UP: &str = "^ ";

//Top left playable square is (0, 0)
//Bottom right playable square is (w-2, h-2) 
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord(usize, usize);

enum Square{
    Empty,
    Apple,
    Head,
    Body,
}

enum Heading{
    Right,
    Down,
    Left,
    Up,
}

struct GameInfo{
    apple: Coord,
    head: Coord,
    body: VecDeque<Coord>,
    pop: HashMap<Coord, Square>,
    facing: Heading,
    size: Coord,  
}

//main game loop.
fn main() {
    let w: usize = 10;
    let h: usize = 10;
    
    let mut game = generate_initial_board(w, h);

    print_board(w, h, &game);

    loop {
        game = advance(game);
        print_board(w, h, &game);
    }
}

//TODO: redo this. Copied from the internet and it's a little buggy.
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn end_game() {
    println!("Game Over!");
    println!("Thank you for playing!");
    std::process::exit(0);
}

fn advance(mut game: GameInfo) -> GameInfo{
    let new_head: Coord;
    let Coord(x, y) = game.head;

    new_head = match game.facing {
        Heading::Right => Coord(x+1, y),
        Heading::Down => Coord(x, y+1),
        Heading::Left => Coord(x-1, y),
        Heading::Up => Coord(x, y-1),
    };

    //Checking for game over.
    if new_head.0 < 1 ||
        new_head.0 > (game.size.0 - 1) ||
        new_head.1 < 1 ||
        new_head.1 > (game.size.1 - 1){
            end_game()
        }
    
    match game.pop.get(&new_head) {
        Some(Square::Body) => end_game(),
        _ => (),
    } 

    game.body.push_back(game.head);
    game.pop.insert(game.head, Square::Body);

    game.head = new_head;
    game.pop.insert(game.head, Square::Head);
    

    //checking if apple is caught.
    if game.apple == new_head{
        //TODO: only generate on non body squares.
        let rand_x = rand::thread_rng().gen_range(1..(game.size.0-1));
        let rand_y = rand::thread_rng().gen_range(1..(game.size.1-1));

        game.apple = Coord(rand_x, rand_y);
        game.pop.insert(game.apple, Square::Apple);
    } else {
        match game.body.pop_front() {
            Some(X) => game.pop.insert(X, Square::Empty),
            _ => None, 
        };
    };
    
    println!("New head: {}, {}", new_head.0, new_head.1);

    return game;
}

//Initial board generation.
//Positions the player in the top left and the apple in the bottom right.
fn generate_initial_board(w: usize, h: usize) -> GameInfo { 
    let mut game = GameInfo{
        apple: Coord(w - 2, h - 2),
        head: Coord(1, 1),
        body: VecDeque::new(),
        pop: HashMap::new(),
        facing: Heading::Right,
        size: Coord(w, h),
    };

    game.pop.insert(Coord(1, 1), Square::Head);
    game.pop.insert(Coord(w-2, h-2), Square::Apple);

    return game;
}

//Given the information about the game, prints the board in ascii to stdout.
fn print_board(width: usize, height: usize, game: &GameInfo){
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
                        Some(Square::Head) => sq_char = {
                            match game.facing{
                                Heading::Left => LEFT,
                                Heading::Down => DOWN,
                                Heading::Right => RIGHT,
                                Heading::Up => UP,
                            }
                        },
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