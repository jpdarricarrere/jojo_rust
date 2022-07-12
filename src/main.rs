struct Coord(u32, u32);

const WIDTH: usize = 10_000_000;
const HEIGTH: usize = 10_000_000;
struct GameMap([Entity; WIDTH * HEIGTH]);

impl GameMap {
    pub fn print_map(&self) {
        for entity in self.0.iter() {
            match entity {
                Entity::Minion(_minion) => println!("M"),
                Entity::Player(_player) => println!("P"),
                Entity::None => println!(" "),
            }
        }
    }
}

enum Entity {
    None,
    Minion(Minion),
    Player(Player),
}

enum AttackType {
    NoAttack,
    MeleeAttack(u32),
    RangedAttack { damage: u32, hit: u32 },
}

struct Minion {
    loc: Coord,
    attack_type: AttackType,
    health: u32,
}

struct Player {
    loc: Coord,
    health: u32,
    attack_type: AttackType,
}

fn main() {
    let player = Player {
        loc: Coord(0, 0),
        health: 100,
        attack_type: AttackType::NoAttack,
    };
    let minion = Minion {
        loc: Coord(5, 8),
        health: 50,
        attack_type: AttackType::NoAttack,
    };
    let map = GameMap([Entity::None; WIDTH * HEIGTH]);

    map.print_map();
}
