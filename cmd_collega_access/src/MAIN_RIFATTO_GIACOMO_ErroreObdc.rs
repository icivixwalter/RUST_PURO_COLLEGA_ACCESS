#![allow(unused_must_use, dead_code)]

// TODO: Importante: Per connettersi a un qualsiasi DB Access bisogna PRIMA installare
// la versione opportuna (x64) di AccessDatabaseEngine
// da https://www.microsoft.com/en-us/download/details.aspx?id=54920
// devi scarica questo eseguibile accessdatabaseengine_X64.exe da internet per consetire il funzionamento
// dell'odbc di access


/*      
    UTILITA
        TUTORIAL_GIT
            Il tutorial per git si trova in questa path, per tutti i comandi
            git in ordine alfabetico:

                START "APRI GIT" CALL c:\CASA\LINGUAGGI\GIT_DESKTOP\GIT_DESKTOP.sublime-project
        
        GIT_PUSH
            a volta non funziona il git push automatico, per cui devi provare questo:
                git push https://github.com/icivixwalter/RUST_PURO_COLLEGA_ACCESS


            oppure per salvare i parametri di configurazione

                git push --set-upstream https://github.com/icivixwalter/RUST_PURO_COLLEGA_ACCESS

        

        RUST_TUTORIAL
            il tutorial su rust si trova qui:
        
                https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

            inoltre tutto il tutorial di rust si trova qui:
                START "APRI RUST TUTORIAL" call "c:\CASA\LINGUAGGI\RUST_TUTORIAL\RUST_TUTORIAL.sublime-project"
CMDCMD


            COMANDI_RUST
                cargo build

                    Costruzione per il rilascio
                    Quando il progetto è finalmente pronto per il rilascio, è possibile 
                    compilarlo con le ottimizzazioni. Questo comando creerà un oggetto 
                    eseguibile in target/release invece di target/debug. Le ottimizzazioni 
                    rendere il tuo codice Rust più veloce, ma attivarli allunga il tempo 
                    necessario per la compilazione del programma. Questo è il motivo per 
                    cui ci sono due diversi profili: uno per lo sviluppo, quando si 
                    desidera ricostruire rapidamente e spesso, e un altro per 
                    Costruire il programma finale che darai a un utente che non verrà 
                    ricompilato ripetutamente e che funzionerà il più velocemente possibile. 
                    Se stai effettuando il benchmarking del tuo tempo di esecuzione del codice, 
                    assicurarsi di eseguire ed eseguire il benchmark con L'eseguibile 
                    in Target/Release.cargo build 

                        cargo build --release
*/

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

//funzione connessione + diagnostica
/*fn connect<'env>() -> std::result::Result<&'env Connection<'env, AutocommitOn>, DiagnosticRecord> {
    //assegna a env un ambiente odbc
    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    //Importante: per leggere da excel usare:
    // "Driver={Microsoft Excel Driver (*.xls, *.xlsx, *.xlsm, *.xlsb)};DBQ=c:/my/file.xls"

    /* codice che recupero la stringa della cnn da cli*/
    //let mut mybuffer = String::new();
    // println!("Inserire la stringa di connessione [esempio: Driver={{Microsoft Access Driver (*.mdb, *.accdb)}}; DBQ=c:/my/file.accdb]: ");
    // io::stdin().read_line(&mut buffer).unwrap();
    // //\r\n= unico modo per windows per riconoscere solo l'invio senza stringa  e per default attiv l'avviso
    // if buffer.eq(&String::from("\r\n")) {
    //  mybuffer = "Driver={Microsoft Access Driver (*.mdb, *.accdb)}; DBQ=./PRES3000_N25_PIANTA_ORGANICA.mdb;".to_owned(); // to_owned() converte &str in String
    // }

    /* ALTERNATIVA = uso stringa di connessione costante*/
    let mybuffer = r#"Driver={Microsoft Access Driver (*.mdb, *.accdb)}; DBQ=c:\CASA\LINGUAGGI\RUST_PROGETTI\RUST_PURO\RUST_PURO_COLLEGA_ACCESS\archivi_mdb\PRES3000_N25_PIANTA_ORGANICA.mdb;"#; // to_owned() converte &str in String


    //attivo la conenessione passando il buffer
    return Ok(env.connect_with_connection_string(mybuffer)?);
}*/

//questa funzione esegue una query qualsiasi attenzione viene utilizzato 'env = life time delle fuzione e dell'oggetto
fn execute_statement<'env>(
    query: &str, 
    nome_file: &str, 
    conn: &Connection<'env, AutocommitOn>
    ) -> Result<()> {
    
    // Crea un nuovo Statement (comando SQL vuoto) a partire dalla connessione passata come parametro.
    let stmt = Statement::with_parent(conn)?;

    /* codice che recupera la stringa della query da cli*/
    // let mut sql_text = String::new();
    // println!("Inserire il comando SQL: [default: SELECT * FROM PRES3000_TB25_DIPENDENTI;]");
    // io::stdin().read_line(&mut sql_text).unwrap();
    // // Se non scrivi nulla, cioè premi invio (\r\n), viene eseguita una query di default.
    // if sql_text.eq(&String::from("\r\n")) {
    //     sql_text = "SELECT * FROM PRES3000_TB25_DIPENDENTI;".to_owned(); // to_owned() converte &str in String
    // }

    // assegno alla variabile il valore &str ()= RIFERIMENTO a stringa immutabile
    // let sql_text = "SELECT * FROM PRES3000_TB25_DIPENDENTI;".to_owned(); // to_owned() converte &str in String
    // alternativa: String::from("SELECT * FROM PRES3000_TB25_DIPENDENTI;");

    /*

    QUERY CON ORDINAMENTO: funziona ma non ci devono essere i caratteri
    speciali; ordina per id

    let sql_text = "
    SELECT DIPENDENTI.ID_DIPEN_lng,
    DIPENDENTI.DENOMINAZIONE_s,
    DIPENDENTI.COGNOME_S,
    DIPENDENTI.NOME_S,
    DIPENDENTI.INIZIALI_DIP_S
    FROM DIPENDENTI
    ORDER BY DIPENDENTI.ID_DIPEN_lng
    WITH OWNERACCESS OPTION;".to_owned();


    */

    //let nome_tabella="LLPP_ATTI_Tb01_Gestione";
    


//    let sql_text = format!("SELECT LLPP_ATTI_Tb01_Gestione.IDGestione, LLPP_ATTI_Tb01_Gestione.CodPratica_s, 
//                            LPP_ATTI_Tb01_Gestione.NroPratica_i, LLPP_ATTI_Tb01_Gestione.AnnoCodPratica_i, 
//                            LPP_ATTI_Tb01_Gestione.OggettoAtto_m, LLPP_ATTI_Tb01_Gestione.CodOpera_s, 
//                            LPP_ATTI_Tb01_Gestione.DescrCodOpera_s
//                            ROM {nome_tabella}
//                            HERE (((LLPP_ATTI_Tb01_Gestione.IDGestione)=20809 
//                            r (LLPP_ATTI_Tb01_Gestione.IDGestione)=20818 Or
//                            (LLPP_ATTI_Tb01_Gestione.IDGestione)=20649 Or
//                            (LLPP_ATTI_Tb01_Gestione.IDGestione)=20834 Or 
//                            (LLPP_ATTI_Tb01_Gestione.IDGestione)=19748));");
//
//
//

     // Apre un file in modalità di scrittura, creandolo se non esiste
    let mut file = File::create(format!("{nome_file}.csv")).unwrap();

    



    //QUERY FISSA = attenzione con valori null nei campi da errore
    // eseguo la query scritta da linea di comando con exec_direct()
    // costrutto match con due rami: Data(statement) e NoData(_)
    //attenzione il ? = significa che in caso di errore ritorna alla funzione chiamante l'errore.
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


/// 01
/// tabella DIPENDENTI  TUTTI I CAMPI  ---> FUNZIONA
/// todo: query su tabella ma non funzione per i caratteri accententati tipo Scirè Calabrisotto Andrea oppure De Podestà Emanuela
fn query_tutti_campi() -> (String, String) {
    return ("SELECT D.* FROM DIPENDENTI D;".to_string(), "File_Dipendenti_tutto".to_string());


}



/// 02
/// tabella DIPENDENTI  CON CAMPO MEMO ---> FUNZIONA ; 
/// .to_owned() = da oggetto prestato trasformato in oggetto di proprieta
fn query_campo_memo() -> (String, String) {
    return
    (
        "SELECT D.ID, D.COGNOME_S, D.NOME_S, D.INIZIALI_DIP_S, D.MEMO_m 
         FROM DIPENDENTI D
         WHERE (((D.ID)<7));".to_owned(),
        "File_Dipendenti_campo_memo".to_owned()
    );
}


/// 03
fn query_senza_campo_memo() -> (String, String) {
    return 
    (
        "SELECT  D.ID_DIPEN_lng, D.DENOMINAZIONE_s, D.COGNOME_S, D.NOME_S, D.INIZIALI_DIP_S
         FROM DIPENDENTI D
         ORDER BY D.ID_DIPEN_lng
         WITH OWNERACCESS OPTION;".to_owned(),
        "File_Dipendenti_senza_campo_memo".to_owned()
    );
}


