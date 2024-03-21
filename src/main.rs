use std::io;

#[derive(PartialEq, Debug)]
enum Action {
    Bow,
    Clap,
    Other,
}

struct Saho {
    sequence: Vec<Action>,
    current_index: usize,
}

impl Saho {
    fn new() -> Saho {
        Saho {
            sequence: vec![
                Action::Bow,
                Action::Bow,
                Action::Clap,
                Action::Clap,
                Action::Bow,
            ],
            current_index: 0,
        }
    }

    fn omairi(&mut self, action: Action) -> bool {
        match action {
            Action::Other => {
                println!("Oops, Start over the omairi");
                self.current_index = 0;
                false
            }
            _ => {
                if action == self.sequence[self.current_index] {
                    println!("{:?}", self.sequence[self.current_index]);
                    self.current_index += 1;

                    if self.current_index == self.sequence.len() {
                        println!("ちょっと良いことあるかも");
                        true
                    } else {
                        false
                    }
                } else {
                    println!("Oops, Start over the omairi");
                    self.current_index = 0;
                    false
                }
            }
        }
    }
}

fn main() {
    println!("Omairi start ([礼: press`b`][拍手: press`c`])");

    let mut saho = Saho::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let action = match input.trim() {
            "b" => Action::Bow,
            "c" => Action::Clap,
            _ => Action::Other,
        };

        if saho.omairi(action) {
            break;
        }
    }
}
