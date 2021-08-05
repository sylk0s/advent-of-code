fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
passport = {"byr":False, "iyr":False, "eyr":False, "hgt":False, "hcl":False, "ecl":False, "pid":False}

valid = 0
count = 0

for line in open("Day-4/day4.txt"):
    elements = line.split()
    for element in elements:
        if element[0:3] in fields:
            passport[element[0:3]] = True # why tf is this 0:3 instead of 0:2

    if line == "\n":
        satisfied = True
        for field in fields:
            if not passport[field]:
                satisfied = False
            passport[field] = False
        if satisfied:
            valid += 1

satisfied = True
for field in fields:
    if not passport[field]:
        satisfied = False
    passport[field] = False
if satisfied:
    valid += 1
#there is definitly a smarter way to do this but... it works. Basically what happens here is that it doesn't get the last passport unless i iterate a final time

#print(valid)

#im gonna just rewite it for the second challenge in a better way
for field in fields:
    passport[field] = False

data = []
for line in open("Day-4/day4.txt"):
    if line == "\n":
        data.append("\n")
    else:
        vals = line.split()
        for val in vals:
            data.append(val)

data.append("\n")

passports = []
passport = {}

for val in data:
    if val == "\n":
        passports.append(passport)
        passport = {}

    else:
        elements = val.split(":")
        passport[elements[0]] = elements[1]

valid1 = 0
valid2 = 0

for passport in passports:
    complete = True
    for cat in fields:
        if not cat in passport:
            complete = False

    if complete:
        valid1 += 1
        byr = int(passport["byr"]) >= 1920 and int(passport["byr"]) <= 2002
        iyr = int(passport["iyr"]) >= 2010 and int(passport["iyr"]) <= 2020
        eyr = int(passport["eyr"]) >= 2020 and int(passport["eyr"]) <= 2030
        hgt = (passport["hgt"][-2:] == "cm" and (150 <= int(passport["hgt"][:-2]) <= 193)) or (passport["hgt"][-2:] == "in" and (59 <= int(passport["hgt"][:-2]) <= 76))

        if passport["hcl"].startswith("#") and (len(passport["hcl"]) == 7):
            isHex = True
            for i in range(1,6):
                if not passport["hcl"][i] in ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f']:
                    isHex = False
        hcl = isHex

        ecl = passport["ecl"] in ['amb','blu','brn','gry','grn','hzl','oth']
        if len(passport["pid"]) == 9:
            isNum = True
            for i in range(0,8):
                if not passport["pid"][i] in ['0','1','2','3','4','5','6','7','8','9']:
                    isNum = False
            pid = isNum

        #hcl and pid are issues... some weird hcl stuf... idk 

        if byr and iyr and eyr and hgt and hcl and ecl and pid:
            valid2 += 1

print(valid1)
print(valid2)
print(len(passports))