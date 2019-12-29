
!
! Project Euler Problem 1: Multiples of 3 and 5
!
! D. C. Groothuizen Dijkema - November, 2019
!

subroutine run
  implicit none
  integer :: itr,sum

  sum=0

  do itr=1,999
    if(mod(itr,3).eq.0 .or. mod(itr,5).eq.0) then
      sum=sum+itr
    end if
  end do

  print *,"The sum of all multiples of 3 and 5 less than 1000 is:",sum

end subroutine run

program multiples
  implicit none
  integer :: beginning,end,rate

  call system_clock(beginning,rate)
  call run
  call system_clock(end)
  print *,"Elapsed time:",real(end-beginning)/real(rate)

end program
