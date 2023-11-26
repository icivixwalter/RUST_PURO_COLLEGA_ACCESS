#![allow(unused_must_use, dead_code)]




extern crate odbc;
// Use this crate and set environmet variable RUST_LOG=odbc to see ODBC warnings
extern crate env_logger;
extern crate odbc_safe;

//per scrivere sul file
use std::fs::File;
use std::io::Write;

use std::io;
use odbc::*;

//#[allow(unused_imports)]        //permetto l'uso di use std altrimenti da errore
use odbc_safe::AutocommitOn;
// use std::x::conessione;

// const DB: &str = "./EsempioStudenti.accdb";

fn main() {
    //inizializzo la connessione
    env_logger::init();


    let env = create_environment_v3().map_err(|e| e.expect("Impossibile usare env")).expect("errore creazione environment");

    /* ALTERNATIVA = uso stringa di connessione costante*/
    let mybuffer = r#"Driver={Microsoft Access Driver (*.mdb, *.accdb)}; DBQ=.\PRES3000_N25_PIANTA_ORGANICA.mdb;"#; // to_owned() converte &str in String
    // let mybuffer = r#"{Microsoft Access Driver (*.mdb, *.accdb)};        DBQ=c:\CASA\LINGUAGGI\RUST_PROGETTI\RUST_PURO\RUST_PURO_COLLEGA_ACCESS\archivi_mdb\PRES3000_N25_PIANTA_ORGANICA.mdb;"#;

    //attivo la conenessione passando il buffer
    let Ok(conn) = env.connect_with_connection_string(mybuffer) else {
        panic!("errore di connessione");
    };

    //recupero la stringa query 
    //01
    let (my_query, my_file_name) = query_tutti_campi();
    
    //02
    //let my_query: String = query_campo_memo();
    
    //03
    // let my_query: String = query_senza_campo_memo()

    //se ho la connessione esegui la query
    execute_statement(&my_query, &my_file_name,&conn);

    //prende l'input con lo scopo di ritardare la chiusura della shell
    println!("Premere invio per terminare.");
    io::stdin().read_line(&mut String::new()).unwrap();
}


//questa funzione esegue una query qualsiasi attenzione viene utilizzato 'env = life time delle fuzione e dell'oggetto
fn execute_statement<'env>(
    query: &str, 
    nome_file: &str, 
    conn: &Connection<'env, AutocommitOn>) -> Result<()> {
    
        // Crea un nuovo Statement (comando SQL vuoto) a partire dalla connessione passata come parametro.
        let stmt = Statement::with_parent(conn)?;

      
         // Apre un file in modalità di scrittura, creandolo se non esiste
        let mut file = File::create(format!("{nome_file}.csv")).unwrap();

        



        /* QUERY PARAMETRICA FISSA =VIENE RECUPERATA DAL PARAMETRO query: attenzione con valori 
            null nei campi da errore eseguo la query scritta da linea di comando con exec_direct()
                costrutto match con due rami: Data(statement) e NoData(_)
                attenzione il ? = significa che in caso di errore ritorna alla funzione chiamante l'errore.
            */
                match stmt.exec_direct(query)? {
                    // Se ci sono dati, li stampo in output
                    Data(mut stmt) => {
                        // Stampo ogni colonna separando con uno spazio
                        let cols = stmt.num_result_cols()?; // numero di colonne recuperato con il ciclo while
                                                            // finche' c'e' qualche (Some) riga (la riga si prende con statement.fetch())
                        while let Some(mut cursor) = stmt.fetch()? {
                            // per ogni colonna (le colonne partono da 1 per ODBC)
                            for i in 1..=cols {
                                // intervallo di numeri 1,2,3,...,numero_colonne. l'= serve per includere l'estremo superiore dell'intervallo

                                match cursor.get_data::<&str>(i as u16)? {
                                    // se ci sono dati nella cella, stampo con uno spazio davanti per separare.
                                    Some(val) => {
                                        // Scrive dei dati nel file
                                        file.write_all(format!("\"{val}\";").as_bytes()).expect("errore impossibile scrivere sul file");
                                        print!("\"{val}\";");
                                    }
                                    // se non ci sono dati nella cella, stampo la parola NULL (valore nullo)
                                    None => {
                                        // Scrive dei dati nel file
                                        file.write_all(b"NULL;").expect("errore impossibile scrivere sul file");
                                        print!("NULL;");
                                    }
                                }
                            }
                            // vado a capo
                            file.write_all(b"\n").expect("errore impossibile scrivere sul file");
                            println!("");
                        }
                    }
                    // Se la query non restituisce dati, stampo una stringa.
                    NoData(_) => {
                        println!("La query e' stata eseguita correttamente, ma non ha restituito dati")
                    }
                }

    // Restituisco Ok al chiamante. Se si arriva qui, è andato tutto bene (Ok!)
    Ok(())
}


// LE FUNZIONI DELLE QUERY
//-----------------------------------------------------------------------------------------------------//

    /// 01
    /// tabella DIPENDENTI  TUTTI I CAMPI  ---> FUNZIONA
    /// query su tabella e funziona con gli apostrofi ma bisogna vedere se funziona per i caratteri accententati tipo Scirè Calabrisottò Andrea oppure De Podestà Emanuela
    /// TODO: finire di controllare le vocali con accenti PERCHE' ORA DANNO ERROE: à,ò,ù,è,é
    fn query_tutti_campi() -> (String, String) {
        // ORDER BY D.ID_DIPEN_lng attenzione se manca il field viene dato un errore di connessione odb, ma in realta il campo non esiste.
        return ("SELECT D.* FROM DIPENDENTI D ORDER BY D.ID;".to_string(), "File_Dipendenti_tutto".to_string()); }



    /// 02
    /// TABELL DIPENDENTI  CON CAMPO MEMO ---> FUNZIONA ; 
    /// .to_owned() = da oggetto prestato trasformato in oggetto di proprieta
    fn query_campo_memo() -> (String, String) {
        return
        (
            "SELECT D.ID, D.COGNOME_S, D.NOME_S, D.INIZIALI_DIP_S, D.MEMO_m 
             FROM DIPENDENTI D
             WHERE (((D.ID)<7));".to_owned(),
            "File_Dipendenti_campo_memo".to_owned()); }


    /// 03
    fn query_senza_campo_memo() -> (String, String) {
        return 
        (
            "SELECT  D.ID_DIPEN_lng, D.DENOMINAZIONE_s, D.COGNOME_S, D.NOME_S, D.INIZIALI_DIP_S
             FROM DIPENDENTI D
             ORDER BY D.ID_DIPEN_lng
             WITH OWNERACCESS OPTION;".to_owned(),
            "File_Dipendenti_senza_campo_memo".to_owned()
        );}

// *** fine *** ///
//-----------------------------------------------------------------------------------------------------//
