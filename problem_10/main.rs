
//
// Project Euler Problem 10: Summation of primes
//
// D. C. Groothuizen Dijkema - February, 2020
//

fn run()
{
  let n: i32=2_000_000;
  let upper: i32=(n as f64).sqrt().ceil() as i32;

  let mut primes: std::vec::Vec<bool>=vec![true;(n+1) as usize];
  primes[0]=false;
  primes[1]=false;

  // find all primes between 0 and 2000000
  for itr in 2..=upper
  {
    for jtr in (itr*itr..=n).step_by(itr as usize) { primes[jtr as usize]=false; }
  }

  //s sum the primes
  let mut sum: u64=0;
  for (itr,prime) in primes.into_iter().enumerate()
  {
    if prime { sum+=itr as u64; }
  }
  print!("The sum of all primes less than 2,000,000 is {}\n",sum);
}

fn main()
{
  let now: std::time::Instant=std::time::Instant::now();
  run();
  println!("Elapsed time: {}ms",now.elapsed().as_millis());
}
