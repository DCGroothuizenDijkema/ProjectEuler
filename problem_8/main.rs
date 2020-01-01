
//
// Project Euler Problem 8: Largest product in a series
//
// D. C. Groothuizen Dijkema - January, 2020
//

fn run()
{
  // read in the string
  let filename: std::string::String="number.txt".to_string();
  let contents: std::string::String=std::fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

  let mut greatest_prod: u64=1;
  // loop over each 13 digit slice
  for itr in 0..contents.chars().count()-12
  {
    // get the 13 digit substring
    let slice=&contents[itr..itr+13];
    // if the substring contains 0, the product will be 0
    if slice.contains('0') { continue; }
    // otherwise, calculate the product of all hte digits
    let mut prod: u64=1;
    for num in slice.chars()
    {
      // convert the character to a number
      let subnum: u32=match num.to_digit(10)
      {
        Some(x) => x,
        None    => panic!("A number could not be extracted for the char."),
      };
      prod*=subnum as u64;
    }
    // update
    if prod>greatest_prod { greatest_prod=prod; }
  }
  println!("The greatest product of thirteen adjacent digits is: {}",greatest_prod);
}

fn main()
{
  let now: std::time::Instant=std::time::Instant::now();
  run();
  println!("Elapsed time: {}ms", now.elapsed().as_millis());
}
