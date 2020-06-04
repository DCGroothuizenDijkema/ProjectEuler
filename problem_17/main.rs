
//
// Project Euler Problem 17: Number letter counts
//
// D. C. Groothuizen Dijkema - March, 2020
//

fn digit_to_word(num: isize) -> std::string::String
{
  match num
  {
    0 => "zero".to_string()
    ,1 => "one".to_string()
    ,2 => "two".to_string()
    ,3 => "three".to_string()
    ,4 => "four".to_string()
    ,5 => "five" .to_string()
    ,6 => "six".to_string()
    ,7 => "seven".to_string()
    ,8 => "eight".to_string()
    ,9 => "nine".to_string()
    ,10 => "ten".to_string()
    ,11 => "eleven".to_string()
    ,12 => "twelve".to_string()
    ,13 => "thirteen".to_string()
    ,14 => "fourteen".to_string()
    ,15 => "fifteen".to_string()
    ,16 => "sixteen".to_string()
    ,17 => "seventeen".to_string()
    ,18 => "eighteen".to_string()
    ,19 => "nineteen".to_string()
    ,20 => "twenty".to_string()
    ,30 => "thirty".to_string()
    ,40 => "forty".to_string()
    ,50 => "fifty".to_string()
    ,60 => "sixty".to_string()
    ,70 => "seventy".to_string()
    ,80 => "eighty".to_string()
    ,90 => "ninety".to_string()
    ,_ => "".to_string()
  }
}

fn tens_to_word(num: isize) -> std::string::String
{
  if num<20 { return digit_to_word(num); }
  let mut words: std::string::String=digit_to_word(num/10*10);
  if num%10!=0 { words+=&digit_to_word(num%10); }
  words
}

fn hundreds_to_word(num: isize) -> std::string::String
{
  let mut words: std::string::String=digit_to_word(num/100)+&"hundred";
  if num%100!=0 { words+=&("and".to_owned()+&tens_to_word(num%100)); }
  words
}

fn num_to_words(num: isize) -> std::string::String
{
  let num_digits: i32=(num as f64).log10().floor() as i32;
  if num_digits==0 { return digit_to_word(num); }
  if num_digits==1 { return tens_to_word(num); }
  if num_digits==2 { return hundreds_to_word(num); }
  return "onethousand".to_string();
}

fn run()
{
  let mut sum: usize=0;
  for itr in 1..=1000 { sum+=num_to_words(itr).chars().count(); }

  println!("The sum of number letter counts is {}",sum);
}

fn main()
{
  let now: std::time::Instant=std::time::Instant::now();
  run();
  println!("Elapsed time: {}ms",now.elapsed().as_millis());
}
