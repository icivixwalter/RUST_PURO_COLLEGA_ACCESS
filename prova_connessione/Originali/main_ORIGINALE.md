//connessione funzionante


extern crate odbc;
use odbc::*;

fn main() {
    match connect() {
        Ok(_) => println!("connessione attivata con Successo"),
        Err(diag) => println!("Error: {}", diag),
    }
}


fn connect() -> std::result::Result<(), DiagnosticRecord> {
    let env = Environment::new().map_err(|e| e.unwrap())?;

    // Specifica il percorso del file DSN
    let dsn_path = "c:\\CASA\\LINGUAGGI\\RUST_PROGETTI\\RUST_PURO\\RUST_PURO_COLLEGA_ACCESS\\archivi_mdb\\ODBC_25_PIANTA_ORGANICA.dsn";

    // Costruisci la stringa di connessione utilizzando il DSN
    let conn_str = format!("FILEDSN={}", dsn_path);

    // Connettiti al database utilizzando il DSN
    let _conn = env.connect_with_connection_string(&conn_str)?;

    Ok(())
}
