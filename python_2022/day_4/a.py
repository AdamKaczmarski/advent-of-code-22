def check_if_containts(x,y,a,b):
    #print(f"{x} {y} {a} {b}")
    #if((a-x<=0 and b-y<=0) or (x-a<=0 or y-b<=0)):
    if(x-a<=0 and y-b>=0):
        return 1
    elif(x-a>=0 and y-b<=0):
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
