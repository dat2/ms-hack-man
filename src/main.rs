use std::collections::HashMap;
use std::io;
use std::fmt;

#[derive(Debug, Default)]
struct Settings {
  timebank: usize,
  time_per_move: usize,
  player_names: Vec<String>,
  my_bot: String,
  my_bot_id: usize,
  field_width: usize,
  field_height: usize,
  max_rounds: usize,
}

impl Settings {
  fn update(&mut self, key: &str, value: &str) {
    match key {
      "timebank" => self.timebank = value.parse().unwrap(),
      "time_per_move" => self.time_per_move = value.parse().unwrap(),
      "player_names" => self.player_names = value.split(",").map(String::from).collect(),
      "your_bot" => self.my_bot = value.to_owned(),
      "your_botid" => self.my_bot_id = value.parse().unwrap(),
      "field_width" => self.field_width = value.parse().unwrap(),
      "field_height" => self.field_height = value.parse().unwrap(),
      "max_rounds" => self.max_rounds = value.parse().unwrap(),
      _ => {}
    }
  }
}

#[derive(Debug, Default)]
struct Game {
  settings: Settings,
  round: usize,
  field: Field,
  players: HashMap<String, Player>,
}

impl Game {
  fn update_settings(&mut self, key: &str, value: &str) {
    self.settings.update(key, value);
    for pid in &self.settings.player_names {
      self.players.insert(pid.to_owned(), Default::default());
    }
  }

  fn update(&mut self, update_type: &str, key: &str, value: &str) {
    match update_type {
      "game" => {
        match key {
          "round" => self.round = value.parse().unwrap(),
          "field" => self.field = parse_field(&self.settings, value),
          _ => {}
        }
      }
      pid => {
        let mut player = self.players.entry(pid.to_owned()).or_insert_with(Default::default);
        player.update(key, value.parse().unwrap());
      }
    }
  }
}

#[derive(Debug, Default)]
struct Field {
  cells: Vec<Vec<Cell>>,
}

fn parse_field(settings: &Settings, field: &str) -> Field {
  let parsed_cells: Vec<_> = field.split(",")
    .map(|cell| parse_cell(cell))
    .collect();
  Field {
    cells: parsed_cells.chunks(settings.field_width)
      .map(|iter| iter.to_vec())
      .collect(),
  }
}

#[derive(Clone, Debug)]
struct Cell {
  types: Vec<CellType>,
}

fn parse_cell(cell: &str) -> Cell {
  Cell { types: cell.split(";").map(|cell_type| parse_cell_type(cell_type)).collect() }
}

#[derive(Clone, Debug)]
enum CellType {
  Nothing,
  Inaccessible,
  Player { id: usize },
  BugSpawnPoint { rounds_before_spawn: usize },
  GateLeft,
  GateRight,
  Bug { ai_type: usize },
  Mine { rounds_before_explode: usize },
  PickUpMine,
  CodeSnippet,
}

fn parse_cell_type(cell_type: &str) -> CellType {
  match cell_type {
    "." => CellType::Nothing,
    "x" => CellType::Inaccessible,
    "S" => CellType::BugSpawnPoint { rounds_before_spawn: 0 },
    "Gl" => CellType::GateLeft,
    "Gr" => CellType::GateRight,
    "B" => CellType::PickUpMine,
    "C" => CellType::CodeSnippet,
    c => {
      match c.chars().nth(0).unwrap() {
        'P' => CellType::Player { id: c[1..].parse().unwrap() },
        'S' => CellType::BugSpawnPoint { rounds_before_spawn: c[1..].parse().unwrap() },
        'E' => CellType::Bug { ai_type: c[1..].parse().unwrap() },
        'B' => CellType::Mine { rounds_before_explode: c[1..].parse().unwrap() },
        _ => panic!("invalid cell type!"),
      }
    }
  }
}

#[derive(Debug, Default)]
struct Player {
  snippets: usize,
  bombs: usize,
}

impl Player {
  fn update(&mut self, key: &str, value: usize) {
    match key {
      "snippets" => self.snippets = value,
      "bombs" => self.bombs = value,
      _ => {}
    }
  }
}

#[derive(Debug)]
enum ChooseCharacter {
  Bixie,
  Bixiette,
}

impl fmt::Display for ChooseCharacter {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
           "{}",
           match *self {
             ChooseCharacter::Bixie => "bixie",
             ChooseCharacter::Bixiette => "bixiette",
           })
  }
}

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
           "{}",
           match *self {
             Direction::Up => "up",
             Direction::Down => "down",
             Direction::Left => "left",
             Direction::Right => "right",
           })
  }
}

#[derive(Debug)]
enum Move {
  Direction { direction: Direction },
  DropBomb { direction: Direction, rounds: usize },
  Pass,
}

impl fmt::Display for Move {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Move::Direction { ref direction } => write!(f, "{}", direction),
      Move::DropBomb { ref direction, ref rounds } => {
        write!(f, "{};drop_bomb {}", direction, rounds)
      }
      Move::Pass => write!(f, "pass"),
    }
  }
}

fn action_character(_time: usize) -> ChooseCharacter {
  ChooseCharacter::Bixie
}

fn action_move(game: &Game, _time: usize) -> Move {
  Move::Pass
}

fn main() {
  let mut game: Game = Default::default();

  let stdin = io::stdin();
  loop {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input.pop();

    let commands: Vec<_> = input.split(" ").collect();

    match commands[0] {
      "settings" => game.update_settings(commands[1], commands[2]),
      "update" => game.update(commands[1], commands[2], commands[3]),
      "action" => {
        match commands[1] {
          "character" => println!("{}", action_character(commands[2].parse().unwrap())),
          "move" => println!("{}", action_move(&game, commands[2].parse().unwrap())),
          _ => {}
        }
      }
      _ => {}
    }
  }
}
