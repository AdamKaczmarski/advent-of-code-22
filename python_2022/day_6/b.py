def get_marker(line):
    line = line.strip()
    tmp=[]
    for idx,char in enumerate(list(line)):
        if(len(tmp)==14):
            return idx
        if char not in tmp:
            tmp.append(char)
        else:
            right = tmp[tmp.index(char)+1:]
            tmp=right
            tmp.append(char)



def main():
    with  open('./input.prod') as file:
        lines = file.readlines();
        for line in lines:
            print(line)
            print(get_marker(line))
                    
            



if __name__ == '__main__':
    main()
