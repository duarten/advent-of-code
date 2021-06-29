use std::io::{BufRead, BufReader};
use std::{cmp::max, fs::File};

use regex::Regex;

#[derive(Debug, Default, Clone, Copy)]
struct IngredientProps {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: u64,
}

impl std::ops::Mul<usize> for IngredientProps {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            capacity: self.capacity * rhs as i64,
            durability: self.durability * rhs as i64,
            flavor: self.flavor * rhs as i64,
            texture: self.texture * rhs as i64,
            calories: self.calories * rhs as u64,
        }
    }
}

impl IngredientProps {
    fn score(&self) -> i64 {
        max(&self.capacity, &0)
            * max(&self.durability, &0)
            * max(&self.flavor, &0)
            * max(&self.texture, &0)
    }
}

impl std::ops::Add for IngredientProps {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    props: IngredientProps,
}

fn max_score(
    ingredients: &[Ingredient],
    calories_target: Option<u64>,
    idx: usize,
    used: usize,
    prev: IngredientProps,
) -> i64 {
    if idx >= ingredients.len() {
        if used == 100 && calories_target.map(|t| t == prev.calories).unwrap_or(true) {
            return prev.score();
        }
        return 0;
    }
    (0..=(100 - used))
        .map(|teaspoons| {
            max_score(
                ingredients,
                calories_target,
                idx + 1,
                used + teaspoons,
                prev + (ingredients[idx].props * teaspoons),
            )
        })
        .max()
        .unwrap()
}

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r#"^([\w]+): capacity (-?\d), durability (-?\d), flavor (-?\d), texture (-?\d), calories (\d)"#).unwrap();
}

fn main() {
    let file = File::open("aoc2015/inputs/day15.input").unwrap();
    let ingredients = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .filter_map(|l| {
            RE.captures(&l).map(|g| Ingredient {
                name: g.get(1).unwrap().as_str().to_owned(),
                props: IngredientProps {
                    capacity: g.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                    durability: g.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                    flavor: g.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                    texture: g.get(5).unwrap().as_str().parse::<i64>().unwrap(),
                    calories: g.get(6).unwrap().as_str().parse::<u64>().unwrap(),
                },
            })
        })
        .collect::<Vec<_>>();
    println!(
        "part 1: {}",
        max_score(&ingredients, None, 0, 0, Default::default())
    );
    println!(
        "part 2: {}",
        max_score(&ingredients, Some(500), 0, 0, Default::default())
    );
}
