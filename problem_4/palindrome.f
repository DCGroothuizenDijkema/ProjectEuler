
!
! Project Euler Problem 4: Largest palindrome product
!
! D. C. Groothuizen Dijkema - November, 2019
!

function reversible(num)
  ! Determine if an integer is palindromic in its digits

  implicit none

  logical :: reversible
  integer, intent(in) :: num
  integer :: len,itr
  ! 999*999 is 998001, so only 6 characters are needed
  character(len=6) :: str,rev
  ! print to the string
  write (str,"(I6)") num
  len=len_trim(str)
  ! flip the string and save to rev
  do itr=1,len
    rev(itr:itr)=str(len+1-itr:len+1-itr)
  end do

  reversible=str(1:len).eq.rev(1:len)

end function reversible

subroutine run
  implicit none
  logical, external :: reversible

  integer :: itr,jtr,val
  val=0

  itr=993
  jtr=913

  do itr=999,100,-1
    do jtr=999,itr,-1
      if (reversible(itr*jtr) .and. itr*jtr.gt.val) then
        val=itr*jtr
        exit ! multiplying itr by a smaller jtr isn't going to give us a bigger number
      end if
    end do
  end do

  print *,'The largest palindrome which is the product of two three digit numbers is:',val

end subroutine run

program palindrome
  implicit none
  integer :: beginning,end,rate

  call system_clock(beginning,rate)
  call run
  call system_clock(end)
  print *,"Elapsed time:",real(end-beginning)/real(rate)

end program
