extern crate odbc;
extern crate env_logger;
extern crate odbc_safe;

use odbc::*;
use odbc_safe::AutocommitOn;

fn main() {
    env_logger::init();
    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}

fn connect() -> std::result::Result<(), DiagnosticRecord> {
    let env = create_environment_v3().map_err(|e| e.unwrap())?;
    let mybuffer = "Driver={Microsoft Access Driver (*.mdb, *.accdb)}; DBQ=C:/CASA/LINGUAGGI/RUST_PROGETTI/RUST_PURO/RUST_PURO_COLLEGA_ACCESS/archivi_mdb/PRES3000_N25_PIANTA_ORGANICA.mdb;".to_owned();

    let conn = env.connect_with_connection_string(&mybuffer)?;
    execute_statement(&conn)
}

fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;
    let sql_text = "SELECT * FROM DIPENDENTI ORDER BY DIPENDENTI.ID_DIPEN_lng;".to_owned();

    match stmt.exec_direct(&sql_text)? {
        Data(mut stmt) => {
            // Creare una stringa HTML per la tabella
            let mut html_table = String::from("<table border='1'>");
            
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                // Iniziare una nuova riga della tabella
                html_table.push_str("<tr>");
                
                for i in 1..=cols {
                    // Per ogni colonna, creare una cella nella tabella HTML
                    html_table.push_str("<td>");
                    
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => html_table.push_str(val),
                        None => html_table.push_str("NULL"),
                    }
                    
                    html_table.push_str("</td>");
                }
                
                // Chiudere la riga della tabella
                html_table.push_str("</tr>");
            }
            
            // Chiudere la tabella HTML
            html_table.push_str("</table>");
            
            // Stampa la stringa HTML
            println!("{}", html_table);
        }
        NoData(_) => {
            println!("La query e' stata eseguita correttamente, ma non ha restituito dati")
        }
    }

    Ok(())
}


