import re

class Planet:
    def __init__(self, name: str, x=0, y=0, z=0, vx=0, vy=0, vz=0):
        self.name = name
        self.x = x
        self.y = y
        self.z = z
        self.vx = vx
        self.vy = vy
        self.vz = vz

    def apply_gravity(self, planets: list):
        for planet in planets:
            if self.x != planet.x:
                if planet.x > self.x:
                    self.vx += 1
                else:
                    self.vx -= 1
            if self.y != planet.y:
                if planet.y > self.y:
                    self.vy += 1
                else:
                    self.vy -= 1

            if self.z != planet.z:
                if planet.z > self.z:
                    self.vz += 1
                else:
                    self.vz -= 1

    def apply_velocity(self):
        self.x += self.vx
        self.y += self.vy
        self.z += self.vz

    def calc_energy(self):
        pot = abs(self.x) + abs(self.y) + abs(self.z)
        kin = abs(self.vx) + abs(self.vy) + abs(self.vz)
        return pot * kin


# Quest input
initial_state_str = """<x=17, y=5, z=1>
<x=-2, y=-8, z=8>
<x=7, y=-6, z=14>
<x=1, y=-10, z=4>"""
init = [int(s) for s in re.findall(r'-?\d+\.?\d*', initial_state_str)]

# Initialize planets
planet_names = ['Io', 'Europa', 'Ganymede', 'Callisto']
planets = []
for planet in planet_names:
    exec('%s = Planet(planet, %d, %d, %d)' % (planet, init.pop(0), init.pop(0), init.pop(0)))
    exec('planets.append(%s)' % (planet))

iterations = 1
while iterations <= 1000:
    print(iterations)
    for planet in planets:
        planet.apply_gravity(planets)

    for planet in planets:
        planet.apply_velocity()
        print(planet.x, planet.y, planet.z)

    iterations += 1

energy = 0
for planet in planets:
    energy += planet.calc_energy()

print(f'Energy: {energy}')
