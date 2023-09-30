def main():
    max_cal = 0
    tmpSum = 0
    with  open('./input.txt') as file:
        for line in file:
            if line.strip() =="":
                if tmpSum>max_cal:
                    max_cal = tmpSum
                tmpSum=0
            else:
                line = line.strip()
                tmpSum += int(line)
    print(max_cal)




if __name__ == '__main__':
    main()
