
#
# Project Euler Problem 6: Sum square difference
#
# D. C. Groothuizen Dijkema - January, 2020
#

from time import process_time as time

import numpy as np

def run():
  
  n=100

  sum=n*(n+1)/2
  squared=n*(n+1)*(2*n+1)/6
  r=sum**2-squared

  print('The difference between the sum of the squares of the first one hundred natural numbers and the square of the sum is: {}'.format(r))

if __name__=='__main__':
  t0=time()
  run()
  t1=time()
  print("Elapsed time: {}".format(t1-t0))
