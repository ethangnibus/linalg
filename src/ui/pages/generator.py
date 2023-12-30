import os

def main():
    dir_list = os.listdir(".")
    print("List of directories and files before creation:")
    print(dir_list)
    print()

    for ch_num in range(0, 17):
        for sec_num in range(0, 5):
            for subsec_num in range(0, 10):
                filename = "chapter" + str(ch_num) + "section" + str(sec_num) + "subsection" + str(subsec_num) + ".rs"
                with open(filename, 'w') as fp:
                    pass

    dir_list = os.listdir(".")
    print("List of directories and files after creation:")
    print(dir_list)
if __name__ == '__main__':
    main()
