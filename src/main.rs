use std::collections::{VecDeque, HashMap};
use rand::Rng;
use std::io::{stdin, stdout, Read, Write};
use std::thread;
use std::time;
use std::str;

const HASH: &str = "# ";
const APPLE: &str = "$ ";
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

#[derive(PartialEq, Eq)]
enum Heading{
    Right,
    Down,
    Left,
    Up,
    None,
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
    println!("Welcome to Snake in Rust!");
    println!("To move, enter wasd between frames.");

    let w: usize = 10;
    let h: usize = 10;
    
    let mut game = generate_initial_board(w, h);

    print_board(w, h, &game);

    loop {
        let head = pause();
        if head != Heading::None { 
            game.facing = head; 
        }

        game = advance(game);
        print_board(w, h, &game);
    }
}

fn pause() -> Heading {
    let mut buffer = [0; 10];
    let mut stdout = stdout();

    stdin().read(&mut buffer).unwrap();

    let input = str::from_utf8(&buffer).unwrap().trim();

    let delay = time::Duration::from_millis(100);
    thread::sleep(delay);

    let head: Heading = match input.chars().nth(0).unwrap() {
        'd' => Heading::Right,
        's' => Heading::Down,
        'a' => Heading::Left,
        'w' => Heading::Up,
        _ => Heading::None,
    };

    return head;
}

fn end_game() {
    println!("Game Over!");
    println!("Thank you for playing!");
    std::process::exit(0);
}

//advances the game by a frame, performing essential game checks.
fn advance(mut game: GameInfo) -> GameInfo{
    let new_head: Coord;
    let Coord(x, y) = game.head;

    new_head = match game.facing {
        Heading::Down => Coord(x, y+1),
        Heading::Left => Coord(x-1, y),
        Heading::Up => Coord(x, y-1),
        _ => Coord(x+1, y),
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
        let mut rand_x: usize;
        let mut rand_y: usize;

        loop {
            rand_x = rand::thread_rng().gen_range(1..(game.size.0-1));
            rand_y = rand::thread_rng().gen_range(1..(game.size.1-1));
            let c = Coord(rand_x, rand_y);

            match game.pop.get(&c) {
                None => break,
                Some(Square::Empty) => break,
                _ => continue,
            }
        }

        game.apple = Coord(rand_x, rand_y);
        game.pop.insert(game.apple, Square::Apple);
    } else {
        match game.body.pop_front() {
            Some(x) => game.pop.insert(x, Square::Empty),
            _ => None, 
        };
    };

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
                                Heading::Down => DOWN,
                                Heading::Left => LEFT,
                                Heading::Up => UP,
                                _ => RIGHT,
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