# Matrikel-Nr.: 1811609
import matplotlib.pyplot as plt
import math

def read_data(file_name):
    x, y, z = [], [], []
    with open(file_name) as f:
        for line in f.readlines():
            data = line.strip().split(";")
            x.append(float(data[0]))
            y.append(float(data[1]))
            z.append(float(data[2]))
    return x, y, z


def knn(k, x, y, z):
    r = [x*0.02 for x in range(-50, 50)]
    new_x, new_y, new_z = [], [], []

    for i in r:
        for j in r:
            neighbors = {distance(x[n], y[n], i, j): z[n] for n in range(0, len(x))}
            keys = sorted(neighbors.keys())[0:k]
            colors = [neighbors.get(key) for key in keys]
            red, blue = 0, 0
            for c in colors:
                if c == -1:
                    red += 1
                else:
                    blue += 1
            c = -1 if red > blue else 1
            new_x.append(i)
            new_y.append(j)
            new_z.append(c)
    
    return x + new_x, y + new_y, z + new_z

def distance(x, y, i, j):
    return math.sqrt((x-i)**2 + (y-j)**2)

def make_image(x, y, z):
    colors = list(map(lambda x: 'r' if x == -1 else 'b', z))
    plt.scatter(x, y, c=colors)
    plt.savefig("output.png")

k = 5
file_name = "spiral.txt"

x, y, z = read_data(file_name)
x, y, z = knn(k, x, y, z)
make_image(x, y, z)
