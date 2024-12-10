use std::collections::HashSet;

use crate::common::day::Day;
pub struct Day6 {
    pub input: String,
}

impl Day for Day6 {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut world = World::from(&self.input);
        let mut visited_coords: HashSet<Coord> = HashSet::new();
        visited_coords.insert(world.player.coord);
        loop {
            match world.move_player() {
                PlayerMoveResult::Location(coord) => _ = visited_coords.insert(coord),
                PlayerMoveResult::Turned => {}
                PlayerMoveResult::FellOfWorld => {
                    break;
                }
            }
        }
        Ok(visited_coords.len().to_string())
    }

    fn part2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;
        sum += 1;
        Ok(sum.to_string())
    }
}

#[derive(Debug)]
enum Position {
    Free,
    Obstacle,
}

#[derive(Debug)]
enum FaceDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Player {
    coord: Coord,
    dir: FaceDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct World {
    space: Vec<Vec<Position>>,
    player: Player,
}

enum PlayerMoveResult {
    Location(Coord),
    Turned,
    FellOfWorld,
}

impl Player {
    fn set_pos(&mut self, x: usize, y: usize) {
        self.coord.x = x as i32;
        self.coord.y = y as i32;
    }
}

impl World {
    fn from(input: &str) -> Self {
        let mut player = Player {
            coord: Coord { x: -1, y: -1 },
            dir: FaceDirection::Up,
        };
        let rows = input.lines().count();
        let cols = input
            .lines()
            .next()
            .expect("Expected at least one line")
            .chars()
            .count();

        let mut space = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut vec = Vec::with_capacity(cols);
            for _ in 0..cols {
                vec.push(Position::Free);
            }
            space.push(vec);
        }
        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                match char {
                    '#' => space[i][j] = Position::Obstacle,
                    '^' => {
                        player.set_pos(j, i);
                    }
                    _ => {}
                }
            }
        }
        World { player, space }
    }

    fn move_player(&mut self) -> PlayerMoveResult {
        let (dx, dy) = match self.player.dir {
            FaceDirection::Up => (0, -1),
            FaceDirection::Right => (1, 0),
            FaceDirection::Down => (0, 1),
            FaceDirection::Left => (-1, 0),
        };

        let next_tile = (self.player.coord.x + dx, self.player.coord.y + dy);

        if next_tile.0 < 0
            || next_tile.1 < 0
            || next_tile.0 as usize >= self.space[0].len()
            || next_tile.1 as usize >= self.space.len()
        {
            return PlayerMoveResult::FellOfWorld;
        }

        match self.space[next_tile.1 as usize][next_tile.0 as usize] {
            Position::Free => {
                self.player
                    .set_pos(next_tile.0 as usize, next_tile.1 as usize);
                PlayerMoveResult::Location(self.player.coord)
            }
            Position::Obstacle => {
                self.player.dir = match self.player.dir {
                    FaceDirection::Up => FaceDirection::Right,
                    FaceDirection::Right => FaceDirection::Down,
                    FaceDirection::Down => FaceDirection::Left,
                    FaceDirection::Left => FaceDirection::Up,
                };
                PlayerMoveResult::Turned
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let day = Day6 {
            input: "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
                .to_string(),
        };
        assert_eq!(day.part1().unwrap().trim(), "41");
    }
    #[test]
    fn part2_example() {
        let day = Day6 {
            input: "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
                .to_string(),
        };
        assert_eq!(day.part2().unwrap().trim(), "xxxx");
    }
}
