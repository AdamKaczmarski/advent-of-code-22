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
    with  open('./input1.prod') as file:
        for line in file:
            line = line.strip()
            half = int(len(line)/2)
            left = line[0:half]
            right = line[half:]
            common = set(right).intersection(left) 
            if (len(common)>1):
                print("Something might be wrong, length of common is bigger than 1")
            else:
                sum+=charToValue(list(common)[0])
    print(sum)

if __name__ == '__main__':
    main()
