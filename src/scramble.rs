use rand::prelude::*;
use rand::rng;

pub fn generate_scramble() -> String {
    let faces = ["U", "D", "F", "B", "L", "R"];
    let modifiers = ["", "^", "2"];
    let mut rng = rng();
    let mut scramble = Vec::new();
    let mut last_face = "";

    while scramble.len() < 20 {
        let face = faces.choose(&mut rng).unwrap();

        if face == &last_face {
            continue;
        }

        let modifier = modifiers.choose(&mut rng).unwrap();
        scramble.push(format!("{face}{modifier}"));
        last_face = face;
    }

    scramble.join("  ")
}
