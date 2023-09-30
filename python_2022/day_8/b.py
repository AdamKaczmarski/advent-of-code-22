rows = []
def get_line(direction,idx_col, idx_row):
    line = []
    if direction=="N":
        for i in range(idx_row-1,-1,-1):
            line.append(rows[i][idx_col])
    if direction=="S":
        for i in range(idx_row+1,len(rows),1):
            line.append(rows[i][idx_col])
    if direction=="W":
        line=rows[idx_row][:idx_col]
        line.reverse()
    if direction=="E":
        line=rows[idx_row][idx_col+1:]
    return line

def check_visibility(treerow,treecol):
    tree = rows[treerow][treecol]
    total=1
    for x in ["N","S","E","W"]:
        score=0
        line_oftrees:list = get_line(x,treecol,treerow)
        for i in line_oftrees:
            score+=1
            if (i>=tree):
                break
        total*=score
    return total


def main():
    highestVis = 0
    with  open('./input.prod') as file:
        linecount=0
        for line in file:
            line = line.strip()
            rows.append(list(line))
            linecount+=1
    for i in range(len(rows)):
        for k in range(len(rows[i])):
            visscore=check_visibility(i,k)
            if(highestVis<visscore):
                highestVis=visscore
    print(highestVis)
if __name__ == '__main__':
    main()

