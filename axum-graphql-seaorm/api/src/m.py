def scramble(s1:str, s2:str)->bool:
    dico2 = {}
    dico1 = {}
    for lettre in set(s2):
        dico2[lettre] = s2.count(lettre)
    for lettre in set(s1):
        dico1[lettre] = s1.count(lettre)
    for cle, value in dico2.items():
        print(dico1)
        print(cle)
        if dico2[cle] >= dico1[cle]:
            continue
        else:
            return False
    return True

scramble("rokdwq", "world")