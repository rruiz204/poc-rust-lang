enum Step {
  First,
  Second,
  Thirty,
}

fn casino(step: Step) {
  match step {
    Step::First => println!("is the first option"),
    Step::Second => println!("is the second option"),
    Step::Thirty => println!("is the thirty option"),
  }
}

pub fn showcase() {
  println!("=== Enums and Matching");

  let step = Step::First;
  casino(step);
  
  let step: Step = Step::Second;
  casino(step);

  let step: Step = Step::Thirty;
  casino(step);
}