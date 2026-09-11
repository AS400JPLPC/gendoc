use std::io::Read;
use std::io::Write;

use std::fmt;
use std::io;
use std::os::unix::io::AsRawFd;

pub struct RawMode {
	// On stocke la structure termios native de libc
	pub orig_termios: libc::termios,
}

impl RawMode {
	pub fn enable() -> io::Result<Self> {
		let fd = std::io::stdin().as_raw_fd();

		unsafe {
			let mut orig_termios: libc::termios = std::mem::zeroed();

			// Équivalent de Termios::from_fd()
			if libc::tcgetattr(fd, &mut orig_termios) != 0 {
				return Err(io::Error::last_os_error());
			}

			let mut raw = orig_termios;

			// Application exacte de vos masques avec les constantes de libc
			raw.c_iflag &=
				!(libc::IGNBRK
					| libc::BRKINT | libc::PARMRK
					| libc::INPCK | libc::ISTRIP
					| libc::ICRNL | libc::IGNCR
					| libc::IXON);

			raw.c_oflag &= !libc::OPOST;

			raw.c_cflag |= libc::CS8;
			raw.c_cflag &= !libc::PARENB;

			raw.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::IEXTEN | libc::ISIG);

			// Configuration des caractères de contrôle (VMIN et VTIME)
			raw.c_cc[libc::VMIN] = 1; // Attendre au moins 1 caractère
			raw.c_cc[libc::VTIME] = 0; // Pas de time-out

			// Équivalent de tcsetattr()
			if libc::tcsetattr(fd, libc::TCSANOW, &raw) != 0 {
				return Err(io::Error::last_os_error());
			}

			Ok(RawMode { orig_termios })
		}
	}

	// Désactive manuellement le mode RAW
	pub fn disable(&self) -> io::Result<()> {
		let fd = std::io::stdin().as_raw_fd();
		unsafe {
			if libc::tcsetattr(fd, libc::TCSANOW, &self.orig_termios) != 0 {
				return Err(io::Error::last_os_error());
			}
		}
		Ok(())
	}

	pub fn flush_input() {
		let fd = std::io::stdin().as_raw_fd();
		unsafe {
			libc::tcflush(fd, libc::TCIFLUSH);
		}
	}
}

#[repr(i32)]
#[derive(Debug, Clone, PartialEq)]
pub enum Style {
	NotStyled = 0,
	Bold = 1,
	Dim = 2,
	Italic = 3,
	Underline = 4,
	Blink = 5,
	BlinkRapid = 6,
	Reverse = 7,
	Hidden = 8,
	Crossed = 9,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ZoneAttr {
	pub styles: [i32; 4], // Tableau de 4 styles (comme en Zig)
	pub background: BackgroundColor,
	pub foreground: ForegroundColor,
}

// Attribut par défaut pour le cadre
pub const ATR_TERM: ZoneAttr = ZoneAttr {
	styles: [
		Style::NotStyled as i32, // styleDim
		Style::NotStyled as i32, // notStyle
		Style::NotStyled as i32, // notStyle
		Style::NotStyled as i32, // notStyle
	],
	background: BackgroundColor::Black, // bgBlack
	foreground: ForegroundColor::Black, // fgRed
};

#[repr(i32)]
#[derive(Debug, Clone, PartialEq)]
// les couleurs sont testés avec gnome-terminal / xfce4-terminal
pub enum ForegroundColor {
	Black = 16,   // black
	Red = 1,      // red
	Green = 10,   // green
	Yellow = 11,  // yellow
	Blue = 12,    // blue
	Magenta = 13, // magenta
	Cyan = 14,    // cyan
	White = 15,   // white
	Gray = 8,     // Gray
	Orange = 208, // Orange
}
#[repr(i32)]
#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundColor {
	Black = 16,   // black	pour la compatobilté des terminaux le 0 erase certain terminaux
	Red = 1,      // red	pour la justification entre rose et orange !!!
	Green = 10,   // green
	Yellow = 11,  // yellow
	Blue = 12,    // blue
	Magenta = 13, // magenta
	Cyan = 14,    // cyan
	White = 15,   // white
	Gray = 8,     // Gray
	Orange = 208, // Orenge
}

//======================================
// pause facilite debug primaire
//======================================
//	write-All
pub fn print_all(text: &str) {
	io::stdout().write_all(text.as_bytes()).unwrap();
	std::io::stdout().flush().unwrap(); // Force l'affichage
}
//	write-format
#[allow(clippy::explicit_write)]
pub fn print_fmt(args: fmt::Arguments<'_>) {
	write!(io::stdout(), "{args}").unwrap();
	std::io::stdout().flush().unwrap(); // Force l'affichage
}

// Applique un style au terminal.
// Applique tous les styles d'une `ZoneAttr`.
pub fn set_styles(styles: &[i32; 4]) {
	for &style in styles {
		if style > 0 {
			print_fmt(format_args!("\x1b[{}m", style));
		}
	}
}

// Écrit du texte avec des attributs de style et de couleur.
pub fn print_styled_text(text: &str, attr: ZoneAttr) {
	print_fmt(format_args!("\x1b[38;5;{}m", attr.foreground as i32));
	print_fmt(format_args!("\x1b[48;5;{}m", attr.background as i32));
	set_styles(&attr.styles);
	print_fmt(format_args!("{}\x1b[0m", text));
}
pub fn pause(msg: &str) {
	let attr =
		ZoneAttr { styles: [0, 0, 0, 0], background: BackgroundColor::Black, foreground: ForegroundColor::Yellow };
	print_styled_text(&format!("Pause  {}\r\n", msg), attr);

	let mut stdin = io::stdin();

	// nettoyer le buffer, Lire d'abord les données.
	RawMode::flush_input();
	loop {
		let mut buffer = [0; 1]; // Buffer de taille 1
		if let Ok(c) = stdin.read(&mut buffer) {
			if c == 1 && buffer[0] == 27 {
				break;
			}
		}
	}
}

// Clear all screen
pub fn clear_screen() {
	print!("\x1B[2J\x1B[3J\x1B[H");
}

// if use resize ok : vte application terminal ex TermVte
pub fn resize_term(line: usize, col: usize) {
	if line > 0 && col > 0 {
		print_fmt(format_args!("\x1b[8;{};{}t", line, col));
	}
}
/// Update title terminal
pub fn title_term(title: &str) {
	if !title.is_empty() {
		print_fmt(format_args!("\x1b]0;{}\x07", title));
	}
}

/// offMouse
pub fn off_mouse() {
	print_all("\x1b[?1000;1005;1006l");
}

// boucle en attent d'avoir tout lu
pub fn get_escape() {
	loop {
		RawMode::flush_input();
		let mut buffer = [0; 1]; // Buffer de taille 1
		if let Ok(c) = std::io::stdin().read(&mut buffer) {
			if c == 1 && buffer[0] == 27 {
				break;
			}
		}
	}
}
