
def floop (lnum1:list[int],lnum2:list[int]):
    i=0
    j=0
    while True:

        if j < len(lnum2) :    
            print("")
            print(f"pick lnum1:{lnum1[i]} ,lnum2:{lnum2[j]}")
            if (lnum1[i] > lnum2[j] ) or lnum1[i] == lnum2[j]:
                print(f"in case 1")
                lnum1.insert(i,lnum2[j])
                if j < len(lnum2):
                    j+=1
                #i+=1
            elif ( lnum1[i] < lnum2[j] ):
                #print(f"in case 2")
                if (i < len(lnum1)-1 ) :
                    if (lnum1[i+1] > lnum2[j]):
                        print(f"in case 2.1")
                        lnum1.insert(i+1,lnum2[j])
                        if j < len(lnum2):
                            j+=1
                        

                else : #i == len(lnum1)
                    print(f"in case 2.2")
                    lnum1.append(lnum2[j])
                    if j < len(lnum2):
                        j+=1
        else :
            print(f"j is out of range")            
                

        
        print(f"{i},{j}:{lnum1}")


        print(f"{i+1} >= {len(lnum1)} and {j+1} >= {len(lnum2)} : {(i+1 >= len(lnum1))} and {(j+1 >= len(lnum2))}")
        print("--------------------------------------------------")
        # fix break this here

        if (i+1 >= len(lnum1)) and (j+1 >= len(lnum2)) :
            break

        if i+1 < len(lnum1):
            i+=1
        else :
            print(f"lnum1 is out of range")

def myMedian(lnum:list[int])->float:
    if len(lnum) %2 ==0:
        i = int(len(lnum)/2)-1
        j = int(len(lnum)/2)
        print(f"i,j:{i},{j} -> {lnum[i]}+{lnum[j]}/2 -> {(lnum[i]+lnum[j])/2}")
        return (lnum[i] + lnum[j])/2
    else :
        i = int(len(lnum)/2)
        print(f"i:{i} - > {lnum[i]}")
        return lnum[i]
        
def run (lnum1:list[int],lnum2:list[int]) :
    if len(lnum1) > len(lnum2):
        floop(lnum1,lnum2)
        print(lnum1)
    elif len(lnum1) < len(lnum2) :
        floop(lnum2,lnum1)
        print(lnum2)
        return myMedian(lnum2)
    else : #len(lnum1) == len(lnum2)
        lnum1.insert(0,-1)
        floop(lnum1,lnum2)
        lnum1.remove(-1)
        print(lnum1)
    print("**********************************************")
    return myMedian(lnum1)


import unittest
import numpy
class UnitTest(unittest.TestCase):

    def test1(self):
        self.assertEqual(run([1,2],[3,4]),numpy.median([1,2,3,4]))
    
    def test2(self):
        self.assertEqual(run([1,3],[2]),numpy.median([1,2,3]))

    def test3(self):
        self.assertEqual(run([1,3,5,7,11],[1,2,3,4]),numpy.median([1,1,2,3,3,4,5,7,11]))

    def test4(self):
        self.assertEqual(run([3],[1,2,4]),numpy.median([1,2,3,4]))
    

if __name__ == "__main__" :      
    unittest.main()