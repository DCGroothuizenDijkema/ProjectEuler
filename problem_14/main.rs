
//
// Project Euler Problem 14: Longest Collatz sequence
//
// D. C. Groothuizen Dijkema - February, 2020
//

fn collatz(n: u64, itr: u64, calculated: &std::boxed::Box<[Option<u64>]>) -> u64
{
  if n<1_000_001 && calculated[(n-1) as usize].is_some() { return itr+calculated[(n-1) as usize].unwrap(); }
  if n%2==0 { return collatz(n/2,itr+1,calculated); }
  else if n%2==1&&n!=1 { return collatz(3*n+1,itr+1,calculated); }
  itr
}

fn run()
{
  let mut calculated: std::boxed::Box<[Option<u64>]>=vec![None;1_000_000].into_boxed_slice();

  for itr in 1..1000000 { calculated[itr-1]=Some(collatz(itr as u64,1,&calculated)); }
  let arg_max: std::option::Option<(&std::option::Option<u64>, usize)>=calculated.iter().enumerate().map(|(x, y)| (y, x)).max();

  println!("{}",arg_max.unwrap().1+1);
}

fn main()
{
  let now: std::time::Instant=std::time::Instant::now();
  run();
  println!("Elapsed time: {}ms",now.elapsed().as_millis());
}
