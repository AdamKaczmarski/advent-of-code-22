def compare(top_three,tmpSum):
    if len(top_three)<3:
        top_three.append(tmpSum)
    else:
        idx=-1
        for cal in top_three:
            if (tmpSum>cal):
                idx = top_three.index(cal)
        if idx!=-1:
            top_three[idx]=tmpSum
    top_three.sort(reverse=True)
    return top_three

def main():
    top_three = []
    tmpSum = 0
    with  open('./input2.txt') as file:
        for line in file:
            if line.strip() =="":
                top_three = compare(top_three,tmpSum)
                tmpSum=0
            else:
                line = line.strip()
                tmpSum += int(line)
       #Handle the last block of input. It is copied because the file doesn't end with an empty line 
        top_three = compare(top_three,tmpSum)
    print(sum(top_three))




if __name__ == '__main__':
    main()
