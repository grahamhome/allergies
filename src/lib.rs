#![feature(variant_count)]

mod tests;

pub struct Allergies {score: u32}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies{score}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        let allergen_count = std::mem::variant_count::<Allergen>();
        let max_allergen_score: u32 = (0..allergen_count).map(|i| 2_u32.pow(i as u32)).sum();
        let mut allergen_score = self.score.clone();
        // Adjust score by subtracting increasing powers of 2
        'score_adjuster: while allergen_score > max_allergen_score {
            let mut index = allergen_count+1;
            loop {
                if (2_u32.pow(index as u32) as u32) > allergen_score && (2_u32.pow(index as u32 - 1) as u32) <= allergen_score {
                    allergen_score -= 2_u32.pow(index as u32 - 1);
                    continue 'score_adjuster
                } else {
                    index += 1;
                }
            }
        }
        'allergen_collector: while allergen_score > 0 {
            // Get the highest-scoring allergen that is less than the patient's score
            for index in 1..=std::mem::variant_count::<Allergen>() {
                if (2_u32.pow(index as u32) as u32) > allergen_score && (2_u32.pow(index as u32 - 1) as u32) <= allergen_score {
                    allergies.push(match index - 1 {
                        0 => Allergen::Eggs,
                        1 => Allergen::Peanuts,
                        2 => Allergen::Shellfish,
                        3 => Allergen::Strawberries,
                        4 => Allergen::Tomatoes,
                        5 => Allergen::Chocolate,
                        6 => Allergen::Pollen,
                        7 => Allergen::Cats,
                        _ => panic!()
                    });
                    allergen_score -= 2_u32.pow(index as u32 - 1);
                    continue 'allergen_collector;
                }
            }
        }
        allergies
    }
}