use burtle::*;
use std::collections::HashMap;

fn derive_iter<'a>(þ: &'a mut String, n: usize, p: &[&'static str; 2]) {
    let mut rules: HashMap<char, String> = HashMap::new();

    for rule in p {
        let parts: Vec<&str> = rule.split("->").map(|s| s.trim()).collect();
        if let [lhs, rhs] = parts.as_slice() {
            let key = lhs.chars().next().unwrap();
            rules.insert(key, rhs.to_string());
        }
    }

    for _ in 0..n {
        let mut deriv_þ = String::new();
        for character in þ.chars() {
            if let Some(rule) = rules.get(&character) {
                deriv_þ.push_str(rule);
            } else {
                deriv_þ.push(character);
            }
        }
        *þ = deriv_þ;
    }
}

fn dessiner(burtle: &mut Burtle, chaine: &String, angle: f32, length: f32) {
    for character in chaine.chars() {
        burtle.wait(0);
        match character {
            '+' => burtle.right(angle),

            '-' => burtle.left(angle),

            '[' => burtle.set_waypoint(),

            ']' => {
                burtle.pen_up();
                burtle.goto_waypoint();
                burtle.pen_down()
            }

            _ => burtle.forward(length),
        }
    }
}

fn main() {
    let mut þ = String::from("X");
    let p = ["X -> F[-X][X]F[-X]+FX", "F -> FF"];
    derive_iter(&mut þ, 9, &p);

    let mut burtle = Burtle::new();
    burtle.goto(0., 0.);
    burtle.pen_down();
    dessiner(&mut burtle, &þ, 25., 0.5);
    burtle.run(1500., 1000.)
}
