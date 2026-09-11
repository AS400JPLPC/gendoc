use rusqlite::{Connection, Result};

use gen_doc::terminal::*;
use std::env;
use std::path::Path;

pub fn is_dir(dir: &str) -> bool {
	std::path::Path::new(dir).is_dir()
}

// recupère le $HOME
fn get_home() -> Option<String> {
	env::var("HOME").ok().or_else(|| env::var("USERPROFILE").ok())
}

fn get_path(lib_bd: &str) -> &'static str {
	let s = format!("{}{}", get_home().unwrap(), lib_bd);
	std::boxed::Box::leak(s.into_boxed_str())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Récupération du terminal
	let raw_mode = RawMode::enable().expect("Ce programme nécessite un terminal valide");

	// 1. Récupérer le nom du programme depuis les arguments
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		pause(&format!("Usage: {} <nom_du_programme>", args[0]));
		return Ok(());
	}
	let programme = &args[1];

	// 2. Chemin absolu vers la base SQLite
	let db_path = get_path("/Zrust/gen_doc/sqlite/help_pgm.db");

	// 3. Vérifier si le fichier existe
	if !Path::new(db_path).exists() {
		pause(&format!("\x1B[31mErreur : Le fichier de la base SQLite {} n'existe pas.\x1B[0m\r\n", db_path));
		return Ok(());
	}
	// 4. ouvrir la connexion SQLite
	let conn = match Connection::open(db_path) {
		Ok(conn) => conn,
		Err(e) => {
			pause(&format!("\x1B[31mErreur : Impossible d'ouvrir la base SQLite : {}\x1B[0m\r\n", e));
			return Ok(());
		}
	};

	// 5. Supprimer la table `help` si elle existe, puis la recréer
	// conn.execute("DROP TABLE IF EXISTS help", [])?;
	let del_pgm = format!("DELETE FROM help WHERE programme = '{}' ", programme);
	conn.execute(&del_pgm, []).unwrap();

	// Restaurer les attributs du terminal
	let _ = raw_mode.disable();
	Ok(())
}
