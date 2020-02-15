
//
// Project Euler Problem 3: Largest prime factor
//
// D. C. Groothuizen Dijkema - November, 2019
//

#include <chrono>
#include <iostream>

void run(void)
{
  const long long n=600851475143;
  long long val=n;
  int itr=2;

  while(itr*itr<val)
  {
    if (val%itr==0) { val/=itr; }
    else { ++itr; }
  }

  if (val>itr) { itr=val; }

  std::cout << "The largest prime factor of " << n << " is " << itr << std::endl;
}

int main(void)
{
  std::chrono::time_point<std::chrono::steady_clock> start = std::chrono::high_resolution_clock::now();
  run();
  std::chrono::time_point<std::chrono::steady_clock> finish = std::chrono::high_resolution_clock::now();
  std::chrono::duration<double> elapsed=finish-start;
  std::cout << "Elapsed time: " << elapsed.count() << std::endl;

  return 0;
}
