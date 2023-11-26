Note.md


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




//  @note_(@funzione @di@connessione + @diagnostica)
/*
  fn connect<'env>() -> std::result::Result<&'env Connection<'env, AutocommitOn>, DiagnosticRecord> {
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




// TODO: Importante: Per connettersi a un qualsiasi DB Access bisogna PRIMA installare
// la versione opportuna (x64) di AccessDatabaseEngine
// da https://www.microsoft.com/en-us/download/details.aspx?id=54920
// devi scarica questo eseguibile accessdatabaseengine_X64.exe da internet per consetire il funzionamento
// dell'odbc di access


/*      
    UTILITA
        @TUTORIAL_@GIT
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
    ERRORI_RUST
        ERRORE NELLA QUERY - ERRORE ODBC
            Quando compare questo campo manca la query costruita è sbagliata, di solito nella query precostituita
            viene indicato un campo inesistene.
                    
                    @ERRORE@ODBC_(errore di @connessione perche la query STATA costruita CON un @campo @inesistente)

            L'errore consiste nel fatto che odbc si aspetta un parametro e la quary si basa su un campo che non esiste, occorre correggerlo.

                [2023-11-26T16:06:38Z ERROR odbc::result] State: 07002, Native error: -3010, Message: [Microsoft][ODBC Microsoft Access Driver] Too few parameters. Expected 1.             

Premere invio per terminare.  
*/


