
def floop (lnum1:list[int],lnum2:list[int]):
    i=0
    j=0
    while True:

        if i < len(lnum1)-1 or j < len(lnum2)-1 :
            x,y=-2,-2
            try :
                x = lnum1[i]
            except :
                print(f"lnum1[{i}] is out of range")
            try : 
                y = lnum2[j]
            except :
                print(f"lnum2[{j}] is out of range")
            print(f"pick lnum1:{lnum1[i]} ,lnum2:{lnum2[j]}")
        if (lnum1[i] > lnum2[j] ) or lnum1[i] == lnum2[j]:
            print(f"in case 1")
            lnum1.insert(i,lnum2[j])
            #if j < len(lnum2)-1:
            j+=1
            #i+=1
        elif ( lnum1[i] < lnum2[j] ):
            #print(f"in case 2")
            if (i < len(lnum1)-1 ) :
                if (lnum1[i+1] > lnum2[j]):
                    print(f"in case 2.1")
                    lnum1.insert(i+1,lnum2[j])
                    j+=1
                    #i+=1

            else : #i == len(lnum1)
                print(f"in case 2.2")
                lnum1.append(lnum2[j])
                j+=1
                #i+=1
            #if j < len(lnum2)-1:

        
        print(f"{i},{j}:{lnum1}")
        i+=1
        print(f"{i} >= {len(lnum1)-1} and {j} > {len(lnum2)-1} : {(i >= len(lnum1)-1)} and {(j > len(lnum2)-1)}")
        if (i >= len(lnum1)-1) and (j > len(lnum2)-1) :
            break

        print("--------------------------------------------------")
        
        
        
def median(lnum:list[int])->float:
    if len(lnum) %2 ==0:
        return (lnum[int(len(lnum)/2)-1] + lnum[int(len(lnum)/2)])/2
    else :
        return lnum[int(len(lnum)/2)]
        
def run (lnum1:list[int],lnum2:list[int]) :
    if len(lnum1) > len(lnum2):
        floop(lnum1,lnum2)
        print(lnum1)
    elif len(lnum1) < len(lnum2) :
        floop(lnum2,lnum1)
        print(lnum2)
    else : #len(lnum1) == len(lnum2)
        lnum1.insert(0,-1)
        floop(lnum1,lnum2)
        lnum1.remove(-1)
        print(lnum1)
    return median(lnum1)


import unittest
from numpy import median
class UnitTest(unittest.TestCase):
    def test1(self):
        self.assertEqual(run([1,2],[3,4]),median([1,2,3,4]))
    
    def test2(self):
        self.assertEqual(run([1,3],[2]),median([1,2,3]))

    def test3(self):
        self.assertEqual(run([1,3,5,7,11],[1,2,3,4]),median([1,1,2,3,4,5,7,11]))
if __name__ == "__main__" :      
    unittest.main()
    