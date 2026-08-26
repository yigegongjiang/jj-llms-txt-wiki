> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sicurezza

> Scopri le misure di sicurezza di Claude Code e le migliori pratiche per un utilizzo sicuro.

<h2 id="how-we-approach-security">
  Come affrontiamo la sicurezza
</h2>

<h3 id="security-foundation">
  Fondamento della sicurezza
</h3>

La sicurezza del vostro codice è fondamentale. Claude Code è costruito con la sicurezza al centro, sviluppato secondo il programma di sicurezza completo di Anthropic. Scopri di più e accedi alle risorse (rapporto SOC 2 Type 2, certificato ISO 27001, ecc.) presso [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="permission-based-architecture">
  Architettura basata su permessi
</h3>

Claude Code utilizza permessi di sola lettura rigorosi per impostazione predefinita. Quando sono necessarie azioni aggiuntive (modifica di file, esecuzione di test, esecuzione di comandi), Claude Code richiede un'autorizzazione esplicita. Gli utenti controllano se approvare le azioni una sola volta o consentirle automaticamente.

Claude Code richiede l'approvazione prima di eseguire comandi Bash che possono modificare il vostro sistema. Un insieme integrato di [comandi di sola lettura](/docs/it/permissions#read-only-commands) come `ls`, `cat` e `git status` viene eseguito senza un prompt. Questo approccio consente agli utenti e alle organizzazioni di configurare i permessi direttamente.

Per la configurazione dettagliata dei permessi, vedere [Permissions](/docs/it/permissions).

<h3 id="built-in-protections">
  Protezioni integrate
</h3>

Per mitigare i rischi nei sistemi agentici:

* **Strumento bash in sandbox**: [Sandbox](/docs/it/sandboxing) comandi bash con isolamento del filesystem e della rete, riducendo i prompt di permesso mantenendo la sicurezza. Abilita con `/sandbox` per definire i confini dove Claude Code può lavorare autonomamente
* **Restrizione della directory di lavoro**: Claude Code può scrivere solo nella cartella in cui è stato avviato e nelle sue sottocartelle, e non può modificare file nelle directory padre senza autorizzazione esplicita. La lettura di percorsi al di fuori di questo confine con gli strumenti Read, Grep e Glob è possibile dopo un prompt di approvazione. Estendi il confine con [directory aggiuntive](/docs/it/permissions#working-directories) per saltare il prompt, o limita l'accesso in lettura più ampio disponibile ai comandi Bash di sola lettura con [regole sandbox `denyRead`](/docs/it/sandboxing#filesystem-isolation), che si applicano solo quando il sandboxing è abilitato
* **Mitigazione dell'affaticamento da prompt**: Supporto per l'allowlisting di comandi sicuri utilizzati frequentemente per utente, per codebase o per organizzazione
* **Modalità Accept Edits**: Approva automaticamente le modifiche ai file e un insieme fisso di comandi Bash del filesystem come `mkdir`, `touch`, `rm`, `mv`, `cp` e `sed` per i percorsi nella directory di lavoro. Gli altri comandi Bash e i percorsi fuori ambito richiedono comunque l'approvazione

<h3 id="user-responsibility">
  Responsabilità dell'utente
</h3>

Claude Code ha solo i permessi che gli concedete. Siete responsabili della revisione del codice e dei comandi proposti per la sicurezza prima dell'approvazione.

<h2 id="protect-against-prompt-injection">
  Proteggiti dall'iniezione di prompt
</h2>

L'iniezione di prompt è una tecnica in cui un attaccante tenta di ignorare o manipolare le istruzioni di un assistente AI inserendo testo dannoso. Claude Code include diversi meccanismi di protezione contro questi attacchi:

<h3 id="core-protections">
  Protezioni fondamentali
</h3>

* **Sistema di permessi**: Le operazioni sensibili richiedono un'approvazione esplicita
* **Analisi consapevole del contesto**: Rileva istruzioni potenzialmente dannose analizzando la richiesta completa
* **Sanitizzazione dell'input**: Previene l'iniezione di comandi elaborando gli input dell'utente
* **Approvazione dei comandi di rete**: I comandi che recuperano contenuti dal web come `curl` e `wget` non sono approvati automaticamente per impostazione predefinita. Richiedono un'approvazione come qualsiasi altro comando Bash non di sola lettura, quindi è possibile comunque approvarli una volta o aggiungere una regola di autorizzazione esplicita come `Bash(curl *)`. Per bloccarli completamente, aggiungeteli a [`permissions.deny`](/docs/it/permissions#tool-specific-permission-rules)

<h3 id="privacy-safeguards">
  Misure di protezione della privacy
</h3>

Abbiamo implementato diversi meccanismi di protezione per proteggere i vostri dati, tra cui:

* Periodi di conservazione limitati per le informazioni sensibili (consultare il [Privacy Center](https://privacy.anthropic.com/en/articles/10023548-how-long-do-you-store-my-data) per ulteriori informazioni)
* Accesso limitato ai dati della sessione utente
* Controllo dell'utente sulle preferenze di addestramento dei dati. Gli utenti consumer possono modificare le loro [impostazioni di privacy](https://claude.ai/settings/privacy) in qualsiasi momento.

Per i dettagli completi, consultare i nostri [Termini di servizio commerciali](https://www.anthropic.com/legal/commercial-terms) (per utenti Team, Enterprise e API) o [Termini consumer](https://www.anthropic.com/legal/consumer-terms) (per utenti Free, Pro e Max) e [Informativa sulla privacy](https://www.anthropic.com/legal/privacy).

<h3 id="additional-safeguards">
  Misure di protezione aggiuntive
</h3>

* **Approvazione della richiesta di rete**: Gli strumenti che effettuano richieste di rete richiedono l'approvazione dell'utente per impostazione predefinita
* **Finestre di contesto isolate**: Web fetch utilizza una finestra di contesto separata per evitare di iniettare prompt potenzialmente dannosi
* **Verifica della fiducia**: Le prime esecuzioni di codebase e i nuovi server MCP richiedono la verifica della fiducia
  * Nota: La verifica della fiducia è disabilitata quando si esegue in modo non interattivo con il flag `-p`
  * Nota: Quando avvii Claude Code direttamente nella tua directory home, l'accettazione della fiducia viene mantenuta solo per la sessione corrente e non viene scritta su disco, quindi il prompt riappare ad ogni avvio. Non esiste un'impostazione per renderla persistente. Avvia Claude Code da una sottodirectory del progetto, dove l'accettazione della fiducia viene salvata per directory
* **Rilevamento dell'iniezione di comandi**: I comandi bash sospetti richiedono l'approvazione manuale anche se precedentemente allowlisted
* **Corrispondenza fail-closed**: I comandi non corrispondenti richiedono per impostazione predefinita l'approvazione manuale
* **Descrizioni in linguaggio naturale**: I comandi bash complessi includono spiegazioni per la comprensione dell'utente
* **Archiviazione sicura delle credenziali**: Le chiavi API e i token sono archiviati nel Keychain di macOS quando disponibile, e protetti dalle autorizzazioni dei file su Windows e Linux. Vedere [Credential Management](/docs/it/authentication#credential-management)

<Warning>
  **Rischio di sicurezza WebDAV su Windows**: Quando si esegue Claude Code su Windows, consigliamo di non abilitare WebDAV o di non consentire a Claude Code di accedere a percorsi come `\\*` che potrebbero contenere sottodirectory WebDAV. [WebDAV è stato deprecato da Microsoft](https://learn.microsoft.com/en-us/windows/whats-new/deprecated-features#:~:text=The%20Webclient%20\(WebDAV\)%20service%20is%20deprecated) a causa di rischi di sicurezza. L'abilitazione di WebDAV potrebbe consentire a Claude Code di attivare richieste di rete a host remoti, aggirando il sistema di permessi.
</Warning>

**Migliori pratiche per lavorare con contenuti non attendibili**:

1. Rivedere i comandi suggeriti prima dell'approvazione
2. Evitare di inviare contenuti non attendibili direttamente a Claude tramite pipe
3. Verificare le modifiche proposte ai file critici
4. Utilizzare macchine virtuali (VM) per eseguire script e effettuare chiamate di strumenti, soprattutto quando si interagisce con servizi web esterni
5. Segnalare comportamenti sospetti con `/feedback`

<Warning>
  Sebbene queste protezioni riducano significativamente il rischio, nessun sistema è completamente
  immune da tutti gli attacchi. Mantenete sempre buone pratiche di sicurezza quando lavorate
  con qualsiasi strumento AI.
</Warning>

<h2 id="mcp-security">
  Sicurezza MCP
</h2>

Claude Code consente agli utenti di configurare server Model Context Protocol (MCP). L'elenco dei server MCP consentiti è configurato nel vostro codice sorgente, come parte delle impostazioni di Claude Code che gli ingegneri controllano nel controllo del codice sorgente.

Incoraggiamo sia la scrittura dei vostri server MCP che l'utilizzo di server MCP da provider di cui vi fidate. Siete in grado di configurare i permessi di Claude Code per i server MCP. Anthropic esamina i connettori rispetto ai suoi [criteri di valutazione](https://claude.com/docs/connectors/building/review-criteria) prima di aggiungerli alla [Directory Anthropic](https://claude.ai/directory), ma non esegue audit di sicurezza né gestisce alcun server MCP.

<h2 id="ide-security">
  Sicurezza dell'IDE
</h2>

Vedere [VS Code security and privacy](/docs/it/vs-code#security-and-privacy) per ulteriori informazioni sull'esecuzione di Claude Code in un IDE.

<h2 id="cloud-execution-security">
  Sicurezza dell'esecuzione nel cloud
</h2>

Quando si utilizza [Claude Code sul web](/docs/it/claude-code-on-the-web), sono in vigore controlli di sicurezza aggiuntivi:

* **Macchine virtuali isolate**: Ogni sessione cloud viene eseguita in una VM isolata gestita da Anthropic
* **Controlli di accesso alla rete**: L'accesso alla rete è limitato per impostazione predefinita e può essere configurato per essere disabilitato o consentire solo domini specifici
* **Protezione delle credenziali**: L'autenticazione viene gestita tramite un proxy sicuro che utilizza una credenziale con ambito all'interno della sandbox, che viene quindi tradotta nel vostro token di autenticazione GitHub effettivo
* **Restrizioni di ramo**: Le operazioni di push Git sono limitate al ramo di lavoro corrente
* **Registrazione di audit**: Tutte le operazioni negli ambienti cloud vengono registrate per scopi di conformità e audit
* **Pulizia automatica**: Gli ambienti cloud vengono terminati automaticamente al completamento della sessione

Per ulteriori dettagli sull'esecuzione nel cloud, vedere [Claude Code sul web](/docs/it/claude-code-on-the-web).

Le sessioni di [Remote Control](/docs/it/remote-control) funzionano diversamente: l'interfaccia web si connette a un processo Claude Code in esecuzione sulla vostra macchina locale. Tutta l'esecuzione del codice e l'accesso ai file rimangono locali, e il traffico della sessione viaggia attraverso l'API Anthropic su TLS; mentre connessi, la trascrizione della sessione viene archiviata sui server Anthropic per sincronizzare la conversazione tra i dispositivi, come descritto in [Connessione e sicurezza](/docs/it/remote-control#connection-and-security). Non sono coinvolte VM cloud o sandbox. La connessione utilizza più credenziali di breve durata e con ambito ristretto, ciascuna limitata a uno scopo specifico e con scadenza indipendente, per limitare il raggio di esplosione di qualsiasi singola credenziale compromessa.

<h2 id="security-best-practices">
  Migliori pratiche di sicurezza
</h2>

<h3 id="working-with-sensitive-code">
  Lavorare con codice sensibile
</h3>

* Rivedere tutte le modifiche suggerite prima dell'approvazione
* Utilizzare impostazioni di permesso specifiche del progetto per repository sensibili
* Considerare l'utilizzo di [dev containers](/docs/it/devcontainer) per un isolamento aggiuntivo
* Controllare regolarmente le impostazioni di permesso con `/permissions`

<h3 id="team-security">
  Sicurezza del team
</h3>

* Utilizzare [managed settings](/docs/it/settings#settings-files) per applicare gli standard organizzativi
* Condividere le configurazioni di permesso approvate tramite il controllo del codice sorgente
* Formare i membri del team sulle migliori pratiche di sicurezza
* Monitorare l'utilizzo di Claude Code tramite [metriche OpenTelemetry](/docs/it/monitoring-usage)
* Controllare o bloccare le modifiche alle impostazioni durante le sessioni con [`ConfigChange` hooks](/docs/it/hooks#configchange)

<h3 id="reporting-security-issues">
  Segnalazione di problemi di sicurezza
</h3>

Se scoprite una vulnerabilità di sicurezza in Claude Code:

1. Non divulgatela pubblicamente
2. Segnalatela tramite il nostro [programma HackerOne](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new)
3. Includete i passaggi di riproduzione dettagliati
4. Concedete il tempo necessario per affrontare il problema prima della divulgazione pubblica

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Security guidance plugin](/docs/it/security-guidance): consentire a Claude di rivedere e correggere le vulnerabilità nei propri cambiamenti di codice durante la sessione
* [Sandbox environments](/docs/it/sandbox-environments): confrontare gli approcci di isolamento e sceglierne uno per il vostro modello di minaccia
* [Sandboxing](/docs/it/sandboxing): isolamento del filesystem e della rete per i comandi Bash
* [Permissions](/docs/it/permissions): configurare i permessi e i controlli di accesso
* [Monitoring usage](/docs/it/monitoring-usage): tracciare e controllare l'attività di Claude Code
* [Development containers](/docs/it/devcontainer): ambienti sicuri e isolati
* [Anthropic Trust Center](https://trust.anthropic.com): certificazioni di sicurezza e conformità
