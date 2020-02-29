
//
// Project Euler Problem 12: Highly divisible triangular number
//
// D. C. Groothuizen Dijkema - February, 2020
//

#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <time.h>

int triangle_number(const int n)
{
  return n*(n+1)/2;
}

int num_divisors(int n, const int init)
{
  if (n==1|n==2) { return n; }

  for (int itr=init;itr<(int)sqrt(n)+1;++itr)
  {
    // we've found the first prime that divides into n
    if (n%itr==0)
    {
      // divide the prime out of n
      int cnt=1;
      while (n%itr==0)
      {
        ++cnt;
        n/=itr;
      }
      // find the next prime that divides in, starting from after the current prime
      return cnt*num_divisors(n,itr+1);
    }
  }
  // if nothing up to sqrt(n) divides in, it must be prime
  return 2;
}

void run(void)
{
  bool found=false;
  int cnt_divisors=2;
  for (int itr=3;!found;++itr)
  {
    // itr and (itr+1) will never share factors as they are coprime.
    if (itr%2==0) { cnt_divisors=num_divisors(itr/2,2)*num_divisors(itr+1,2); }
    else { cnt_divisors=num_divisors(itr,2)*num_divisors((itr+1)/2,2); }

    if (cnt_divisors>500)
    {
      printf("The first triangle number with more than 500 factors is: %d\n",triangle_number(itr));
      found=true;
    }
  }
}

int main(void)
{
  clock_t t=clock(); 
  run();
  printf("Elapsed time: %fs\n",((double)(clock()-t))/CLOCKS_PER_SEC); 

  return 0; 
}
