
!
! Project Euler Problem 3: Largest Prime Factor
!
! D. C. Groothuizen Dijkema - November, 2019
!

subroutine run
  implicit none
  integer :: itr
  integer(kind=8) :: val
  integer(kind=8),parameter :: n=600851475143

  val=n
  itr=2

  do while(itr*itr<val)
    if (mod(val,itr).eq.0) then
      val=val/itr
    else
      itr=itr+1
    end if
  end do

  if (val>itr) then
    itr=val
  end if

  print *,"largest prime factor of ",n," is:",itr

end subroutine run

program prime
  implicit none
  integer :: beginning,end,rate

  call system_clock(beginning,rate)
  call run
  call system_clock(end)
  print *,"Elapsed time:",real(end-beginning)/real(rate)

end program
