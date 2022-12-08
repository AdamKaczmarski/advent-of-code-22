def main():
    
    with  open('./input.prod') as file:
        tree={}
        cwd=""
        for line in file:
            line = line.strip()
            elements = line.split(' ')
            if elements[0]=="$":
                #command
                if elements[1]=="ls":
                    pass
                elif elements[1]=="cd":
                    if cwd=="":
                        cwd+="/"
                    elif elements[2]=="..":
                        cwd=cwd[:cwd.rindex("/")]
                        if cwd=="":
                            cwd+="/"
                    else:
                        if (cwd[len(cwd)-1]!="/"):
                            cwd+="/"
                        cwd+=elements[2]
                    if tree.get(cwd) is None:
                        tree[cwd]=0
                    
            elif elements[0]=="dir":
                #dir
                pass
            else:
                #file
                if tree.get(cwd) is None:
                    tree[cwd]=int(elements[0])
                else:
                    tree[cwd]=tree.get(cwd)+int(elements[0])
                if cwd!="/":
                    tmp = cwd
                    while(tmp!=""):
                        tmp=tmp[:tmp.rindex("/")]
                        print(tmp)
                        if tmp=="":
                            tree["/"]=tree.get("/")+int(elements[0])
                        else:
                            print(tmp,elements[0])
                            print(tree.get(tmp)+int(elements[0]))
                            tree[tmp]=tree.get(tmp)+int(elements[0])
        print(tree)
        sum=0
        for k,v in tree.items():
            if v<=100000:
                sum+=v
        print(sum)





if __name__ == '__main__':
    main()
