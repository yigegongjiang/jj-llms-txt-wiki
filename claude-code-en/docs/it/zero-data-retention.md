> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Zero data retention

> Scopri Zero Data Retention (ZDR) per Claude Code, disponibile per account qualificati su Claude for Enterprise, inclusi ambito, funzionalità disabilitate e come richiedere l'abilitazione.

Zero Data Retention (ZDR) per Claude Code è disponibile per account qualificati su Claude for Enterprise. Quando ZDR è abilitato, i prompt e le risposte del modello generate durante le sessioni di Claude Code vengono elaborate in tempo reale e non vengono archiviate da Anthropic dopo la restituzione della risposta, tranne dove necessario per conformarsi alla legge o combattere l'uso improprio.

<Note>
  ZDR non è incluso nel piano standard Claude for Enterprise e non può essere abilitato dalle impostazioni di amministrazione. È disponibile per account qualificati e richiede un'abilitazione separata da parte di Anthropic. Se la vostra organizzazione richiede ZDR, [contattate il team di vendita](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) o il vostro team di account Anthropic per confermare l'idoneità.
</Note>

ZDR su Claude for Enterprise offre ai clienti enterprise la possibilità di utilizzare Claude Code con zero data retention e accedere alle funzionalità amministrative:

* Controlli dei costi per utente
* Dashboard [Analytics](/docs/it/analytics)
* [Server-managed settings](/docs/it/server-managed-settings)
* Audit log

ZDR per Claude Code su Claude for Enterprise si applica solo alla piattaforma diretta di Anthropic. Per i deployment di Claude su Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, fare riferimento alle politiche di data retention di quelle piattaforme.

<h2 id="zdr-scope">
  Ambito di ZDR
</h2>

ZDR copre l'inferenza di Claude Code su Claude for Enterprise.

<Warning>
  ZDR è abilitato su base per-organizzazione. Ogni nuova organizzazione richiede che ZDR sia abilitato separatamente dal team dell'account Anthropic. ZDR non si applica automaticamente alle nuove organizzazioni create nello stesso account. Contattare il team dell'account per abilitare ZDR per qualsiasi nuova organizzazione.
</Warning>

<h3 id="route-claude-code-traffic-to-your-zdr-organization">
  Instradare il traffico di Claude Code verso la vostra organizzazione ZDR
</h3>

ZDR si applica alle richieste che si autenticano in un'organizzazione abilitata per ZDR. Se uno sviluppatore accede a Claude Code con un account personale o con una chiave API da un'organizzazione diversa, quelle sessioni non sono coperte. Per richiedere che gli accessi claude.ai degli sviluppatori appartengano alla vostra organizzazione ZDR, distribuire le impostazioni gestite `forceLoginMethod` e `forceLoginOrgUUID`; vedere [Limitare l'accesso alla vostra organizzazione](/docs/it/authentication#restrict-login-to-your-organization), che spiega anche come queste chiavi trattano gli accessi a Claude Console.

<h3 id="what-zdr-covers">
  Cosa copre ZDR
</h3>

ZDR copre le chiamate di inferenza del modello effettuate tramite Claude Code su Claude for Enterprise. Quando si utilizza Claude Code nel terminale, i prompt inviati e le risposte generate da Claude non vengono conservate da Anthropic. Questo si applica a ogni modello disponibile per la vostra organizzazione ZDR. Alcuni modelli richiedono la conservazione dei dati per impostazione predefinita; vedere [Disponibilità dei modelli in ZDR](#model-availability-under-zdr).

<h3 id="what-zdr-does-not-cover">
  Cosa non copre ZDR
</h3>

ZDR non si estende ai seguenti elementi, anche per le organizzazioni con ZDR abilitato. Queste funzionalità seguono le [politiche standard di data retention](/docs/it/data-usage#data-retention):

| Funzionalità                | Dettagli                                                                                                                                                                                                                                                                                 |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Chat su claude.ai           | Le conversazioni di chat tramite l'interfaccia web Claude for Enterprise non sono coperte da ZDR.                                                                                                                                                                                        |
| Cowork                      | Le sessioni Cowork non sono coperte da ZDR.                                                                                                                                                                                                                                              |
| Claude Code Analytics       | Non archivia prompt o risposte del modello, ma raccoglie metadati di produttività come email dell'account e statistiche di utilizzo. Le metriche di contributo non sono disponibili per le organizzazioni ZDR; il [dashboard analytics](/docs/it/analytics) mostra solo metriche di utilizzo. |
| Gestione utenti e posti     | I dati amministrativi come email dell'account e assegnazioni di posti vengono conservati secondo le politiche standard.                                                                                                                                                                  |
| Integrazioni di terze parti | I dati elaborati da strumenti di terze parti, MCP servers o altre integrazioni esterne non sono coperti da ZDR. Esaminare indipendentemente le pratiche di gestione dei dati di questi servizi.                                                                                          |

<h2 id="features-disabled-under-zdr">
  Funzionalità disabilitate in ZDR
</h2>

Quando ZDR è abilitato per un'organizzazione Claude Code su Claude for Enterprise, determinate funzionalità che richiedono l'archiviazione di prompt o completamenti vengono automaticamente disabilitate a livello di backend:

| Funzionalità                                                                                                        | Motivo                                                                                                                           |
| ------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| [Cloud sessions](/docs/it/claude-code-on-the-web), incluse quelle avviate dall'[app Desktop](/docs/it/desktop#cloud-sessions) | Richiede l'archiviazione lato server dei dati di sessione, inclusa la cronologia delle conversazioni con prompt e completamenti. |
| [Claude Tag](https://claude.com/docs/claude-tag)                                                                    | Conserva la memoria del canale e le trascrizioni della sessione.                                                                 |
| [Artifacts](/docs/it/artifacts)                                                                                          | Richiede l'archiviazione del contenuto della pagina pubblicata sull'infrastruttura gestita da Anthropic.                         |
| Invio di feedback (`/feedback`, `/bug`, `/share`)                                                                   | L'invio di feedback invia i dati della conversazione ad Anthropic.                                                               |
| [Remote Control](/docs/it/remote-control)                                                                                | Archivia la trascrizione della sessione sui server Anthropic per sincronizzare la conversazione tra i dispositivi.               |

Queste funzionalità sono bloccate nel backend indipendentemente dalla visualizzazione lato client. Se si vede una funzionalità disabilitata nel terminale Claude Code durante l'avvio, il tentativo di utilizzarla restituisce un errore che indica che le politiche dell'organizzazione non consentono tale azione.

Le funzionalità future potrebbero anche essere disabilitate se richiedono l'archiviazione di prompt o completamenti.

<h3 id="model-availability-under-zdr">
  Disponibilità dei modelli in ZDR
</h3>

Claude Fable 5.1 e Fable 5 sono [Covered Models](https://support.claude.com/en/articles/15425695-covered-models) che [richiedono la conservazione dei dati](https://platform.claude.com/docs/en/manage-claude/api-and-data-retention#model-specific-data-retention-requirements) per impostazione predefinita, e se un'organizzazione o uno spazio di lavoro ZDR può utilizzarli è governato dalle politiche Covered Models piuttosto che da Claude Code. Laddove l'organizzazione non possa utilizzarli, i modelli sono assenti dal selettore `/model` oppure vengono visualizzati come disabilitati, e il server rifiuta le richieste per essi indipendentemente dalla configurazione del client.

Gli altri modelli rimangono disponibili in ZDR. I modelli Fable non sono il modello predefinito, e l'alias `best`, che si risolve nel modello Fable più recente dove è disponibile, si risolve in Opus per le organizzazioni dove non lo è.

<h2 id="data-retention-for-policy-violations">
  Data retention per violazioni delle politiche
</h2>

Anche con ZDR abilitato, Anthropic può conservare i dati dove richiesto dalla legge o per affrontare violazioni della Usage Policy. Se una sessione viene contrassegnata per una violazione della politica, Anthropic può conservare gli input e gli output associati per un massimo di 2 anni, in linea con la politica ZDR standard di Anthropic.

<h2 id="request-zdr">
  Richiedere ZDR
</h2>

Per richiedere ZDR per Claude Code su Claude for Enterprise, [contattare il team di vendita](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) o il team dell'account Anthropic. Il team dell'account presenterà la richiesta internamente e Anthropic esaminerà e abiliterà ZDR sulla vostra organizzazione dopo aver confermato l'idoneità. Tutte le azioni di abilitazione vengono registrate negli audit log.

Se attualmente si utilizza ZDR per Claude Code tramite chiavi API pay-as-you-go, è possibile passare a Claude for Enterprise per ottenere l'accesso alle funzionalità amministrative mantenendo ZDR per Claude Code. Contattare il team dell'account per coordinare la migrazione.
