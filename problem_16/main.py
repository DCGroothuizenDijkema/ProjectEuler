
#
# Project Euler Problem 16: Power digit sum
#
# D. C. Groothuizen Dijkema - March, 2020
#

from time import process_time as time

def run():
  val=sum([int(itr) for itr in str(2**1000)])

  print('The sum of digits in 2^1000 is: {}'.format(val))

if __name__=='__main__':
  t0=time()
  run()
  t1=time()
  print("Elapsed time: {}".format(t1-t0))
