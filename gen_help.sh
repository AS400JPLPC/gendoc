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
    echo "Erreur : sqlite3 n'est pas installé. Installez-le avec :"
    echo "  sudo apt-get install sqlite3   # Pour Debian/Ubuntu"
    echo "  sudo dnf install sqlite        # Pour Fedora"
    echo "  sudo pacman -S sqlite          # Pour Arch"
    f_pause
    exit 1
fi

# Vérifier que la base existe
if [ ! -f "$DB_PATH" ]; then
    echo "Erreur : La base de données $DB_PATH n'existe pas."
    f_pause
    exit 1
fi

# Fonction pour afficher l'aide d'un programme
display_help() {
    local programme="$1"
    echo "=== Aide pour le programme : $programme ==="
    sqlite3 "$DB_PATH" "SELECT ligne, code_attribut, text FROM help WHERE programme = '$programme' ORDER BY ligne;" | \
    while IFS="|" read -r ligne code_attribut text; do
        # Remplacer les sauts de ligne ou espaces multiples
        text=$(echo "$text" | sed 's/  */ /g')
        # Afficher avec un formatage basique
        case "$code_attribut" in
            "*") echo -e "\033[1m$text\033[0m" ;;  # Gras pour les titres
            "!") echo -e "\033[33m$text\033[0m" ;; # Jaune pour les remarques
            "-") echo -e "  - $text" ;;           # Liste à puces
            ".") echo "  $text" ;;                # Texte normal
            *) echo "$text" ;;
        esac
    done
    echo
}

# Fonction pour lister les programmes disponibles
list_programmes() {
    echo "Programmes disponibles dans la base :"
    sqlite3 "$DB_PATH" "SELECT DISTINCT programme FROM help ORDER BY programme;" | \
    awk '{print NR ". " $0}'
}

# Afficher le menu
echo "=== Menu d'aide ==="
list_programmes

# Demander à l'utilisateur de choisir un programme
read -p "Entrez le numéro du programme pour afficher son aide (ou 'q' pour quitter) : " choice

if [[ "$choice" == "q" ]]; then
    exit 0
fi

# Récupérer le nom du programme correspondant au choix
programme=$(sqlite3 "$DB_PATH" "SELECT DISTINCT programme FROM help ORDER BY programme LIMIT 1 OFFSET $((choice - 1));")

if [ -z "$programme" ]; then
    echo "Numéro invalide."
    f_pause
    exit 1
fi

# Afficher l'aide pour le programme sélectionné
./export_help.sh "$programme"

