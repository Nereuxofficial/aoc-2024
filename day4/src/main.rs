fn main() {
    let input = std::fs::read_to_string("day4/input.txt").unwrap();
    let character_map = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    find_xmas_in_character_map(&character_map);
}

enum Direction{
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}

impl Direction{
    /// Returns the modifiers to coordinates in the direction (x,y)
    fn get_modifiers(&self) -> (isize, isize){
        match self{
            Direction::Up => (0,-1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::UpRight => (1, -1),
            Direction::UpLeft => (-1, -1),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (-1, 1),
        }
    }
}

fn search_in_direction(accept: &[char], character_map: &[Vec<char>], direction: &Direction, mut x: usize, mut y: usize) -> Option<char>{
       if let Some((new_x,new_y)) = apply_modifiers(x, y, direction){
           x = new_x;
           y = new_y;
       }
       let current_char = character_map.get(y).map(|line| line.get(x).unwrap_or(&'Z')).unwrap_or(&'Z');
       if accept.contains(current_char) {
           return Some(*current_char);
       }
       None
}

fn apply_modifiers(x: usize, y: usize, direction: &Direction) -> Option<(usize,usize)>{
    let (x_mod, y_mod) = direction.get_modifiers();
    if let Some(x) = x.checked_add_signed(x_mod){
        if let Some(y) = y.checked_add_signed(y_mod){
            return Some((x,y));
        } 
    }
    None
}

fn find_xmas_in_character_map(character_map: &[Vec<char>]){
    let mut counter = 0usize;
    for y in 0..character_map.len(){
        for x in 0..character_map[y].len(){
            if character_map[y][x] == 'A' {
                if search_in_direction(&['M','S'], character_map, &Direction::UpLeft, x, y).is_some_and(|c| search_in_direction(&['M', 'S'], character_map, &Direction::DownRight, x, y).is_some_and(|b| b!=c)) && search_in_direction(&['M', 'S'], character_map, &Direction::UpRight, x, y).is_some_and(|c| search_in_direction(&['M','S'], character_map, &Direction::DownLeft, x, y).is_some_and(|b| b!=c)){
                    counter +=1;
                } 
            }
        }
    }
    println!("{}", counter);
}
