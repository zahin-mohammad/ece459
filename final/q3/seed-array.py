
iterations = 10000000


blah = []
counter = 0
for i in range(iterations):
    blah.append(counter)
    counter = (counter * 5 + 13) % 17
    if counter == 0:
        break
print(blah)
print(len(blah))

blah = []
counter = 0
for i in range(iterations):
    blah.append(counter)
    counter = (counter * 9 + 17) % 31
    if counter == 0:
        break
print(blah)
print(len(blah))