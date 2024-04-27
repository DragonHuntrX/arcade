use console::Key;
use console::Term;
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use snake::GameState;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::TrySendError;
use std::thread;
use std::time::Duration;

mod colors;
mod snake;
mod term2048;

use colors::*;

use std::env::args;

// const MESSAGE2048: &str = r"
// ██████████████████████████████████████████████████
//     ██████      ██████     ██   ██     █████
//          ██    ██  ████    ██   ██    ██   ██
//      █████     ██ ██ ██    ███████     █████
//     ██         ████  ██         ██    ██   ██
//     ███████     ██████          ██     █████
// ██████████████████████████████████████████████████";

const MESSAGE2048_ALT: &str = r"
    ██████      ██████     ██   ██     █████        
         ██    ██  ████    ██   ██    ██   ██       
     █████     ██ ██ ██    ███████     █████        
    ██         ████  ██         ██    ██   ██       
    ███████     ██████          ██     █████        ";

const COLOR_ORDER: [(&str, &str); 7] = [
    (WHITEB, BLACK),
    (YELLOWB, WHITE),
    (REDB, WHITE),
    (MAGENTAB, WHITE),
    (CYANB, WHITE),
    (BLUEB, WHITE),
    (GREENB, WHITE),
];

fn main() {
    let currently_avail = format!(
        "+{:->38}+\n|{:<38}|\n|{:<38}|\n|{:<38}|\n+{:->38}+",
        "", " Currently available:", "     0: 2048", "     1: Snake", ""
    );

    println!();
    println!("\tWelcome to the arcade!");
    println!("{}", currently_avail);

    print!(
        "{}",
        "Please press the key of the game you would\nlike to play (or press escape to exit):",
    );
    std::io::stdout().flush().unwrap();

    let stdout = Term::buffered_stdout();
    loop {
        if let Ok(key) = stdout.read_key() {
            println!();
            match key {
                Key::Char('0') => {
                    twenty_forty_eight();
                    break;
                }
                Key::Char('1') => {
                    snake();
                    break;
                }
                Key::Escape => break,
                _ => continue,
            }
        }
    }

    println!("Thank you for playing in the arcade!\n");
}

fn snake() {
    let mut game: snake::Game = snake::Game::new();

    let (tx, rx) = mpsc::channel::<snake::Direction>();

    let handle = thread::spawn(move || loop {
        let mut direc = game.dir;
        for dir in rx.try_iter() {
            direc = dir;
        }
        game.set_heading(direc);
        game.step();
        displaysnake(&game);
        if game.state == GameState::Over {
            println!("Game over!");
            break;
        }
        thread::sleep(Duration::from_millis(200))
    });

    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(key) = stdout.read_key() {
            let send_result = match key {
                Key::ArrowUp | Key::Char('w') => tx.send(snake::Direction::Up),
                Key::ArrowDown | Key::Char('s') => tx.send(snake::Direction::Down),
                Key::ArrowRight | Key::Char('d') => tx.send(snake::Direction::Right),
                Key::ArrowLeft | Key::Char('a') => tx.send(snake::Direction::Left),
                Key::Escape => break,
                _ => continue,
            };
            match send_result {
                Ok(_) => {}
                Err(mpsc::SendError(_)) => break,
            }
        }
    }
}

fn twenty_forty_eight() {
    let num_args: Vec<i32> = args()
        .filter(|x| x.parse::<i32>().is_ok())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut game;

    if num_args.len() >= 2 {
        game = term2048::Game::from_r_and_c(num_args[0] as usize, num_args[1] as usize);
    } else {
        game = term2048::Game::new();
    }

    display2048(&game);
    // println!("{}", game);

    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(key) = stdout.read_key() {
            match key {
                Key::ArrowUp | Key::Char('w') => game.step(term2048::Direction::Up),
                Key::ArrowDown | Key::Char('s') => game.step(term2048::Direction::Down),
                Key::ArrowRight | Key::Char('d') => game.step(term2048::Direction::Right),
                Key::ArrowLeft | Key::Char('a') => game.step(term2048::Direction::Left),
                Key::Escape => break,
                _ => continue,
            }
            if game.state == term2048::GameState::Moved {
                // println!("{}", game);
                display2048(&game);
            }
            if game.state == term2048::GameState::Over {
                // println!("{}", game);
                display2048(&game);
                break;
            }
        }
    }
}

fn displaysnake(game: &snake::Game) {
    let mut stdout = stdout();
    let mut output = CLEAR.to_string();
    for r in 0..game.rows {
        for c in 0..game.cols {
            let ind = r * game.cols + c;
            output += format!(
                "{}{}",
                match game.world[ind as usize] {
                    1 => GREEN.to_string() + "██",
                    2 => RED.to_string() + "██",
                    _ => DEFAULT.to_string() + "  ",
                },
                DEFAULT
            )
            .as_str();
        }
        output += "\n";
    }
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.queue(crossterm::style::Print(output)).unwrap();
    stdout.flush().unwrap();
}

fn display2048(game: &term2048::Game) {
    let spacer = format!("{}  {}", BWHITEB, DEFAULT);

    let mut output: String = CLEAR.to_string();
    output += MESSAGE2048_ALT;
    output += "\n";
    output += spacer.as_str();
    for _ in 0..game.board.cols {
        output += format!("{}{:^10}{}", BWHITEB, "", DEFAULT).as_str();
        output += spacer.as_str();
    }
    output += "\n";
    for r in 0..game.board.rows {
        for _ in 0..2 {
            output += spacer.as_str();
            for c in 0..game.board.cols {
                let element = game.board.state[game.board.to_index(r, c)];
                let color_theme = if element > 0 {
                    COLOR_ORDER[((element as f32).log2().floor() as i32 - 1) as usize % 7]
                } else {
                    (BBLACK, BLACK)
                };
                output += format!("{}{:^10}{}", color_theme.0, "", DEFAULT).as_str();
                output += spacer.as_str();
            }
            output += "\n";
        }

        output += spacer.as_str();
        for c in 0..game.board.cols {
            let element = game.board.state[game.board.to_index(r, c)];
            let color_theme = if element > 0 {
                COLOR_ORDER[((element as f32).log2().floor() as i32 - 1) as usize % 7]
            } else {
                (BBLACK, BLACK)
            };
            output += format!(
                "{}{}{:^10}{}",
                color_theme.1,
                color_theme.0,
                if game.board.state[game.board.to_index(r, c)] == 0 {
                    "".to_string()
                } else {
                    game.board.state[game.board.to_index(r, c)].to_string()
                },
                DEFAULT
            )
            .as_str();

            output += spacer.as_str();
        }
        output += "\n";

        for _ in 0..2 {
            output += spacer.as_str();
            for c in 0..game.board.cols {
                let element = game.board.state[game.board.to_index(r, c)];
                let color_theme = if element > 0 {
                    COLOR_ORDER[((element as f32).log2().floor() as i32 - 1) as usize % 7]
                } else {
                    (BBLACK, BLACK)
                };
                output += format!("{}{:^10}{}", color_theme.0, "", DEFAULT).as_str();
                output += spacer.as_str();
            }
            output += "\n";
        }

        output += spacer.as_str();
        for _ in 0..game.board.cols {
            output += format!("{}{:^10}{}", BWHITEB, "", DEFAULT).as_str();
            output += spacer.as_str();
        }
        output += "\n";
    }
    write!(std::io::stdout(), "{}", output).unwrap();
}

/* Graveyard

struct Square {
    bcolor: String,
    fcolor: String,
}

for &element in &game.board.state {
        squares.push(match element {
            0 => Square {
                bcolor: BLACKB.to_string(),
                fcolor: BWHITE.to_string(),
            },
            2 => Square {
                bcolor: YELLOWB.to_string(),
                fcolor: BWHITE.to_string(),
            },
            4 => Square {
                bcolor: CYANB.to_string(),
                fcolor: BWHITE.to_string(),
            },
            8 => Square {
                bcolor: GREENB.to_string(),
                fcolor: BWHITE.to_string(),
            },
            16 => Square {
                bcolor: REDB.to_string(),
                fcolor: BWHITE.to_string(),
            },
            32 => Square {
                bcolor: BLUEB.to_string(),
                fcolor: BWHITE.to_string(),
            },
            64 => Square {
                bcolor: BYELLOWB.to_string(),
                fcolor: BLACK.to_string(),
            },
            128 => Square {
                bcolor: BCYANB.to_string(),
                fcolor: BBLACK.to_string(),
            },
            256 => Square {
                bcolor: BGREENB.to_string(),
                fcolor: BLACK.to_string(),
            },
            512 => Square {
                bcolor: BREDB.to_string(),
                fcolor: BLACK.to_string(),
            },
            1024 => Square {
                bcolor: BBLUEB.to_string(),
                fcolor: BLACK.to_string(),
            },
            2048 => Square {
                bcolor: MAGENTAB.to_string(),
                fcolor: BWHITE.to_string(),
            },
            _ => Square {
                bcolor: WHITEB.to_string(),
                fcolor: BLACK.to_string(),
            },
        });
    }*/
