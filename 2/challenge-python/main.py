#!/usr/bin/env python
from dataclasses import dataclass
from functools import partial, reduce

import sys

@dataclass
class Cubeset:
    red: int = 0
    green: int = 0
    blue: int = 0

    def passes(self, filter):
        if (self.blue <= filter.blue and
                self.red <= filter.red and
                self.green <= filter.green):
            print(f"Passes: blue is {self.blue}")
            return True
        return False

class Game:
    num = 0

    def __init__(self, line):
        self.plays: list[Cubeset] = []
        self.line = line
        prefix, plays_raw = line.split(": ")
        self.num = int(prefix.split()[1])
        print(f"Doing Game {self.num}")
        plays_str = plays_raw.split("; ")
        for play_str in plays_str:
            cubecounts = play_str.split(", ")
            cubeset = Cubeset()
            #print(cubeset)
            for cubecount in cubecounts:
                print(f"play includes {cubecount}")
                ccs = cubecount.split(" ")
                print("ccss ", ccs)
                num_s, color = ccs
                num = int(num_s)
                match color:
                    case "red":
                        cubeset.red = num
                    case "blue":
                        print(f"Found blue with {num_s}")
                        cubeset.blue = num
                    case "green":
                        cubeset.green = num
            self.plays.append(cubeset)

    def select_possible(self, filter: Cubeset) -> int:
        for cs in self.plays:
            if not cs.passes(filter):
                #print("Failed: ", self.line)
                return 0
        return self.num

    def minimum_set(self) -> Cubeset:
        soln = Cubeset()
        for cs in self.plays:
            soln.red = max(soln.red, cs.red)
            soln.green = max(soln.green, cs.green)
            soln.blue = max(soln.blue, cs.blue)
        return soln


if __name__ == "__main__":
    with open(sys.argv[1]) as f:
        lines = f.readlines()
        filter = Cubeset(12,13,14)
        print(filter)
        #sum = reduce(lambda a, b: a+b, [Game(line).select_possible(filter) for line in lines])
        print("Solving day 2 part 1\n\n")
        sum = 0
        power_sum = 0
        for line in lines:
            game = Game(line.strip())
            game_poss = game.select_possible(filter)
            if game_poss > 0:
                sum += game_poss
                print(f"game_poss is {game_poss}")
                print(f"cum_sum is {sum}")
            min_set = game.minimum_set()
            power = min_set.blue * min_set.red * min_set.green
            power_sum += power
        print(sum)
        print(f"Part 2: {power_sum}")
