from termcolor import colored as c

n, l, h, w, r = "Linux", len("Linux"), c("@", "red"), 15, range

print(

         "\n".join      (["\n".
       join([" "*(  1-i)+h*(5+i)+
      " "*(3-i)+h*(5+i) for i in [0
    ,2]])," "+h*(w-2)+" "," "*(4//2)+
      h*(((w-4)-l)//2)+c(n,"green")
       +h*(((w-4)-l)//2),"\n".join
         ([" "*((w-i)//2)+h*i+" "
            *((w-i)//2) for 
              i in r(9,3,
                 -2)])
                   ])

)
