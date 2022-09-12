use std::io;
use std::fs::{File, read_dir, remove_file};
use std::io::prelude::*;
use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
//use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Font {
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
    h: String,
    i: String,
    j: String,
    k: String,
    l: String,
    m: String,
    n: String,
    o: String,
    p: String,
    q: String,
    r: String,
    s: String,
    t: String,
    u: String,
    v: String,
    w: String,
    x: String,
    y: String,
    z: String,
}

trait New {
    fn new() -> Font;
}

impl New for Font {
    fn new() -> Font{
        return Font {
            a: String::new(),
            b: String::new(),
            c: String::new(),
            d: String::new(),
            e: String::new(),
            f: String::new(),
            g: String::new(),
            h: String::new(),
            i: String::new(),
            j: String::new(),
            k: String::new(),
            l: String::new(),
            m: String::new(),
            n: String::new(),
            o: String::new(),
            p: String::new(),
            q: String::new(),
            r: String::new(),
            s: String::new(),
            t: String::new(),
            u: String::new(),
            v: String::new(),
            w: String::new(),
            x: String::new(),
            y: String::new(),
            z: String::new(),
        }
    }
}

//implementation indexing for Font Struct
impl Index<&'_ i8> for Font {
    type Output = String;
    fn index(&self, s: &i8) -> &String {
        match s{
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            4 => &self.e,
            5 => &self.f,
            6 => &self.g,
            7 => &self.h,
            8 => &self.i,
            9 => &self.j,
            10 => &self.k,
            11 => &self.l,
            12 => &self.m,
            13 => &self.n,
            14 => &self.o,
            15 => &self.p,
            16 => &self.q,
            17 => &self.r,
            18 => &self.s,
            19 => &self.t,
            20 => &self.u,
            21 => &self.v,
            22 => &self.w,
            23 => &self.x,
            24 => &self.y,
            25 => &self.z,
            _ => panic!("Unknown field {}", s),
        }
    }
}
impl IndexMut<&'_ i8> for Font {
    fn index_mut(&mut self, s: &i8) -> &mut String {
        match s{
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            4 => &mut self.e,
            5 => &mut self.f,
            6 => &mut self.g,
            7 => &mut self.h,
            8 => &mut self.i,
            9 => &mut self.j,
            10 => &mut self.k,
            11 => &mut self.l,
            12 => &mut self.m,
            13 => &mut self.n,
            14 => &mut self.o,
            15 => &mut self.p,
            16 => &mut self.q,
            17 => &mut self.r,
            18 => &mut self.s,
            19 => &mut self.t,
            20 => &mut self.u,
            21 => &mut self.v,
            22 => &mut self.w,
            23 => &mut self.x,
            24 => &mut self.y,
            25 => &mut self.z,
            _ => panic!("Unknowen field {}", s),
        }
    }
}

impl ToString for Font {
    fn to_string(&self) -> String {
        return format!("a: {}, b: {}, c: {}", self.a, self.b, self.c);
    }
}

// function to create a font and save it
pub fn create_font() {
    // variables
    let paths = read_dir("./src/fonts/").unwrap();
    let mut font: Font = Font::new();
    let mut name = String::new();
    let fonts_exist = check_fonts_list();
    let mut fonts;
    let mut fonts_list = HashMap::new();
    let mut fonts_list_str = String::new();

    fonts_list = load_fonts_list();
    remove_file("./src/fonts/fonts.txt").expect("Failed to remove font file");

    fonts = File::create("./src/fonts/fonts.txt").expect("Failed to create font list");

    // input from use:
    println!("Enter Fonts Name: ");
    io::stdin().read_line(&mut name).expect("Error reading line");
    name = name.trim().to_string();

    // loop throw every entry in ./src/fonts path to look for given name of the font
    for entry in paths {
        let mut entry_str = String::from(entry.as_ref().unwrap().path().into_os_string().into_string().unwrap());
        entry_str = entry_str.replace("./src/fonts/", "");

        if entry_str.eq(&name.trim()) {
            // check if the font is already exists
            match fonts_list.get(&name) {
                Some(_) => {
                    println!("Font already exists!");
                    return;
    
                },
                None => {},
            };

            // loop throw every file in given font directory and read the files
            for (i, l) in read_dir(entry.as_ref().unwrap().path().into_os_string().into_string().unwrap()).unwrap().enumerate() {
                let mut letter = File::open(l.as_ref().unwrap().path()).expect("Couldn't open file");
                let mut contents = String::new();

                letter.read_to_string(&mut contents)
                    .expect("Couldn't read file");

                let index: i8 = i.try_into().unwrap();

                font[&index] = contents;
            }
            break;
        }
    }
    println!("font: {}", font.to_string());

    let json_str = save_font(&font); 
    fonts_list.insert(name, json_str);

    fonts_list_str = hash_to_str(&fonts_list);
    println!("{}", fonts_list_str);
    fonts.write_all(&fonts_list_str.as_bytes())
        .expect("Error Writing fonts file");
}

// function to serialize the given font and return that
fn save_font(font: &Font) -> String {
    let font_ser = serde_json::to_string(font).unwrap();

    return font_ser;
}

// function to turn hashmap to string for saving to file
fn hash_to_str(fonts: &HashMap<String, String>) -> String {
    let mut fonts_list = String::new();
    for (f, s) in fonts {
        fonts_list.push_str(format!("{}::{}::", f, s).as_str());
    }

    return fonts_list;
}

fn str_to_hash(s: &str, h: &mut HashMap<String, String>) {
    let tokens: Vec<&str> = s.split("::").collect();
    for i in 0..(tokens.len() / 2) {
        h.insert(String::from(tokens[i*2]), String::from(tokens[(i*2) + 1]));
    }
}

fn check_fonts_list() -> bool {
    return std::path::Path::new("./src/fonts/fonts.txt").exists();
}

pub fn load_fonts_list() -> HashMap<String, String> {
    let fonts_exist = check_fonts_list();
    let mut fonts;
    let mut fonts_list_str = String::new();
    let mut fonts_list = HashMap::new();

    if fonts_exist {
        fonts = File::open("./src/fonts/fonts.txt").expect("Failed to open font file");
        fonts.read_to_string(&mut fonts_list_str).expect("Failed to read font file");
        str_to_hash(&fonts_list_str.as_str(), &mut fonts_list);
    }

    return fonts_list;
}
