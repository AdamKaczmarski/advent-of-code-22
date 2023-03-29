import ast

def comparelists2( left, right):
    print("left{}  right{}".format(left,right))
    for (idx,element) in enumerate(right):
        if idx>=len(left):
            print("Left side is smaller, so inputs are in the right order")
            return True
        print("LEFTELEMENT {} RIGHTELEMENT {}".format(left[idx],element))
        if isinstance(element,(list)) and not isinstance(left[idx],(list)):
            #convert to (list)

            left[idx]= list(map(int,str(left[idx])))
        if isinstance(left[idx],(list)) and not isinstance(element,(list)):
            #convert to (list)
            element= list(map(int,str(element)))
        print("comparing {} of type {} with {} of type {}".format(left[idx],type(left[idx]),element,type(element)))
        if isinstance(left[idx],(list)) and isinstance(element,(list)):
            print("BOTH ARE LISTS")
            return comparelists(left[idx],element)
            #if not comparelists(left[idx],element):
            #    break 
            #return True
        elif element==left[idx]:
            continue
        elif element<left[idx]:
            print("LEFT > RIGHT")
            return True
        else: 
            return False
    return not len(left)>len(right)

def comparelists( left, right):
    print("left{}  right{}".format(left,right))
    for (idx,element) in enumerate(right):
        if idx>=len(left):
            print("Left side is smaller, so inputs are in the right order")
            return True
        print("LEFTELEMENT {} RIGHTELEMENT {}".format(left[idx],element))
        if isinstance(element,(list)) and not isinstance(left[idx],(list)):
            #convert to (list)

            left[idx]= list(map(int,str(left[idx])))
        if isinstance(left[idx],(list)) and not isinstance(element,(list)):
            #convert to (list)
            element= list(map(int,str(element)))
        print("comparing {} of type {} with {} of type {}".format(left[idx],type(left[idx]),element,type(element)))
        if isinstance(left[idx],(list)) and isinstance(element,(list)):
            print("BOTH ARE LISTS")
            return comparelists(left[idx],element)
            #if not comparelists(left[idx],element):
            #    break 
            #return True
        elif element==left[idx]:
            continue
        elif element<left[idx]:
            print("LEFT > RIGHT")
            return True
        else: 
            return False
    return not len(left)>len(right)


def main():
    packets=[]
    sum_of_idx = 0
    with open("./input.prod") as file:
        for packet in file.read().split("\n\n"):
            packet = packet.split("\n")
            packet[0] = ast.literal_eval(packet[0])
            packet[1] = ast.literal_eval(packet[1])
            packets.append(packet)
    for (idx, packets)  in enumerate(packets):
        print("######idx: {}, packets: {}".format(idx,packets))
        if comparelists2(packets[0],packets[1]):
            print("----------------------THISISGOOD LEFT{} RIGHT{} IDX{}".format(packets[0],packets[1],idx+1))
            sum_of_idx+=idx+1
    print(sum_of_idx)



if __name__ == '__main__':
    main()

