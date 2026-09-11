#!/bin/bash

faStabilo='\033[7m'
fcRouge='\033[31m'
f_pause(){
    echo -en '\033[0;0m'
     echo -en $faStabilo$fcRouge'Press[Enter] key to continue'
    tput civis     # curseur invisible
    read -s -n 1
    echo -en '\033[0;0m'
}

f_cls() {

reset > /dev/null
    echo -en '\033[1;1H'
    echo -en '\033]11;#000000\007'
    echo -en '\033]10;#FFFFFF\007'
}

f_cls
# Chemin vers la base SQLite
DB_PATH="./sqlite/help_pgm.db"

# Vérifier que sqlite3 est installé
if ! command -v sqlite3 &> /dev/null; then
    echo "Erreur : sqlite3 n'est pas installé."
    f_pause
    exit 1
fi

# Vérifier que la base existe
if [ ! -f "$DB_PATH" ]; then
    echo "Erreur : La base de données $DB_PATH n'existe pas."
    f_pause
    exit 1
fi

# Vérifier qu'un nom de programme est fourni
if [ -z "$1" ]; then
    echo "Usage: $0 <nom_du_programme>"
    f_pause
    exit 1
fi

programme="$1"


# Vérifier que le programme existe dans la base
if ! sqlite3 "$DB_PATH" "SELECT 1 FROM help WHERE programme = '$programme' LIMIT 1;" | grep -q "1"; then
    echo "Erreur : Le programme '$programme' n'existe pas dans la base."
    f_pause
    exit 1
fi


# Supprimer help_pgm.txt s'il existe
if [ -f "help_pgm.txt" ]; then
    rm -f "help_pgm.txt"
fi



# Écrire directement dans help_pgm.txt (sans ajouter de ligne supplémentaire)
sqlite3 "$DB_PATH" -separator $'\t' "SELECT code_attribut, text FROM help WHERE programme = '$programme' ORDER BY ligne;" | \
while IFS=$'\t' read -r code_attribut text; do
    echo "$code_attribut $text" >> help_pgm.txt
done

# Ouvrir avec Mousepad
mousepad ./help_pgm.txt