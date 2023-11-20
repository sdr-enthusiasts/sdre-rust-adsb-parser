#!/bin/python

# open aircraft.json, which is a json file containing all aircrafts
# loop through each entry and find all of the unique fields
# output the unique fields to a file called fields.txt
# if a field is not always present, tag it with Optional

import json

with open("aircraft.json") as f:
    data = json.load(f)

fields = set()
fields_unique = set()

for aircraft in data["aircraft"]:
    for field in aircraft:
        # if this is not the first aircraft, loop through all of the fields to see if we've not seen them before
        if len(fields) > 0:
            for f, _ in fields:
                if f not in aircraft:
                    fields_unique.add(f)
        # get the type of the field data
        field_type = type(aircraft[field])
        fields.add((field, str(field_type)))

with open("fields.txt", "w") as f:
    for field, type_of_field in fields:
        f.write(field + " " + type_of_field)
        if field in fields_unique:
            f.write(" (Optional)")
        f.write("\n")
