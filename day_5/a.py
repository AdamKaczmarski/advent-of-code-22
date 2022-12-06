import re
crateStacks = {}
def movecrate(amount,fromStack,toStack):
    for i in range(int(amount)):
        if len(crateStacks[fromStack])!=0:
            crate = crateStacks[fromStack].pop(0)
            crateStacks[toStack]=[crate]+crateStacks[toStack]

def main():
    crates = [] 
    with  open('./input.prod') as file:
        data = file.read().split("\n\n")
    print(data[0])
    crates = (data[0].split("\n"))
    moves = data[1].split('\n')
    moves = moves[:len(moves)-1]
    indexline = crates[len(crates)-1:][0]
    for idx in indexline.split():
        crateStacks[idx]=[]
    for line in crates[:len(crates)-1]:
        letters = re.findall(r'\w',line)
        for letter in letters:
            idx = line.index(letter)
            crateStacks[indexline[idx]].append(letter)
            line=line[:idx]+"_"+line[idx+1:]
    for move in moves:
        whatwhere = re.findall(r"\d+",move)
        movecrate(whatwhere[0],whatwhere[1],whatwhere[2])
    result=""
    for idx,stack in crateStacks.items():
        result+=stack[0]
    print(result)

if __name__ == '__main__':
    main()
