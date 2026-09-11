#!/bin/bash
faStabilo='\033[7m'
fcRouge='\033[31m'
fcJaune='\033[33;1m'
fcCyan='\033[36m'
fcGreen='\033[32m'
fcBleu='\033[34m'
fcNoir='\033[0;0m'

faGras='\033[1m'

#=========================
# function  menu
#=========================

f_cls() {

reset > /dev/null
    echo -en '\033[1;1H'
    echo -en '\033]11;#000000\007'
    echo -en '\033]10;#FFFFFF\007'
}

f_pause(){
    echo -en '\033[0;0m'
     echo -en $faStabilo$fcRouge'Press[Enter] key to continue'
    tput civis     # curseur invisible
    read -s -n 1
    echo -en '\033[0;0m'
}

f_dsplyPos(){ #commande de positionnement    lines + coln + couleur + text
    echo -en '\033[0;0m'
    let lig=$1
    let col=$2
    echo -en '\033['$lig';'$col'f'$3$4

}
f_readPos() {    #commande de positionnement    lines + coln + text
    echo -en '\033[0;0m'
    let lig=$1
    let col=$2
    let colR=$2+${#3}+1  # si on doit coller faire  $2+${#3}
    echo -en '\033['$lig';'$col'f'$fdVert$faGras$fcBlanc$3
    echo -en '\033[0;0m'
    tput cnorm    # curseur visible
     echo -en '\033['$lig';'$colR'f'$faGras$fcGreen
    read
    tput civis     # curseur invisible
    echo -en '\033[0;0m'
}

# resize
printf '\e[8;'30';'80't'

envCPP="1"
envRUST="5"
libRUST="6"
editing="30"
PROJECT="GEN-DOC"
LIBPROJECT="$HOME/Zrust/gen_doc/"
ALL_LIB="$HOME/Zrust/all_lib/"
GEN_LIB="$HOME/Zrust/gen_lib/"
choix=""

#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
# Vérifier que cargo est installé
if ! command -v cargo &> /dev/null; then
    echo -e "${fcRouge}Erreur : cargo n'est pas installé.${fcNoir}"
    exit 1
fi


# Fonction de nettoyage
cleanup() {
    tput sgr0 # Réinitialise les couleurs
    tput cnorm            # Affiche le curseur
    if [ -d "$LIBPROJECT/target" ]; then
        rm -rf "$LIBPROJECT/target"
    fi
    exit 0
}

# Intercepter les signaux de fermeture
trap cleanup SIGHUP SIGTERM SIGINT
#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

while [ "$choix" != "99" ]
do
    cd $LIBPROJECT
    f_cls
    f_dsplyPos  1  24 $faGras$fcJaune'Project: '$faGras$fcCyan$PROJECT

    f_dsplyPos  2  24 $faGras$fcJaune'------------compile cpp-----------------'
    f_dsplyPos  3  20 $faGras$fcRouge' 1.'; f_dsplyPos  3  24 $faGras$fcGreen 'TermDspDoc'
    
		f_dsplyPos  5  24 $faGras$fcJaune'------------compile Rust----------------'
    f_dsplyPos  6  20 $faGras$fcRouge'10.'; f_dsplyPos  6  24 $faGras$fcGreen 'gendoc		->gen_doc'
    f_dsplyPos  7  20 $faGras$fcRouge'11.'; f_dsplyPos  7  24 $faGras$fcGreen 'dspdoc		->gen_doc'
    f_dsplyPos  8  20 $faGras$fcRouge'12.'; f_dsplyPos  8  24 $faGras$fcGreen 'dltdoc		->gen_doc'    
    f_dsplyPos  9  24 $faGras$fcJaune '----------------------------------------'

    f_dsplyPos 10  20 $faGras$fcRouge'30.'; f_dsplyPos 10  24 $faGras$fcGreen 'Debug codelldb'

    f_dsplyPos 12  20 $faGras$fcRouge'50.'; f_dsplyPos 12  24 $faGras$fcCyan  'EDIT FILE all-lib'
    f_dsplyPos 13  20 $faGras$fcRouge'55.'; f_dsplyPos 13  24 $faGras$fcCyan  'EDIT FILE gen-lib'
    f_dsplyPos 14  20 $faGras$fcRouge'60.'; f_dsplyPos 14  24 $faGras$fcCyan  'EDIT FILE'
    
    
    f_dsplyPos 17  24 $faGras$fcBleu '----------------------------------------'    
    f_dsplyPos 18  20 $faGras$fcRouge'77.'; f_dsplyPos 18  24 $faGras$fcCyan  'cargo clean'

    f_dsplyPos 20  20 $faGras$fcRouge'88.'; f_dsplyPos 20  24 $faGras$fcGreen 'Console'
    
    f_dsplyPos 22  20 $faGras$fcRouge'90.'; f_dsplyPos 22  24 $faGras$fcGreen 'gen_doc  to sqlite'
    f_dsplyPos 23  20 $faGras$fcRouge'91.'; f_dsplyPos 23  24 $faGras$fcGreen 'gen_help via sqlite'
    f_dsplyPos 24  20 $faGras$fcRouge'92.'; f_dsplyPos 24  24 $faGras$fcGreen 'dlt_doc  via sqlite'
    f_dsplyPos 25  20 $faGras$fcRouge'93.'; f_dsplyPos 25  24 $faGras$fcGreen 'lst_doc  via sqlite'
    f_dsplyPos 26  24 $faGras$fcBleu '----------------------------------------'    
    f_dsplyPos 27  20 $faGras$fcRouge'99.'; f_dsplyPos 27 24 $faGras$fcRouge  'Exit'

    f_dsplyPos 28  24 $faGras$fcBleu '----------------------------------------'
    f_readPos  29  20  'Votre choix  :'; choix=$REPLY;

    # Recherche de caractères non numériques dans les arguments.
    if echo $choix | tr -d [:blank:] | tr -d [:digit:] | grep . &> /dev/null; then
        f_readPos 29 70  'erreur de saisie Enter'
    else

         case "$choix" in


# APPTERM
        1)
            $HOME/.Terminal/dispatch.sh $envRUST  $LIBPROJECT   "TermDspDoc" "Terminal"
        ;;

#bin
        10)
           $HOME/.Terminal/dispatch.sh $envRUST $LIBPROJECT   "gendoc" "gen-doc"
       ;;

#bin
        11)
           $HOME/.Terminal/dispatch.sh $envRUST $LIBPROJECT   "dspdoc" "gen-doc"
       ;;

#bin
        12)
           $HOME/.Terminal/dispatch.sh $envRUST $LIBPROJECT   "dltdoc" "gen-doc"
       ;;

#debug
        30)
            $HOME/.Terminal/debug.sh $LIBPROJECT"src/"
        ;;

#project
        50)
            $HOME/.Terminal/ProjectNVIM.sh  "all-lib" $ALL_LIB
        ;;

#project
        55)
            $HOME/.Terminal/ProjectNVIM.sh  "gen-lib" $GEN_LIB
        ;;


#project
        60)
            $HOME/.Terminal/ProjectNVIM.sh  $PROJECT $LIBPROJECT
        ;;


#?clear
        77)
        		cargo clean
#            thunar $HOME/.local/state
        ;;

#console

        88)
            $HOME/.Terminal/console.sh $LIBPROJECT
        ;;

        90)
         $LIBPROJECT"gen_doc.sh"
        ;;
        
        
        91)
				 $LIBPROJECT"gen_help.sh"
        ;;
        
        
        92)
				$LIBPROJECT"dlt_doc.sh"
        ;;
         
        
        93)
				$LIBPROJECT"lst_doc.sh"
        ;;               
        
# QUIT
        99)
            break
        ;;

    esac
    fi # fintest option

printf '\e[8;'30';'80't'

done

cleanup
