use std::collections::VecDeque;

fn main() {
    println!("Part 1: {}", op_high_stakes_marbles(455, 71223));
    println!("Part 2: {}", op_high_stakes_marbles(455, 71223 * 100));
}

fn op_high_stakes_marbles(num_players: u32, num_marbles: u32) -> u32 {
    let mut game = GameState::new(num_players, num_marbles);
    let mut game_over = false;

    while !game_over {
        game_over = game.step();
    }

    //Return highest score
    *game.scores.iter().max().unwrap()
}

struct GameState {
    num_players: u32,
    num_marbles: u32,
    scores: Vec<u32>,
    circle: VecDeque<u32>,
    curr_player: u32,
    curr_marble: u32,
    next_marble: u32,
}

impl GameState {
    fn new(num_players: u32, num_marbles: u32) -> Self {
        let scores = vec![0u32; (num_players + 1) as usize];
        let circle = VecDeque::from([0]);

        GameState {
            num_players,
            num_marbles,
            scores,
            circle,
            curr_player: 1,
            curr_marble: 0,
            next_marble: 1,
        }
    }

    fn step(&mut self) -> bool {
        if self.next_marble % 23 == 0 {
            //Special rules
            self.circle.rotate_right(7);

            self.scores[self.curr_player as usize] += self.next_marble;
            self.scores[self.curr_player as usize] += self.circle.pop_front().unwrap();
        } else {
            //Normal rules
            if self.circle.len() >= 2 {
                self.circle.rotate_left(2);
            }
            
            self.circle.push_front(self.next_marble);
        }

        //Setup for next turn
        self.curr_marble = *self.circle.front().unwrap();

        self.next_marble += 1;

        if self.curr_player == self.num_players {
            self.curr_player = 1;
        } else {
            self.curr_player += 1;
        }

        //Return whether out of marbles
        self.next_marble > self.num_marbles
    }
}

#[cfg(test)]
mod test {
    use crate::op_high_stakes_marbles;

    #[test]
    fn test_part1() {
        let tests = vec![
            (9, 25, 32),
            (10, 1618, 8317),
            (13, 7999, 146373),
            (17, 1104, 2764),
            (21, 6111, 54718),
            (30, 5807, 37305),
        ];

        for (num_players, num_marbles, high_score) in tests {
            assert_eq!(high_score, op_high_stakes_marbles(num_players, num_marbles));
        }
    }
}


