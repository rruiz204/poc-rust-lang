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


  println!("=== Manage Nullable values");
  let nullable: Option<i32> = Some(30);

  if let Some(value) = nullable {
    println!("The value is not null: {}", value);
  }
}