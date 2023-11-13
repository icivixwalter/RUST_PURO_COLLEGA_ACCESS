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
use odbc::*;

//#[allow(unused_imports)]        //permetto l'uso di use std altrimenti da errore
use odbc_safe::AutocommitOn;
// use std::x::conessione;

// const DB: &str = "./EsempioStudenti.accdb";

fn main() {
    //inizializzo la connessione
    env_logger::init();
    //select conessione ok or error
    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}

//funzione connessione + diagnostica
fn connect() -> std::result::Result<(), DiagnosticRecord> {
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
    let mybuffer = "Driver={Microsoft Access Driver (*.mdb, *.accdb)}; DBQ=./PRES3000_N25_PIANTA_ORGANICA.mdb;".to_owned(); // to_owned() converte &str in String

    //attivo la conenessione passando il buffer
    let conn = env.connect_with_connection_string(&mybuffer)?;
    execute_statement(&conn)
}

//questa funzione esegue una query qualsiasi attenzione viene utilizzato 'env = life time delle fuzione e dell'oggetto
fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<()> {
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

    //todo: queyr su tabella ma non funzione per i caratteri accententati tipo Scirè Calabrisotto Andrea oppure De Podestà Emanuela
    let sql_text = "SELECT * FROM DIPENDENTI ORDER BY DIPENDENTI.ID_DIPEN_lng;".to_owned();

    // let sql_text = "
    // SELECT DIPENDENTI.ID_DIPEN_lng,
    // DIPENDENTI.DENOMINAZIONE_s,
    // DIPENDENTI.COGNOME_S,
    // DIPENDENTI.NOME_S,
    // DIPENDENTI.INIZIALI_DIP_S
    // FROM DIPENDENTI
    // ORDER BY DIPENDENTI.ID_DIPEN_lng
    // WITH OWNERACCESS OPTION;".to_owned();

    //QUERY FISSA = attenzione con valori null nei campi da errore
    // eseguo la query scritta da linea di comando con exec_direct()
    // costrutto match con due rami: Data(statement) e NoData(_)
    //attenzione il ? = significa che in caso di errore ritorna alla funzione chiamante l'errore.
    match stmt.exec_direct(&sql_text)? {
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
                        Some(val) => print!(" {}", val),
                        // se non ci sono dati nella cella, stampo la parola NULL (valore nullo)
                        None => print!(" NULL"),
                    }
                }
                // vado a capo
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
