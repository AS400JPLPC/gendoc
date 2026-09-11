#!/bin/bash

# Couleurs et styles
faStabilo='\033[7m'
fcRouge='\033[31m'
f_pause() {
    echo -en '\033[0;0m'
    echo -en "$faStabilo$fcRouge Press[Enter] key to continue"
    tput civis  # Curseur invisible
    read -s -n 1
    echo -en '\033[0;0m'
}

f_cls() {
    reset > /dev/null
    echo -en '\033[1;1H'
    echo -en '\033]11;#000000\007'  # Fond noir
    echo -en '\033]10;#FFFFFF\007'  # Texte blanc
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


# Fonction pour lister les programmes disponibles
list_programmes() {
    echo "Programmes disponibles dans la base :"
    sqlite3 "$DB_PATH" "SELECT DISTINCT programme FROM help ORDER BY programme;" | \
    awk '{print NR ". " $0}'
}

# Afficher le menu
echo "=== Menu d'aide ==="
list_programmes

# Boucle principale
while true; do
    read -p "Entrez le numéro du programme pour afficher son aide (ou 'q' pour quitter) : " choice

    if [[ "$choice" == "q" ]]; then
        exit 0
    fi

    # Récupérer le programme correspondant au numéro choisi
    programme=$(sqlite3 "$DB_PATH" "SELECT DISTINCT programme FROM help ORDER BY programme;" | sed -n "${choice}p")

    if [ -z "$programme" ]; then
        echo "Numéro invalide. Veuillez réessayer."
    else
        $HOME/Zrust/gen_doc/TermDspDoc "$programme"
    fi
done