

import json

with open("../tiles/data.json", "r") as f:

    data = json.load(f)
    f.close()

for tile in data["tiles"]:
    
    tile["wall"] = True



with open("data2.json","w") as f:

    json.dump(data,f,indent=4)

    f.close()
