use std::fs::{File, create_dir_all};
use std::io::Write;
use std::io::ErrorKind;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Sonnet {
  number: u8,
  lines: Vec<String>,
}

fn main() {
  let response: String = reqwest::blocking::get(
    "https://ocw.mit.edu/ans7870/6/6.006/s08/lecturenotes/files/t8.shakespeare.txt",
  )
  .unwrap()
  .text()
  .unwrap();

  let sonnets: Vec<Sonnet>  = select_sonnets(response.clone());
  let sonnets_json: String = serde_json::to_string_pretty(&sonnets).unwrap();

  if !Path::new("/data").exists() {
    match create_dir_all("data") {
      Ok(_) => (),
      Err(e) => {
        if e.kind() == ErrorKind::AlreadyExists {
          println!("Directory already exists");
        } else {
          println!("Error creating directory: {:?}", e);
        }
      }
    
    };
  }

  let mut file: File = File::create("data/sonnets.json").unwrap();
  file.write_all(sonnets_json.as_bytes()).unwrap();
  

  // println!("{}", response)
}

fn select_sonnets(text: String) -> Vec<Sonnet> {
  // split shakespear's entire works into just the sonnets as a string each
  let sonnet_strings: Vec<String> = text
    .split("THE SONNETS\n\nby William Shakespeare\n\n\n\n")             // splits the start of the sonnets
    .collect::<Vec<&str>>()
    .get(1)
    .unwrap()
    .split("THE END")                                               // splits the end of the sonnets
    .collect::<Vec<&str>>()
    .get(0)
    .unwrap()                                                                   // here I have just the sonnets
    .split("\n\n")                                                            // splits the sonnets
    .collect::<Vec<&str>>()
    .iter()
    .map(|s: &&str| s.trim().to_string())
    .collect();

  //remove the last element which is empty
  let sonnet_strings: Vec<String> = sonnet_strings
    .get(0..sonnet_strings.len() - 1)
    .unwrap()
    .iter()
    .map(|s: &String| s.to_string())
    .collect();

  // convert the sonnet strings into Sonnet structs
  let sonnets: Vec<Sonnet> = sonnet_strings
    .iter()
    .map(|sonnet: &String| {
      //extract the sonnet number as a string
      let sonnet_number_string: &str = sonnet.split("\n").next().unwrap_or("").trim();
      //convert the sonnet number to a u8
      let sonnet_number: u8 = match sonnet_number_string.parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
          println!("Error parsing sonnet number: {}", sonnet_number_string);
          return Sonnet {
            number: 0,
            lines: vec![],
            // return an empty Sonnet if parsing fails
          }
        }
      };

      let sonnet_lines: Vec<String> = sonnet
      .split("\n")
      .collect::<Vec<&str>>()
      .get(1..)
      .unwrap()
      .iter()
      .map(|line: &&str| line.to_string())
      .collect();

      Sonnet {
        number: sonnet_number,
        lines: sonnet_lines,
      }
    })
    .collect::<Vec<Sonnet>>();


  sonnets.clone()
}

