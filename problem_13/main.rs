
//
// Project Euler Problem 13: Large sum
//
// D. C. Groothuizen Dijkema - February, 2020
//

use std::io::BufRead;

/// Read in the grid of numbers
fn read_numbers() -> [[u32;50];100]
{
  let mut array: [[u32;50];100]=[[0;50];100];
  // create the buffer
  let file: std::fs::File=std::fs::File::open("number.txt").expect("Unable to open file.");
  let lines: std::io::Lines<std::io::BufReader<std::fs::File>>=std::io::BufReader::new(file).lines();
  // loop over the lines
  for (itr,line) in lines.enumerate()
  {
    if let Ok(ip)=line
    {
      let number: std::vec::Vec<u32>=ip.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
      for (jtr,val) in number.iter().enumerate()
      {
        array[itr][jtr]=*val;
      }
    }
  }
  array
}

fn run()
{
  let array: [[u32;50];100]=read_numbers();
  let mut result: [u32;50]=[0;50];
  let mut overflow: u32=0;
  for itr in (0..50).rev()
  {
    let mut sum: u32=0;
    // find the sum of these numbers
    for jtr in 0..100
    {
      sum+=array[jtr][itr]
    }
    // add on the overflow from the previous column
    sum+=overflow;
    // determine the result in this column
    result[itr]=sum%10;
    // determine the overflow to the next column
    overflow=sum/10;
  }
  // 0..8 is just a tad hardcoded.
  let result: u64=result[0..8].iter().fold(overflow as u64, |acc,elem| acc*10+(*elem as u64));
  println!("The first ten digits of the sum is: {}",result);
}

fn main()
{
  let now: std::time::Instant=std::time::Instant::now();
  run();
  println!("Elapsed time: {}ms",now.elapsed().as_millis());
}
