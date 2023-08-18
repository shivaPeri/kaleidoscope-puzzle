import json

# input_path = '../boards/boards.json'
# output_path = '../boards/boards_1.json'
input_path = '../boards/scraped-boards.json'
output_path = '../boards/boards_2.json'

# Read the JSON file
with open(input_path, 'r') as fd:
    data = json.load(fd)

with open(output_path, 'w') as fd:
    fd.write("{\n")
    for i, (key, value) in enumerate(data.items()):
        newVal = ""
        for c in value:
            if c == "1": newVal += "00"
            if c == "2": newVal += "01"
            if c == "3": newVal += "10"
            if c == "4": newVal += "11"

        fd.write(f"\t\"{key}\": \"{newVal}\"")
        if i == len(data)-1: fd.write("\n")
        else: fd.write(",\n")

    fd.write("}")
