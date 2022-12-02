import math
# class Chemistry:
#     def __init__(self,filename):
#         self.filename = filename

receipt = dict()


print('hello')

class Chemistry():
    def __init__(self, filename):
        self.filename = filename
        self.receipt = self.load_data()
        self.ore_mined = 0
        self.ore_used = 0
        self.ore_reserve = dict()
        # self.show_receipt()
        self.calc_needed_ore("FUEL", 1)

    def load_data(self):
        with open(self.filename, 'r') as f:
            lines = f.read().split('\n')
            for line in lines:
                linesplit = line.replace(',', '').replace('=>', '').split(' ')
                receipt[linesplit[-1]] = [linesplit[-2], linesplit[0:-3]]
        return receipt

    def show_result(self):
        print('=====================================')
        print(f'ORE USED: {self.ore_used+sum(self.ore_reserve.values())}')
        return self.ore_used+sum(self.ore_reserve.values())

    def show_receipt(self):
        for entry in self.receipt:
            print(f'{entry} - {self.receipt[entry]}')
        print('=====================================')

    def calc_needed_ore(self, result, amount_of_result=1):
        ingredients = self.receipt[result][1]
        print(f'{amount_of_result} {result} from {ingredients}')

        # LOOP OVER INGREDIENTS
        for i in range(0, len(ingredients)-1, 2):
            amount_of_ingredient = int(ingredients[i])
            ingredient = ingredients[i+1]
            #  print(f'Ingredient {int((i+2)/2)}:  {amount_of_ingredient} {ingredient}')

            if ingredient == 'ORE':
                try:
                    reserve = self.ore_reserve[result]
                except:
                    self.ore_reserve[result] = 0
                    reserve = 0

                self.ore_used += amount_of_result
                if amount_of_result < reserve:
                    self.ore_mind = 0
                else:
                    self.ore_mined = int(math.ceil((amount_of_result-reserve) / amount_of_ingredient)*amount_of_ingredient)
                reserve += self.ore_mined - amount_of_result

                self.ore_mined = 0
                self.ore_reserve[result] = reserve
            else:
                self.calc_needed_ore(ingredient, amount_of_result * amount_of_ingredient)

        #  print('Ingredients finished.')


# input1 = Chemistry('Day14\input1.txt')
# input1.show_result()

input2 = Chemistry('Day14\input2.txt')
input2.show_result()
