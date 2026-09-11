#!/bin/bash

# resize
printf '\e[8;'42';'132't'
f_cls() {

reset > /dev/null
    echo -en '\033[1;1H'
    echo -en '\033]11;#000000\007'
    echo -en '\033]10;#FFFFFF\007'
}

f_cls
LIBPROJECT="$HOME/Zrust/gen_doc/"

cd $LIBPROJECT

./gendoc

