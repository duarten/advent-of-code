use itertools::iterate;

#[derive(Clone)]
enum EffectType {
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone)]
struct Effect(usize, EffectType);

impl Effect {
    fn decrease(&mut self) {
        self.0 -= 1;
    }
}

#[derive(Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Poison,
    Shield,
    Recharge,
}

impl Spell {
    fn mana(&self) -> usize {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Poison => 173,
            Self::Shield => 113,
            Self::Recharge => 229,
        }
    }

    fn can_cast(&self, effects: &[Effect]) -> bool {
        !effects.iter().any(|e| match (self, e.1.clone()) {
            (Self::Poison, EffectType::Poison)
            | (Self::Shield, EffectType::Shield)
            | (Self::Recharge, EffectType::Recharge) => e.0 > 1,
            _ => false,
        })
    }

    fn all() -> Vec<Self> {
        vec![
            Self::MagicMissile,
            Self::Drain,
            Self::Poison,
            Self::Shield,
            Self::Recharge,
        ]
    }
}

#[derive(Clone)]
struct Player {
    hit_points: usize,
    mana: usize,
    hard: bool,
}

impl Player {
    fn new(hard: bool) -> Self {
        Self {
            hit_points: 50,
            mana: 500,
            hard,
        }
    }

    fn recharge(&mut self) {
        self.mana += 101;
    }

    fn attack(&mut self, target: &mut Boss, effects: &[Effect], spell: Spell) -> Option<Effect> {
        self.hit_points -= self.hard as usize;
        if self.hit_points == 0 {
            return None;
        }
        for effect in effects.iter() {
            match effect.1 {
                EffectType::Poison => target.poison(),
                EffectType::Recharge => self.recharge(),
                _ => {}
            }
        }
        if spell.mana() > self.mana {
            self.hit_points = 0;
            None
        } else {
            self.mana -= spell.mana();
            match spell {
                Spell::MagicMissile => {
                    target.hit_points = target.hit_points.saturating_sub(4);
                    None
                }
                Spell::Drain => {
                    target.hit_points = target.hit_points.saturating_sub(2);
                    self.hit_points += 2;
                    None
                }
                Spell::Poison => Some(Effect(6, EffectType::Poison)),
                Spell::Shield => Some(Effect(6, EffectType::Shield)),
                Spell::Recharge => Some(Effect(5, EffectType::Recharge)),
            }
        }
    }
}

#[derive(Clone)]
struct Boss {
    hit_points: usize,
    damage: usize,
}

impl Default for Boss {
    fn default() -> Self {
        Self {
            hit_points: 71,
            damage: 10,
        }
    }
}

impl Boss {
    fn attack(&mut self, target: &mut Player, effects: &[Effect]) {
        let mut damage = self.damage;
        for effect in effects {
            match effect.1 {
                EffectType::Shield => damage = damage.saturating_sub(7),
                EffectType::Poison => self.poison(),
                EffectType::Recharge => target.recharge(),
            }
        }
        target.hit_points = target.hit_points.saturating_sub(std::cmp::max(damage, 1));
    }

    fn poison(&mut self) {
        self.hit_points = self.hit_points.saturating_sub(3);
    }
}

struct State {
    player: Player,
    boss: Boss,
    effects: Vec<Effect>,
    spent: usize,
}

fn turn_end(effects: &mut Vec<Effect>) {
    effects.iter_mut().for_each(Effect::decrease);
    effects.retain(|e| e.0 > 0);
}

fn calculate_state(state: &State, spell: Spell, min_mana: usize) -> (usize, Option<State>) {
    let mut new_player = state.player.clone();
    let mut new_boss = state.boss.clone();
    let mut new_effects = state.effects.clone();
    let spent = state.spent + spell.mana();
    let new_effect = new_player.attack(&mut new_boss, &new_effects, spell);
    turn_end(&mut new_effects);
    new_effects.extend(new_effect.into_iter());
    new_boss.attack(&mut new_player, &new_effects);
    turn_end(&mut new_effects);
    if new_boss.hit_points == 0 {
        (std::cmp::min(min_mana, spent), None)
    } else if new_player.hit_points == 0 || spent >= min_mana {
        (min_mana, None)
    } else {
        (
            min_mana,
            Some(State {
                player: new_player,
                boss: new_boss,
                effects: new_effects,
                spent,
            }),
        )
    }
}

fn fight_one(mut min_mana: usize, current_states: &[State]) -> (usize, Vec<State>) {
    let mut new_states = Vec::new();
    for state in current_states {
        for spell in Spell::all()
            .into_iter()
            .filter(|s| s.can_cast(&state.effects))
        {
            let (mana, new_state) = calculate_state(state, spell, min_mana);
            min_mana = mana;
            new_states.extend(new_state.into_iter());
        }
    }
    (min_mana, new_states)
}

fn run(hard: bool) -> usize {
    iterate(
        (
            usize::MAX,
            vec![State {
                player: Player::new(hard),
                boss: Boss::default(),
                effects: vec![],
                spent: 0,
            }],
        ),
        |(min_mana, states)| fight_one(*min_mana, states),
    )
    .take_while(|(_, states)| !states.is_empty())
    .last()
    .unwrap()
    .0
}

fn main() {
    println!("part 1: {}", run(false));
    println!("part 2: {}", run(true));
}
