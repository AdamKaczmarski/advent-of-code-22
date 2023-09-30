def charToValue(char):
    asccival = ord(char)
    if asccival < 97:
        #upperCase
        return asccival-38
    else:
        #lowerCase
        return asccival-96
def main():
    sum =0
    threelines = []
    with  open('./input2.prod') as file:
        for line in file:
            line = line.strip()
            if (len(threelines)>2):
               common = set(threelines[0]).intersection(threelines[1]).intersection(threelines[2])
               sum+=charToValue(list(common)[0])
               threelines=[]
            threelines.append(line)
        common = set(threelines[0]).intersection(threelines[1]).intersection(threelines[2])
        sum+=charToValue(list(common)[0])
        threelines=[]
    print(sum)

if __name__ == '__main__':
    main()
