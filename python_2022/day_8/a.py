#Forgive me as I have sinned
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
    if direction=="E":
        line=rows[idx_row][idx_col+1:]
    #print("SORT")
    #print(line)
    #print(sorted(line, reverse=True))
    #print("END SORT")
    return sorted(line, reverse=True)

def check_visibility(treecol,treerow):
    tree = rows[treerow][treecol]
    #print("TREE: "+tree)
    line_oftrees:list = get_line("N",treecol,treerow)
    #print(str(line_oftrees))
    if (len(line_oftrees)==0):
        return True
    for i in line_oftrees:
        if (i<tree):
            return True
        else:
            break
    line_oftrees = get_line("S",treecol,treerow)
    if (len(line_oftrees)==0):
        return True
    #print(str(line_oftrees))
    for i in line_oftrees:
        if (i<tree):
            return True
        else:
            break
    line_oftrees = get_line("W",treecol,treerow)
    if (len(line_oftrees)==0):
        return True
    #print(str(line_oftrees))
    for i in line_oftrees:
        if (i<tree):
            return True
        else:
            break
    line_oftrees = get_line("E",treecol,treerow)
    if (len(line_oftrees)==0):
        return True
    #print(str(line_oftrees))
    for i in line_oftrees:
        if (i<tree):
            return True
        else:
            break
    return False
    



def main():
    visibletrees = 0
    with  open('./input.prod') as file:
        for line in file:
            line = line.strip()
            rows.append(list(line))
    #visibletrees+=len(rows)*2+len(rows[0])*2-4
    for i in range(len(rows)):
        for k in range(len(rows[i])):
            if(check_visibility(k,i)):
                visibletrees+=1
                #print("VISIBLE TREES: "+str(visibletrees))
    #print(rows)
    #print(visibletrees)

if __name__ == '__main__':
    main()

