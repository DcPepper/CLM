from random import randint

MENU = """DUEL à l'épée ! Souhaitez-vous attaquer (1) ou utiliser une potion (2) ? """

MENU_CHOICE = ["1", "2"]

choice_user = ""
user_life = 50
computer_life = 50
potion = 3

while user_life >0 or computer_life > 0:
    choice_user = input(MENU)
    if choice_user not in MENU_CHOICE:
        print("Choisissez un nombre entre 1 et 2 pour effectuer une action !")
        continue
    if choice_user == "1":
        user_attack = randint(5,10)
        computer_life -= user_attack
        if computer_life > 0:
            computer_attack = randint(5,15)
            user_life -= computer_attack
            print(f"""Vous avez infligé {user_attack} points de dégat à l'ennemi!
L'ennemi vous a infligé {computer_attack} dégats!
Il vous reste {user_life} points de vie !
Il reste {computer_life} points de vie à l'ordinateur !
                  """)
    if choice_user == "2":
        if potion == 0:
            print("Choisissez une autre option!")
            continue
        if potion > 0:
            potion -= 1
            healing_potion = randint(15,50)
            user_life += healing_potion
            computer_attack = randint(5,15)
            user_life -= computer_attack
            print(f"Votre potion vous à rendu {healing_potion} et il vous en reste {potion}. Vous avez désormais {user_life}")
            print(f"Vous avez perdu {computer_attack} points de vie suite à l'attaque de l'ordinateur")
            
                     
            print("Vous passez votre tour et votre ennemi vous attaque à nouveau")
            computer_attack = randint(5,15)
            user_life -= computer_attack
            print(f"Il vous reste {user_life} points de vie suite à l'attaque de {computer_attack} de l'ordinateur !")
            print(f"Il reste {computer_life} points de vie à l'ordinateur !")    
    if computer_life <= 0:
        print("Vous avez vaincu votre ennemi ! Félicitations!")
        break
    if user_life <= 0:
        print("Vous avez perdu! Recommencez une autre partie pour tenter de gagner !")
        break


dico = {
    0: {
        "prenom": "Thomas",
        "niveau": "Bronze",
        "talent": "néant"
    }, 
    1: {
        "prenom": "Mikaël",
        "niveau": "Radiant",
        "talent": "PGM"
    }
}