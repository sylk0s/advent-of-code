ruleSet = []

for line in open("Day-7/day7.txt"):
    words = line.split()
    rule = []
    for word in words:
        if not word in ["bags", "bag,", "contain", "bag.", "bags.", "bags,", "no", "other"]:
            rule.append(word)
    ruleSet.append(rule)

# Now rule#et is a list of rules following this convention 
# [ "bag adjective", "bag color", "num", "child adjective", "child color"] (repeats last 3 for each child bag)
count1 = 0

#for rule in ruleSet:
#    if "shiny" in rule[2:] and "gold" in rule[2:] and rule.index("shiny")+1 == rule.index("gold"):
#        count1 += 1

# this^ wont account for nested gold bags

parsedRuleList = []

for rule in ruleSet:
    tempBagName = []
    tempRuleArgs = []

    tempList = rule[2:] + ["end"]

    for word in tempList:
        if not (tempList.index(word)+3)%3 == 0:
            tempBagName.append(word)
        else:
            if tempBagName:
                tempRuleArgs.append(tempBagName[0] + " " + tempBagName[1])
            tempBagName = []

    parsedRule = { "Name" : rule[0] + " " + rule[1], "Rule Args" : tempRuleArgs}
    parsedRuleList.append(parsedRule)

def hasGold(rule):
    if 'shiny gold' in rule['Rule Args']:
        #print("Contains Gold: " + str(rule))
        return True
    else:
        has = False
        for arg in rule['Rule Args']:
            for parsedRule in parsedRuleList:
                if parsedRule['Name'] == arg:
                    has = hasGold(parsedRule)
        #if has:
            #print("subset of :" + str(rule))
        return has

#28 is low
#584 is high

# I think I know the issue here... Imagine a list has two elements.... it travels down the first one and on the second recursion it is empty, it will return early and mess up

for rule in parsedRuleList:
    if hasGold(rule):
        count1 += 1


print("Part 1: " + str(count1))