struct Entity {
    hit_points: usize,
    damage: usize,
    armor: usize,
}

impl Entity {
    fn new(hit_points: usize, equipment_set: Vec<Equipment>) -> Self {
        Self {
            hit_points,
            damage: equipment_set.iter().map(damage).sum(),
            armor: equipment_set.iter().map(armor).sum(),
        }
    }

    fn attack(&self, target: &mut Entity) -> bool {
        let hit = std::cmp::max(self.damage.saturating_sub(target.armor), 1);
        target.hit_points = target.hit_points.saturating_sub(hit);
        target.hit_points == 0
    }
}

type Equipment = (usize, usize, usize);

fn cost(e: &Equipment) -> usize {
    e.0
}

fn damage(e: &Equipment) -> usize {
    e.1
}

fn armor(e: &Equipment) -> usize {
    e.2
}

enum Battle {
    Lost,
    Won,
}

fn fight(mut player: Entity, mut boss: Entity) -> Battle {
    loop {
        if player.attack(&mut boss) {
            return Battle::Won;
        }
        if boss.attack(&mut player) {
            return Battle::Lost;
        }
    }
}

fn main() {
    let weapons = vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armor = vec![
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings = vec![
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];
    let mut cheapest = usize::MAX;
    let mut expensivest = usize::MIN;
    for w in weapons.iter().cloned() {
        for a in armor.iter().cloned() {
            for (l_idx, ll) in rings.iter().cloned().enumerate() {
                for (r_idx, lr) in rings.iter().cloned().enumerate() {
                    if r_idx >= l_idx {
                        continue;
                    }
                    let equipment_set = vec![w, a, ll, lr];
                    let cost = equipment_set.iter().map(cost).sum::<usize>();
                    let player = Entity::new(100, equipment_set);
                    let boss = Entity {
                        hit_points: 109,
                        damage: 8,
                        armor: 2,
                    };
                    match fight(player, boss) {
                        Battle::Won if cost < cheapest => cheapest = cost,
                        Battle::Lost if cost > expensivest => expensivest = cost,
                        _ => {}
                    }
                }
            }
        }
    }
    println!("part 1: {}", cheapest);
    println!("part 2: {}", expensivest);
}
