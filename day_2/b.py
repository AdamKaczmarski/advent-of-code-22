
# A - Rock B - Paper C - Scissors
# X - Lose Y - Draw Z - Win

def main():
    score=0
    combinations = {
        "A X":3,
        "A Y":1+3,
        "A Z":2+6,
        "B X":1,
        "B Y":2+3,
        "B Z":3+6,
        "C X":2,
        "C Y":3+3,
        "C Z":1+6,
    }
    with  open('./input_2.txt') as file:
        for line in file:
            line = line.strip()
            score+=combinations[line]

        print(score)




if __name__ == '__main__':
    main()
