# A - Rock B - Paper C - Scissors
# X - Rock Y - Paper Z - Scissors
def cntr(games):
    score=0
    for k,v in games.items():
        for x,y in v.items():
            if k=='A':
                if x=='Y':
                    score+=y*6
                elif x=='X':
                    score+=y*3
            elif k=='B':
                if x=='Z':
                    score+=y*6
                elif x=='Y':
                    score+=y*3
            else:
                if x=='X':
                    score+=y*6
                elif x=='Z':
                    score+=y*3
            if x=='X':
                score+=y*1
            elif x=='Y':
                score+=y*2
            else:
                score+=y*3
    return score


def main():
    games = {}
    games['A'] = {}
    games['B'] = {}
    games['C'] = {}
    with  open('./input_1.txt') as file:
        for line in file:
            line = line.strip()
            forms = line.split(' ')
            if games.get(forms[0]).get(forms[1]) is None:
                games.get(forms[0])[forms[1]]=1
            else:
                games.get(forms[0])[forms[1]]+=1
        print(cntr(games))




if __name__ == '__main__':
    main()
