use gen_doc::terminal::*;
use rusqlite::{Connection, Result};
use std::env;
use std::io::Write;
use std::path::Path;

// recupère le $HOME
fn get_home() -> Option<String> {
	env::var("HOME").ok().or_else(|| env::var("USERPROFILE").ok())
}

fn get_path(lib_bd: &str) -> &'static str {
	let s = format!("{}{}", get_home().unwrap(), lib_bd);
	std::boxed::Box::leak(s.into_boxed_str())
}

fn print_help(conn: &Connection, programme: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();

	let mut stmt = conn.prepare("SELECT ligne, code_attribut, text FROM help WHERE programme = ?1 ORDER BY ligne")?;

	let rows = stmt.query_map([programme], |row| {
		Ok((
			row.get::<_, i32>(0)?,	  // ligne
			row.get::<_, String>(1)?, // code_attribut
			row.get::<_, String>(2)?, // text
		))
	})?;
	for row in rows {
		let (_, code_attribut, text) = row?;
		match code_attribut.as_str() {
			"*" => print!("\x1B[32m\x1B[1m{}\x1B[0m\r\n", text), // \r\n pour forcer le retour à la ligne
			"!" => print!("\x1B[33m{}\x1B[0m\r\n", text),
			"-" => print!("  - {}\r\n", text),
			"." => print!("  {}\r\n", text),
			"?" => print!("\x1B[36m{}\x1B[0m\r\n", text),
			"_" => print!("\x1B[31m\x1B[4m{}\x1B[0m\r\n", text),
			_ => print!("{} {}\r\n", code_attribut, text),
		}
		stdout.flush().unwrap();
	}

	Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Récupération du terminal
	let raw_mode = RawMode::enable().expect("Ce programme nécessite un terminal valide");
	off_mouse();

	// 1. Récupérer le nom du programme depuis les arguments
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		pause(&format!("Usage: {} <nom_du_programme>", args[0]));
		return Ok(());
	}
	let programme = &args[1];

	resize_term(42, 132);

	title_term(&programme); // Titre de la fenêtre

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

	// 5. Vérifier que le programme existe dans la base
	let count: i32 = conn.query_row("SELECT COUNT(*) FROM help WHERE programme = ?1", [programme], |row| row.get(0))?;
	if count == 0 {
		pause(&format!("\x1B[31mAucune aide trouvée pour le programme '{}'.", programme));
		return Ok(());
	}

	// 6. Afficher l'aide
	print_help(&conn, programme)?;

	// Pied de page
	print!("\r\n\x1B[36mAppuyez sur [Esc] pour quitter...\x1B[0m\r\n");

	// boucle de lecture
	get_escape();

	// Restaurer les attributs du terminal
	let _ = raw_mode.disable();
	Ok(())
}
