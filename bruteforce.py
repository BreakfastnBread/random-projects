from distutils.util import subst_vars


guess_word = "sus"
guess_count = 6
while True:
    guess = input("> ")
    if guess == guess_word:
        print("You got it right!")
        break
    else:
        guess_count -= 1
        print("Try again!", guess_count)
    if guess_count < 1:
        print("out of tries!")
        break