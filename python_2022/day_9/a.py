def movehead(posh,x,y):
    posh[0]+=x
    posh[1]+=y

def main():
    list_size=0
    commands=[]
    with  open('./input.test') as file:
        for line in file:
            line = line.strip()
            line=line.split()
            
            commands.append(line)
            if list_size<int(line[1]):
                list_size=int(line[1])
    movemap=[['.']*list_size for i in range(list_size)]
    #starting point
    movemap[list_size-1][list_size-1]="#"
    posh=[list_size-1,0]
    post=[list_size-1,0]
    for command in commands:
        value=int(command[1])
        if command[0]=="R":
            movehead(posh,0,value)
        elif command[0]=="U":
            movehead(posh,-1*value,0)
        elif command[0]=="L":
            movehead(posh,0,-1*value)
        elif command[0]=="D":
            movehead(posh,value,0)



if __name__ == '__main__':
    main()
