
//
// Project Euler Problem 11: Largest product in a grid
//
// D. C. Groothuizen Dijkema - February, 2020
//

use std::io::BufRead;

/// Read in the grid of numbers
fn read_grid() -> [[u64;20];20]
{
  let mut array: [[u64;20];20]=[[0;20];20];
  // create the buffer
  let file: std::fs::File=std::fs::File::open("grid.txt").expect("Unable to open file.");
  let lines: std::io::Lines<std::io::BufReader<std::fs::File>>=std::io::BufReader::new(file).lines();
  // loop over the lines
  for (itr,line) in lines.enumerate()
  {
    if let Ok(ip)=line
    {
      // split the line by commas
      let vec=ip.split(",").collect::<Vec<&str>>();
      let mut jtr: u32=0; // for some reason I can't enumerate the vec
      for val in vec
      {
        array[itr as usize][jtr as usize]=val.parse::<u64>().unwrap();
        jtr+=1;
      }
    }
  }
  array
}

fn run()
{
  let grid: [[u64;20];20]=read_grid();

  let mut max=1;

  // loop over the grid
  for itr in 0..20
  {
    for jtr in 0..17
    {
      // check the verticals
      let prod=grid[jtr][itr]*grid[jtr+1][itr]*grid[jtr+2][itr]*grid[jtr+3][itr];
      if prod>max { max=prod; }
      // check the horizontals
      let prod=grid[itr][jtr]*grid[itr][jtr+1]*grid[itr][jtr+2]*grid[itr][jtr+3];
      if prod>max { max=prod; }
    }
    if itr<17
    {
      for jtr in 0..17
      {
        // check the down and forward diagonals
        let prod=grid[jtr][itr]*grid[jtr+1][itr+1]*grid[jtr+2][itr+2]*grid[jtr+3][itr+3];
        if prod>max { max=prod; }
      }
    }
    if itr>2
    {
      for jtr in 0..17
      {
        // check the up and forward diagonals
        let prod=grid[itr][jtr]*grid[itr-1][jtr+1]*grid[itr-2][jtr+2]*grid[itr-3][jtr+3];
        if prod>max { max=prod; }
      }
    }
  }

  println!("The maximum product of four numbers in the grid is {}",max);
}

fn main()
{
  let now: std::time::Instant=std::time::Instant::now();
  run();
  println!("Elapsed time: {}ms",now.elapsed().as_millis());
}
