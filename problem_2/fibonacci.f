
!
! Project Euler Problem 2: Even Fibonacci numbers
!
! D. C. Groothuizen Dijkema - November, 2019
!

subroutine run
  implicit none
  integer :: fib_0,fib_1,sum,tmp
  integer, parameter :: limit=4000000

  fib_0=1
  fib_1=1
  sum=0

  do while (fib_0.lt.limit)
    if (mod(fib_0,2).eq.0) then
      sum=sum+fib_0
    end if
    tmp=fib_0
    fib_0=fib_1
    fib_1=fib_0+tmp
  end do

  print *,"The sum of all even Fibonacci numbers less than four million is:",sum

end subroutine run

program fibonacci
  implicit none
  integer :: beginning,end,rate

  call system_clock(beginning,rate)
  call run
  call system_clock(end)
  print *,"Elapsed time:",real(end-beginning)/real(rate)

end program
