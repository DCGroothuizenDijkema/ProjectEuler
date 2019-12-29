
#
# Project Euler Problem 4: Largest palindrome product
#
# D. C. Groothuizen Dijkema - November, 2019
#

from time import process_time as time

import numpy as np

def palindrome(num):
  # turn the number into a np array of the strings of its digits
  digits=np.array(list(str(num)))
  return np.all(digits==np.flip(digits))

def run():
  numbers=np.flip(np.arange(start=100,stop=1000))
  palin=0

  for itr in range(0,len(numbers)):
    for jtr in range(itr,len(numbers)):
      num=numbers[itr]*numbers[jtr]
      if palindrome(num) and num>palin:
        palin=num
        break # multiplying itr by a smaller jtr isn't going to give us a bigger number

  print('The largest palindrome which is the product of two three digit numbers is: {}'.format(palin))

if __name__=='__main__':
  t0=time()
  run()
  t1=time()
  print("Elapsed time: {}".format(t1-t0))
