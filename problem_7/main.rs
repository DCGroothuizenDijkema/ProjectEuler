
fn upper_bound(a: i32) -> i32
{
  let n: f64=a as f64;
  (n*(n.ln()+n.ln().ln())).ceil() as i32
}

fn run()
{
  let n: i32=upper_bound(10001);
  let upper: i32=(n as f64).sqrt().ceil() as i32;

  let mut primes: std::vec::Vec<bool>=vec![true;(n+1) as usize];
  primes[0]=false;
  primes[1]=false;

  // find all primes between 0 to the upper bound
  for itr in 2..=upper
  {
    for jtr in (itr*itr..=n).step_by(itr as usize)
    {
      primes[jtr as usize]=false;
    }
  }

  // find the 10001st prime
  let mut cnt: i32=0;
  for (itr,prime) in primes.into_iter().enumerate()
  {
    if prime { cnt+=1; }
    if cnt==10001
    {
      println!("The 10001st prime is {}",itr);
      return;
    }
  }
}

fn main()
{
  let now: std::time::Instant=std::time::Instant::now();
  run();
  println!("Elapsed time: {}ns", now.elapsed().as_nanos());
}
