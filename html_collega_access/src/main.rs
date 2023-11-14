extern crate odbc;
extern crate env_logger;
extern crate odbc_safe;
extern crate actix_web;

use actix_web::{web, App, HttpServer, HttpResponse};
use odbc::*;
use odbc_safe::AutocommitOn;

fn main() {
    env_logger::init();
    HttpServer::new(|| {
        App::new().route("/", web::get().to(handle_request))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}

async fn handle_request() -> HttpResponse {
    match connect() {
        Ok(html_table) => HttpResponse::Ok().body(html_table),
        Err(diag) => HttpResponse::InternalServerError().body(format!("Error: {}", diag)),
    }
}

fn connect() -> std::result::Result<String, DiagnosticRecord> {
    let env = create_environment_v3().map_err(|e| e.unwrap())?;
    let mybuffer = r#"Driver={Microsoft Access Driver (*.mdb, *.accdb)}; DBQ=c:\CASA\LINGUAGGI\RUST_PROGETTI\RUST_PURO\RUST_PURO_COLLEGA_ACCESS\archivi_mdb\PRES3000_N25_PIANTA_ORGANICA.mdb;"#.to_owned(); // to_owned() converte &str in String
    let conn = env.connect_with_connection_string(&mybuffer)?;
    execute_statement(&conn)
}

fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<String> {
    let stmt = Statement::with_parent(conn)?;
    let sql_text = "SELECT * FROM DIPENDENTI ORDER BY DIPENDENTI.ID_DIPEN_lng;".to_owned();

    match stmt.exec_direct(&sql_text)? {
        Data(mut stmt) => {
            let mut html_table = String::from("<table border='1'>");
            let cols = stmt.num_result_cols()?;
            
            while let Some(mut cursor) = stmt.fetch()? {
                html_table.push_str("<tr>");
                
                for i in 1..=cols {
                    html_table.push_str("<td>");
                    
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => html_table.push_str(val),
                        None => html_table.push_str("NULL"),
                    }
                    
                    html_table.push_str("</td>");
                }
                
                html_table.push_str("</tr>");
            }
            
            html_table.push_str("</table>");
            
            Ok(html_table)
        }
        NoData(_) => Ok("La query e' stata eseguita correttamente, ma non ha restituito dati".to_owned()),
    }
}
