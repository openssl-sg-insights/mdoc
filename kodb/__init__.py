import sys
import os
from kodb.download import check_program_availability, download_dependencies
from kodb.build import build_document
from kodb.make_project import make_project

def main():
    if len(sys.argv) == 1:
        check_program_availability()
        help()

    elif sys.argv[1] in ["--help", "-h"]:
        help()

    elif sys.argv[1] == "build":
        check_program_availability()
        build_document()

    elif sys.argv[1] == "init":
        make_project(".")

    elif sys.argv[1] == "new":
        try:
            os.mkdir(sys.argv[2])
            make_project(sys.argv[2])
        except IndexError:
            print("A directory name is required as an argument. Run this command like 'kodb new <name>'.")

    elif sys.argv[1] == "--download-dependencies":
        download_dependencies()
        
def help():
    print("""Welcome to kodb!""")