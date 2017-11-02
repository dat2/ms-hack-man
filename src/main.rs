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
  round: usize,
  field: Field,
}

impl Game {
  fn update(&mut self, settings: &Settings, key: &str, value: &str) {
    match key {
      "round" => self.round = value.parse().unwrap(),
      "field" => self.field = parse_field(settings, value),
      _ => {}
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
    cells: parsed_cells
      .chunks(settings.field_width)
      .map(|iter| iter.to_vec())
      .collect()
  }
}

#[derive(Clone, Debug)]
struct Cell {
  types: Vec<CellType>
}

fn parse_cell(cell: &str) -> Cell {
  Cell {
    types: cell.split(";").map(|cell_type| parse_cell_type(cell_type)).collect()
  }
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
struct Players {
  players: HashMap<usize, Player>
}

impl Players {
  fn update(&mut self, player_id: usize, key: &str, value: usize) {
    let mut player = self.players.entry(player_id).or_insert_with(Default::default);
    player.update(key, value);
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
enum MoveType {
  Up,
  Down,
  Left,
  Right,
  Pass,
}

impl fmt::Display for MoveType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
           "{}",
           match *self {
             MoveType::Up => "up",
             MoveType::Down => "down",
             MoveType::Left => "left",
             MoveType::Right => "right",
             MoveType::Pass => "pass",
           })
  }
}

#[derive(Debug)]
struct DropBomb {
  rounds: usize,
}

impl fmt::Display for DropBomb {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "drop_bomb {}", self.rounds)
  }
}

#[derive(Debug)]
struct Move {
  move_type: MoveType,
  drop_bomb: Option<DropBomb>,
}

impl fmt::Display for Move {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
           "{}{}",
           self.move_type,
           match self.drop_bomb {
             Some(ref db) => format!(";{}", db),
             None => String::new(),
           })
  }
}

fn choose_character(_time: usize) -> ChooseCharacter {
  ChooseCharacter::Bixie
}

fn calculate_move(settings: &Settings, game: &Game, players: &Players, _time: usize) -> Move {
  Move {
    move_type: MoveType::Pass,
    drop_bomb: None
  }
}

fn main() {
  let mut settings: Settings = Default::default();
  let mut game: Game = Default::default();
  let mut players: Players = Default::default();

  let stdin = io::stdin();
  loop {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input.pop();

    let commands: Vec<_> = input.split(" ").collect();

    match commands[0] {
      "settings" => settings.update(commands[1], commands[2]),
      "update" => match commands[1] {
        "game" => game.update(&settings, commands[2], commands[3]),
        pid => players.update(pid.parse().unwrap(), commands[2], commands[3].parse().unwrap())
      },
      "action" => match commands[1] {
        "character" => println!("{}", choose_character(commands[2].parse().unwrap())),
        "move" => println!("{}", calculate_move(&settings, &game, &players, commands[2].parse().unwrap())),
        _ => {}
      },
      _ => {}
    }
  }
}
