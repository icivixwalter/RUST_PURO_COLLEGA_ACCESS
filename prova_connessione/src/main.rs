//extern crate odbc;

use odbc::*;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    // Definisci la rotta principale
    let route = warp::path!("data" / "table")
        .and_then(get_data)
        .recover(handle_rejection);

    // Avvia il server
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Funzione per gestire la richiesta di dati
async fn get_data() -> Result<impl Reply, Rejection> {
    // Connessione al database usando il file DSN
    let env = Environment::new().map_err(|e| warp::reject::custom(e.unwrap()))?;
    let dsn_path =r#"c:\CASA\LINGUAGGI\RUST_PROGETTI\RUST_PURO\RUST_PURO_COLLEGA_ACCESS\archivi_mdb\ODBC_25_PIANTA_ORGANICA.dsn""#.to_owned();

    
    let conn_str = format!("FILEDSN={}", dsn_path);
    let conn = env.connect_with_connection_string(&conn_str).map_err(|e| warp::reject::custom(e))?;

    // Query al database con la tua query specifica
    let sql_text = "SELECT * FROM DIPENDENTI ORDER BY DIPENDENTI.ID_DIPEN_lng;".to_owned();
    let result = conn.query(&sql_text).map_err(|e| warp::reject::custom(e))?;

    // Costruisci una tabella HTML con i risultati della query
    let html_table = build_html_table(result);

    // Ritorna la risposta HTML
    Ok(warp::reply::html(html_table))
}

// Funzione per costruire una tabella HTML da un risultato di query
fn build_html_table(result: Statement) -> String {
    let mut html_table = String::from("<table><tr>");

    // Estrai i nomi delle colonne
    let columns = result.columns().map(|col| col.name().to_owned()).collect::<Vec<_>>();
    for col in &columns {
        html_table.push_str(&format!("<th><a href='#' onclick='sortTable({})'>{}</a></th>", col, col));
    }
    html_table.push_str("</tr>");

    // Estrai i dati delle righe
    while let Some(row) = result.fetch().unwrap() {
        html_table.push_str("<tr>");
        for col in &columns {
            if let Some(value) = row.get(col).unwrap() {
                html_table.push_str(&format!("<td>{}</td>", value));
            } else {
                html_table.push_str("<td>null</td>");
            }
        }
        html_table.push_str("</tr>");
    }

    html_table.push_str("</table>");

    // Aggiungi script JavaScript per l'ordinamento della tabella
    html_table.push_str("<script>
        function sortTable(column) {
            const table = document.querySelector('table');
            const tbody = table.querySelector('tbody');
            const rows = Array.from(tbody.querySelectorAll('tr'));

            const columnIndex = columns.indexOf(column);
            rows.sort((a, b) => {
                const aValue = a.cells[columnIndex].innerText.trim();
                const bValue = b.cells[columnIndex].innerText.trim();
                return isNaN(aValue) ? aValue.localeCompare(bValue) : aValue - bValue;
            });

            while (tbody.firstChild) {
                tbody.removeChild(tbody.firstChild);
            }

            rows.forEach(row => tbody.appendChild(row));
        }
    </script>");

    html_table
}

// Gestisci eventuali errori odbc come reiezioni warp
fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Rejection> {
    if let Some(e) = err.find::<DiagnosticRecord>() {
        // Converti errori odbc in risposta HTML
        let error_html = format!("Error: {}", e);
        Ok(warp::reply::html(error_html))
    } else {
        // Altri tipi di reiezione vengono inoltrati senza alcuna modifica
        Err(err)
    }
}
