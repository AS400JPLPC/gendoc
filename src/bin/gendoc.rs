use rusqlite::{Connection, Result};
use std::fs;

use std::fs::File;
use std::path::Path;

use gen_doc::terminal::*;
use std::io::Write;

use libc::EXIT_SUCCESS;

pub fn is_dir(dir: &str) -> bool {
	std::path::Path::new(dir).is_dir()
}

fn print_help(conn: &Connection, programme: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();

	let mut stmt = conn.prepare("SELECT ligne, code_attribut, text FROM help WHERE programme = ?1 ORDER BY ligne")?;
	let rows = stmt.query_map([programme], |row| {
		Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
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
	let chemin_fichier = "help_pgm.txt";

	// 1. CONTRÔLE D'EXISTENCE
	if !Path::new(chemin_fichier).exists() {
		// Le fichier n'existe pas : vous pouvez afficher un message propre et quitter la fonction
		print!("\x1B[31mErreur : Le fichier '{}' est introuvable.\x1B[0m\r\n", chemin_fichier);
		std::io::stdout().flush().unwrap();
		get_escape();
		return Ok(());
	}
	// Récupération du terminal
	let raw_mode = RawMode::enable().expect("Ce programme nécessite un terminal valide");
	off_mouse();

	let file = File::open(chemin_fichier)?;
	file.lock()?;
	file.unlock()?;
	// let reader = io::BufReader::new(file);
	// let mut lines = reader.lines().peekable();
	let contenu_fichier: String = fs::read_to_string("help_pgm.txt")?;
	let mut lines = contenu_fichier.lines().peekable();

	// 2. Extraire le nom du programme depuis la première ligne (Plus de double ??)
	let first_line = lines.next().ok_or("Fichier vide")?;
	let programme = first_line
		.trim_start_matches('*') // Supprimer le '*' initial
		.trim()
		.to_string();

	// 3. Traiter le reste des lignes (for row in lines)
	let mut stdout = std::io::stdout();
	for text in lines {
		// Si vos lignes contiennent un code d'attribut au début (ex: "* Texte" ou "! Texte")
		// On sépare le premier caractère du reste du texte
		if let Some(first_char) = text.chars().next() {
			let code_attribut = first_char.to_string();
			let reste_texte = &text[first_char.len_utf8()..].trim();

			match code_attribut.as_str() {
				"*" => print!("\x1B[32m\x1B[1m{}\x1B[0m\r\n", reste_texte),
				"!" => print!("\x1B[33m{}\x1B[0m\r\n", reste_texte),
				"-" => print!("  - {}\r\n", reste_texte),
				"." => print!("  {}\r\n", reste_texte),
				"?" => print!("\x1B[36m{}\x1B[0m\r\n", reste_texte),
				"_" => print!("\x1B[31m\x1B[4m{}\x1B[0m\r\n", reste_texte),
				_ => print!("{} {}\r\n", code_attribut, reste_texte),
			}
			let _ = stdout.flush();
		}
	} // Pied de page
	print!("\r\n\x1B[36mAppuyez sur [Esc] pour quitter...\x1B[0m\r\n");

	// boucle de lecture
	get_escape();
	clear_screen();
	// Configuration de la base de données
	let dir = "./sqlite";
	let rep = "./sqlite/help_pgm.db";

	// Vérifie que le répertoire et la base existent
	if !is_dir(dir) {
		std::fs::create_dir_all(dir).expect("Impossible de créer le répertoire");
	}

	// 3. Créer une base SQLite (fichier help.db)
	let conn = Connection::open(rep)?;
	conn.execute(
		"CREATE TABLE IF NOT EXISTS  help (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			programme TEXT NOT NULL,
			ligne INTEGER NOT NULL,
			code_attribut TEXT,
			text TEXT NOT NULL
		)",
		[],
	)?;

	// 4. Supprimer la table `help` si elle existe, puis la recréer
	// conn.execute("DROP TABLE IF EXISTS help", [])?;
	let del_pgm = format!("DELETE FROM help WHERE programme = '{}' ", programme);
	conn.execute(&del_pgm, []).unwrap();

	// 5. Lire le fichier help01.txt
	lines = contenu_fichier.lines().peekable();

	// 6. Initialiser un compteur pour les lignes
	let mut ligne_counter: i32 = 1;

	// 7. Parser chaque ligne (en commençant par la 2ème) et insérer dans SQLite
	for line in lines {
		// Extraire le code_attribut (premier caractère)
		let code_attribut = line.chars().next().unwrap_or(' ').to_string();
		// Le reste est le texte
		let text = line[1..].trim().to_string();
		// Insérer dans la base SQLite avec le numéro de ligne auto-incrémenté
		conn.execute(
			"INSERT INTO help (programme, ligne, code_attribut, text) VALUES (?1, ?2, ?3, ?4)",
			(&programme, ligne_counter, &code_attribut, &text),
		)?;

		// Incrémenter le compteur de ligne
		ligne_counter += 1;
	}

	// 8. Vérifier le contenu de la table
	let count: i32 =
		conn.query_row("SELECT COUNT(*) FROM help WHERE programme = ?1", [programme.clone()], |row| row.get(0))?;
	if count == 0 {
		pause(&format!("\x1B[31mAucune aide trouvée pour le programme '{}'.", programme));
		return Ok(());
	}

	clear_screen();
	// entete de page
	print!("\r\n\x1B[36mControler...\x1B[0m\r\n");

	// 9. Afficher l'aide
	print_help(&conn, &programme.to_string())?;

	// Pied de page
	print!("\r\n\x1B[36mAppuyez sur [Esc] pour quitter...\x1B[0m\r\n");
	// boucle de lecture
	get_escape();
	clear_screen();
	// Restaurer les attributs du terminal
	let _ = raw_mode.disable();
	std::process::exit(EXIT_SUCCESS);
}
