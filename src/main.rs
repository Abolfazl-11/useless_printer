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
    let mut fonts_list: HashMap<String, String> = HashMap::new();
    let font: Font;
    let _alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    
    println!("Enter the text to render: ");
    io::stdin().read_line(&mut text_to_render).expect("Failed to read line!");

    fonts_list = fontmaker::load_fonts_list();
    
    println!("Enter font name: ");

    io::stdin().read_line(&mut font_name)
        .expect("Failed to read font name!");

    font_name = font_name.trim().to_string();

    match fonts_list.get(&font_name) {
        Some(font_json) => {
            font = unserialize_font(&font_json);
        },
        None => println!("No Matches found!"),
    };
}

fn unserialize_font(font: &String) -> Font {
    let font_from_json: Font = serde_json::from_str(&font).unwrap();
    return font_from_json;
}
