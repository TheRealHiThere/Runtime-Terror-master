import RT

print("run rt -h for help")

running = True
while running:
    command = input("Runtime Terror Shell> ")

    result, error = RT.run('<stdin>', command)

    if error: print(error.as_string())
    else: print(result)

    if command == "rt":
        print("Runtime Terror Shell")
        print("Version 1.0.0")
        print("run rt -h for help")
        print("                                                                        ")
        print("                                 .#######                               ")
        print("                      (          #######  /                             ")
        print("                 .////(         #######   ////(                         ")
        print("             /////////(        #######    /(((((((((                    ")
        print("        ,/////////////        ########     (((((((((((((,               ")
        print("     ////////////             #######           (((((((((//(            ")
        print("     ///////(                #######                ///////(            ")
        print("     ///(((((((((/          #######            *////////////            ")
        print("         (((((((((((((     #######,        (////////////                ")
        print("             *(((((((/(   .#######        ////////(,                    ")
        print("                  /(///   #######         ////(                         ")
        print("                      .  #######                                        ")
        print("                        #######                                         ")

    elif command == "rt -e":
        running = False

    elif command == "rt -cl":
        print("\033c")

    elif command == "rt -v":
        print("Runtime Terror v1.0")

    elif command == "rt -c":
        print("Runtime Terror v1.0")
        print("Created by: TheDevConnor")
        print("Github: TheRealHiThere")

    elif command == "rt -h":
        print("rt -e  <=> Exit Runtime Terror Shell")
        print("rt -cl <=> Clear the screen")
        print("rt -h  <=> Display this message")
        print("rt -v  <=> Display the version of Runtime Terror Shell")
        print("rt -c  <=> Display the credits of Runtime Terror Shell")
