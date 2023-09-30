def check_if_containts(x,y,a,b):
    #print(f"{x} {y} {a} {b}")
    #This is so dirty i will need a shower
    pair1=set({})
    pair2=set({})
    for i in range(y-x):
        pair1.add(x+i)
    pair1.add(y)
    for i in range(b-a):
        pair2.add(a+i)
    pair2.add(b)
    if len(pair1.intersection(pair2))>0:
        return 1
    else:
        return 0
def main():
    sum = 0
    with  open('./input.prod') as file:
        for line in file:
            line = line.strip()
            assignments = line.split(',')
            assignments[0]=assignments[0].split('-')
            assignments[1]=assignments[1].split('-')
            sum+=check_if_containts(int(assignments[0][0]),int(assignments[0][1]),int(assignments[1][0]),int(assignments[1][1]))
    print(sum)

if __name__ == '__main__':
    main()
