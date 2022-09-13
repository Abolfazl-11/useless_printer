mod fontmaker;
use fontmaker::Font;
use std::io;
use std::collections::HashMap; 
use serde_json;

fn main() {
    let mut action: String = String::new();

    println!("Enter your action: <Write line, Create new font> [w, c]");
    io::stdin().read_line(&mut action).
        expect("Failed to read input");

    if action.trim() == "c" || action.trim() == "C" || action.trim() == "Create new font" {
        fontmaker::create_font();
    }
    else if action.trim() == "w" || action.trim() == "W" || action.trim() == "Write line" {
        write_line();
    }
}

fn write_line() {
    let mut text_to_render = String::new();
    let mut font_name = String::new();
    let fonts_list: HashMap<String, String>;
    let mut text_lines: Vec<Vec<&str>> = Vec::new();
    let font: Font;
    let _alphabet: Vec<char> = String::from("abcdefghijklmnopqrstuvwxyz").chars().collect();
    
    println!("Enter the text to render(all in lowercase letters and no special characters): ");
    io::stdin().read_line(&mut text_to_render).expect("Failed to read line!");

    fonts_list = fontmaker::load_fonts_list();
    
    println!("Enter font name: ");

    io::stdin().read_line(&mut font_name)
        .expect("Failed to read font name!");

    font_name = font_name.trim().to_string();

    match fonts_list.get(&font_name) {
        Some(font_json) => {
            font = unserialize_font(&font_json);
            for c in text_to_render.trim().to_string().chars() {
                if c == ' '{
                    let mut space: Vec<&str> = Vec::new();
                    for _i in 0..text_lines[0].len() {
                        space.push(" ");
                    }
                    text_lines.push(space);
                    continue;
                }
                let index = _alphabet.iter().position(|&r| r == c).unwrap();
                let lines = font[&index].split('\n').collect();
                text_lines.push(lines);
            }

            print_text(text_to_render.len() - 1, text_lines[0].len(), text_lines);
        },
        None => println!("No Matches found!"),
    };
}

fn unserialize_font(font: &String) -> Font {
    let font_from_json: Font = serde_json::from_str(&font).unwrap();
    return font_from_json;
}

fn print_text(l: usize, fh: usize, text_split: Vec<Vec<&str>>) {
    let mut max_len = 0;

    for c in &text_split {
        for s in c {
            if s.len() > max_len {
                max_len = s.len();
            }
        }
    }

    for i in 0..fh {
        for j in 0..l {
            let text_part: &str = &(&text_split[j])[i].to_string();
            print!("{}", text_part);
            for _k in 0..(max_len - text_part.len() + 1) {
                print!(" ");
            }
        }
        println!("");
    }
}
