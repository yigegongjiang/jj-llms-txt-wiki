> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestisci i costi in modo efficace

> Traccia l'utilizzo dei token, imposta i limiti di spesa del team e riduci i costi di Claude Code con la gestione del contesto, la selezione del modello, le impostazioni del pensiero esteso e gli hook di pre-elaborazione.

Claude Code addebita il consumo di token API. Per i prezzi dei piani di abbonamento (Pro, Max, Team, Enterprise), vedi [claude.com/pricing](https://claude.com/pricing). I costi per sviluppatore variano notevolmente in base alla selezione del modello, alle dimensioni della codebase e ai modelli di utilizzo come l'esecuzione di più istanze o l'automazione.

Nelle distribuzioni aziendali, il costo medio è di circa \$13 per sviluppatore per giorno attivo e \$150-250 per sviluppatore al mese, con costi che rimangono al di sotto di \$30 per giorno attivo per il 90% degli utenti. Per stimare la spesa per il tuo team, inizia con un piccolo gruppo pilota e utilizza gli strumenti di tracciamento di seguito per stabilire una baseline prima di un rollout più ampio.

Questa pagina spiega come [tracciare i tuoi costi](#track-your-costs), [gestire i costi per la tua organizzazione](#manage-costs-for-your-organization) e [ridurre l'utilizzo dei token](#reduce-token-usage).

<h2 id="track-your-costs">
  Traccia i tuoi costi
</h2>

<h3 id="using-the-/usage-command">
  Utilizzo del comando `/usage`
</h3>

<Note>
  Il blocco Session in `/usage` mostra l'utilizzo dei token API ed è destinato agli utenti API. I sottoscrittori di Claude Max e Pro hanno l'utilizzo incluso nel loro abbonamento, quindi la cifra del costo della sessione non è rilevante per scopi di fatturazione. I sottoscrittori vedono barre di utilizzo del piano, statistiche di attività e una suddivisione dell'utilizzo sulla stessa schermata.
</Note>

Il blocco Session in cima a `/usage` mostra statistiche dettagliate sull'utilizzo dei token per la tua sessione attuale. La cifra in dollari è una stima calcolata localmente dai conteggi dei token e potrebbe differire dalla tua fattura effettiva. Per la fatturazione autorevole, vedi la pagina Usage nella [Claude Console](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 19.7s
Total duration (wall): 6h 33m 10.2s
Total code changes:    0 lines added, 0 lines removed
```

Su un piano Pro, Max, Team o Enterprise, `/usage` mostra anche una suddivisione di ciò che conta rispetto ai limiti del tuo piano. Attribuisce l'utilizzo recente a skills, subagents, plugins e singoli server MCP, ciascuno mostrato come percentuale del totale. Premi `d` o `w` per passare tra le ultime 24 ore e gli ultimi 7 giorni. Le cifre sono approssimative e calcolate dalla cronologia della sessione locale su questa macchina, quindi l'utilizzo da altri dispositivi o da claude.ai non è incluso.

Quando la richiesta per i limiti del tuo piano non riesce, il più delle volte perché l'endpoint di utilizzo è limitato dalla frequenza, `/usage` mostra le ultime barre di utilizzo caricate su questa macchina negli ultimi 60 minuti, insieme a una nota `Showing last-known usage` che indica quanto tempo fa sono stati recuperati i dati. Premi `r` per riprovare; un nuovo tentativo riuscito sostituisce le ultime barre conosciute con dati freschi. Senza uno snapshot degli ultimi 60 minuti, `/usage` segnala che l'endpoint di utilizzo è limitato dalla frequenza e offre lo stesso collegamento di riprovazione. Prima della v2.1.208, una richiesta limitata dalla frequenza in una sessione che non aveva ancora caricato l'utilizzo mostrava sempre l'errore senza barre.

Nell'[estensione VS Code](/docs/it/vs-code#check-account-and-usage), la stessa suddivisione appare nella finestra di dialogo Account & usage con un interruttore Day e Week. Richiede Claude Code v2.1.174 o versione successiva.

<h3 id="set-a-spend-limit-on-pro-and-max">
  Imposta un limite di spesa su Pro e Max
</h3>

Su piani Pro e Max, il comando `/usage-credits` apre una finestra di dialogo nella CLI dove puoi gestire i [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans). Dalla finestra di dialogo puoi:

* Attivare i crediti di utilizzo per il tuo account
* Acquistare più crediti di utilizzo, sia un bundle elencato che un importo personalizzato
* Impostare, modificare o rimuovere il tuo limite di spesa mensile
* Configurare il ricaricamento automatico, che acquista automaticamente più crediti di utilizzo quando il tuo saldo scende al di sotto di una soglia che imposti

Su Claude Code versioni precedenti alla v2.1.207 e su account dove la finestra di dialogo in-CLI non è disponibile, `/usage-credits` apre la pagina di fatturazione dei crediti di utilizzo nel tuo browser. Su piani Team e Enterprise, i membri con accesso alla fatturazione ottengono la stessa pagina del browser, e i membri senza accesso alla fatturazione inviano una richiesta dalla CLI chiedendo al loro amministratore di attivare i crediti di utilizzo o aumentare il limite.

La modifica del limite di spesa mensile richiede l'accesso alla fatturazione sull'account. Se raggiungi il limite mentre hai ancora crediti di utilizzo disponibili, Claude Code ti chiede di aumentare o rimuovere il limite in modo da poter continuare senza lasciare la CLI.

Gli importi che digiti nella finestra di dialogo, come un importo di acquisto personalizzato, il limite di spesa mensile o la soglia e l'obiettivo di ricaricamento automatico, devono essere cifre, facoltativamente seguite da un punto e una o due cifre decimali, ad esempio `20` o `20.50`. Qualsiasi altro input, incluse le virgole, mostra un errore in linea e non viene salvato. Le versioni precedenti alla v2.1.207 non mostrano la finestra di dialogo e aprono la pagina di fatturazione.

Claude Code ti chiede di digitare `yes` per confermare ogni acquisto e ogni modifica del ricaricamento automatico, indipendentemente dall'importo, e la conferma dell'acquisto mostra il totale al netto delle tasse che stai approvando. La modifica del limite di spesa mensile richiede la stessa conferma digitata solo al di sopra di \$1.000, o al di sopra di 1.000 unità di una valuta di fatturazione non in dollari USA. Prima della v2.1.208, gli acquisti e le modifiche del ricaricamento automatico utilizzavano quella soglia, quindi gli importi più piccoli passavano attraverso il flusso di dialogo standard senza il passaggio aggiuntivo di `yes` digitato.

I campi di importo si aprono precompilati con un valore suggerito, e la prima cifra che digiti sostituisce il suggerimento invece di aggiungersi ad esso. La schermata che attiva i crediti di utilizzo si apre con Cancel selezionato, quindi attivarli richiede una selezione deliberata piuttosto che un Enter casuale. Entrambi richiedono Claude Code v2.1.208 o versione successiva.

<h2 id="manage-costs-for-your-organization">
  Gestione dei costi per la tua organizzazione
</h2>

I controlli che hai a disposizione dipendono da come la tua organizzazione accede a Claude Code: un piano Claude for Teams o Enterprise, la Claude Console, o un provider cloud. Nei piani Teams e Enterprise, l'utilizzo viene prelevato dall'indennità di posto di ogni membro. Nella Console e presso i provider cloud, l'utilizzo viene fatturato per token alla tua organizzazione. Se la tua organizzazione mescola metodi di accesso, ogni sviluppatore viene misurato in base a quello con cui si è autenticato.

La tabella mappa ogni configurazione a dove vedi la spesa, dove la limiti, e come estrai i numeri per utente.

| La tua configurazione                                                                  | Visualizza spesa                                                                                                                       | Limita spesa                             | Rapporto per utente                                                                                                                                                                                                         |
| :------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams o Enterprise](#claude-for-teams-and-enterprise)                      | [Rapporto spesa in analitiche org](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Limiti di spesa nelle impostazioni admin | [CSV rapporto spesa](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) su Enterprise |
| [Claude Console (API)](#claude-console)                                                | [Pagina utilizzo Console](https://platform.claude.com/usage)                                                                           | Limiti di spesa dell'area di lavoro      | [Dashboard Console](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                  |
| [Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry](#cloud-providers) | La tua console di fatturazione cloud                                                                                                   | I controlli di budget del tuo cloud      | [OpenTelemetry](/docs/it/monitoring-usage) o un [gateway LLM](/docs/it/llm-gateway)                                                                                                                                                   |

[L'esportazione OpenTelemetry](/docs/it/monitoring-usage) funziona su ogni configurazione ed è l'unica opzione che trasmette metriche di token e costo per utente nel tuo stack di osservabilità in tempo quasi reale.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams e Enterprise
</h3>

Nei piani Claude for Teams e Enterprise, l'utilizzo di Claude Code di ogni membro viene prelevato da un'indennità per posto che si ripristina su una finestra mobile di cinque ore e una finestra settimanale. L'indennità è condivisa con Claude chat e Cowork, e la sua dimensione dipende dal [livello di posto](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) (Standard o Premium). I tuoi controlli si trovano nella console admin di claude.ai, non nella Claude Console.

* **Visualizza spesa**: il [rapporto spesa in analitiche org](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) mostra la spesa stimata per utente e per modello, con esportazione CSV, aggiornato quotidianamente. Il rapporto copre la spesa in crediti di utilizzo e appare una volta che i crediti di utilizzo sono attivati. L'utilizzo all'interno dell'indennità di posto non viene misurato in dollari.
* **Visualizza adozione**: il [dashboard analitiche](https://claude.ai/analytics/claude-code) mostra utenti attivi giornalieri, sessioni e metriche di contributo, con esportazione CSV dei dati di contributo. Vedi [traccia l'utilizzo del team con analitiche](/docs/it/analytics).
* **Limita spesa**: l'indennità di posto è il limite predefinito. Per consentire ai membri di continuare oltre, attiva i [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) e imposta limiti di spesa a livello organizzativo, di gruppo o di singolo membro.
* **Estrai numeri per utente**: nel piano Enterprise, l'[Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) restituisce rapporti di utilizzo e costo per utente su tutte le superfici Claude, incluso Claude Code. Un Primary Owner crea una chiave con l'ambito `read:analytics` su [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). Nel piano Teams, esporta il [CSV rapporto spesa](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), che elenca l'utilizzo dei token e la spesa stimata per utente e per modello.

La [guida al consumo Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) è il riferimento di pianificazione per gli amministratori. Spiega come il consumo differisce tra Claude chat, Claude Code e Cowork, e fornisce punti di partenza in dollari per utente per il budgeting. Stanzia di più per un posto di codifica rispetto a un posto di chat: ogni turno di Claude Code contiene contenuti di file, chiamate di strumenti e ragionamento multi-step, quindi una sessione di debug può consumare più di un giorno di chat.

<h3 id="claude-console">
  Claude Console
</h3>

Le organizzazioni API gestiscono la spesa di Claude Code attraverso [aree di lavoro](https://platform.claude.com/docs/en/build-with-claude/workspaces). Puoi [impostare limiti di spesa dell'area di lavoro](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) sulla spesa totale di Claude Code e [visualizzare rapporti di costo e utilizzo](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) nella Console.

<Note>
  Quando autentichi per la prima volta Claude Code con il tuo account Claude Console, viene creata automaticamente un'area di lavoro chiamata "Claude Code". Questa area di lavoro fornisce il tracciamento e la gestione centralizzati dei costi per tutto l'utilizzo di Claude Code nella tua organizzazione. Non puoi creare chiavi API per questa area di lavoro; è esclusivamente per l'autenticazione e l'utilizzo di Claude Code.

  Per le organizzazioni con limiti di velocità personalizzati, il traffico di Claude Code in questa area di lavoro conta verso i limiti di velocità API complessivi della tua organizzazione. Puoi impostare un [limite di velocità dell'area di lavoro](https://platform.claude.com/docs/it/api/rate-limits#setting-lower-limits-for-workspaces) sulla pagina Limits di questa area di lavoro nella Claude Console per limitare la quota di Claude Code e proteggere altri carichi di lavoro di produzione.
</Note>

Per la segnalazione per utente, il [dashboard Console](https://platform.claude.com/claude-code) mostra la spesa e le righe accettate per membro, e l'[Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) restituisce le stesse metriche giornaliere per utente a livello di programmazione con una [chiave API Admin](https://platform.claude.com/settings/admin-keys). Vedi [analitiche per clienti API](/docs/it/analytics#access-analytics-for-api-customers).

<h4 id="rate-limit-recommendations">
  Raccomandazioni sui limiti di velocità
</h4>

Quando configuri Claude Code per i team, considera queste raccomandazioni Token Per Minuto (TPM) e Richieste Per Minuto (RPM) per utente in base alle dimensioni della tua organizzazione:

| Dimensione del team | TPM per utente | RPM per utente |
| ------------------- | -------------- | -------------- |
| 1-5 utenti          | 200k-300k      | 5-7            |
| 5-20 utenti         | 100k-150k      | 2.5-3.5        |
| 20-50 utenti        | 50k-75k        | 1.25-1.75      |
| 50-100 utenti       | 25k-35k        | 0.62-0.87      |
| 100-500 utenti      | 15k-20k        | 0.37-0.47      |
| 500+ utenti         | 10k-15k        | 0.25-0.35      |

Ad esempio, se hai 200 utenti, potresti richiedere 20k TPM per ogni utente, o 4 milioni di TPM totali (200\*20.000 = 4 milioni).

Il TPM per utente diminuisce man mano che le dimensioni del team crescono perché meno utenti tendono a utilizzare Claude Code contemporaneamente nelle organizzazioni più grandi. Questi limiti di velocità si applicano a livello organizzativo, non per singolo utente, il che significa che i singoli utenti possono temporaneamente consumare più della loro quota calcolata quando altri non stanno utilizzando attivamente il servizio.

<Note>
  Se prevedi scenari con utilizzo concorrente insolitamente elevato (come sessioni di formazione dal vivo con grandi gruppi), potresti aver bisogno di allocazioni TPM più elevate per utente.
</Note>

<h3 id="cloud-providers">
  Provider cloud
</h3>

Su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, Claude Code viene fatturato per token al tuo account cloud, e i controlli di spesa si trovano nella console di fatturazione del tuo provider cloud. Claude Code non invia metriche dal tuo cloud ad Anthropic, quindi i [dashboard analitiche](/docs/it/analytics) e l'Claude Code Analytics API non coprono questo utilizzo.

Per l'attribuzione dei costi per utente, hai tre opzioni:

* **OpenTelemetry**: [esporta metriche](/docs/it/monitoring-usage) dalla macchina di ogni sviluppatore nel tuo stack di osservabilità. Questo ti dà conteggi di token per utente, costi e attività di strumenti indipendentemente dal provider.
* **Un gateway di app Claude**: un [gateway di app Claude](/docs/it/claude-apps-gateway) self-hosted fornisce l'attribuzione dell'utilizzo per utente, metriche OTLP con conteggi dei token e [limiti di spesa per utente](/docs/it/claude-apps-gateway-spend-limits) su questi provider.
* **Un gateway LLM**: instrada tutto il traffico di Claude Code attraverso un proxy che traccia la spesa per chiave. Diversi grandi enterprise hanno riferito di utilizzare [LiteLLM](/docs/it/llm-gateway), uno strumento open-source che [traccia la spesa per chiave](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend). Questo progetto non è affiliato ad Anthropic e non è stato sottoposto a audit di sicurezza.

<h3 id="when-a-developer-asks-about-a-limit">
  Quando uno sviluppatore chiede informazioni su un limite
</h3>

Gli sviluppatori di solito portano domande sui limiti al loro amministratore, quindi è utile sapere quale limite hanno raggiunto. Le tre situazioni significano cose diverse:

* **"Hai raggiunto il tuo limite di sessione" o "Hai raggiunto il tuo limite settimanale"**: una finestra di utilizzo basata su posto in un piano di abbonamento. Queste finestre sono condivise su tutti i modelli, quindi cambiare modelli con `/model` non ripristina l'accesso, anche se consente allo sviluppatore di continuare a lavorare dopo il messaggio specifico del modello "Hai raggiunto il tuo limite Opus". Il messaggio mostra quando la finestra si ripristina, e lo sviluppatore può eseguire `/usage-credits` per richiedere utilizzo oltre l'indennità se hai i [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) attivati. Vedi [errori di limite di utilizzo](/docs/it/errors#youve-hit-your-session-limit).
* **Un avviso di contesto o auto-compact**: non è un limite di utilizzo. La conversazione è cresciuta vicino alla dimensione massima di input del modello, e Claude Code riassume la cronologia più vecchia per liberare spazio. Indirizza lo sviluppatore a [riduci l'utilizzo dei token](#reduce-token-usage).
* **Spesa inaspettatamente alta su un piano API o provider cloud**: di solito risale a sessioni lunghe che non sono mai state cancellate o a Opus lasciato come modello predefinito. Le abitudini con il maggiore impatto da condividere sono cancellare tra compiti non correlati e abbinare il modello al lavoro, entrambi coperti in [riduci l'utilizzo dei token](#reduce-token-usage).

<h3 id="agent-team-token-costs">
  Costi dei token del team di agenti
</h3>

I [team di agenti](/docs/it/agent-teams) generano più istanze di Claude Code, ognuna con la propria finestra di contesto. L'utilizzo dei token si ridimensiona con il numero di compagni di squadra attivi e per quanto tempo ognuno viene eseguito.

Per mantenere i costi del team di agenti gestibili:

* Utilizza Sonnet per i compagni di squadra. Bilancia la capacità e il costo per i compiti di coordinamento.
* Mantieni i team piccoli. Ogni compagno di squadra esegue la propria finestra di contesto, quindi l'utilizzo dei token è approssimativamente proporzionale alle dimensioni del team.
* Mantieni i prompt di generazione focalizzati. I compagni di squadra caricano automaticamente CLAUDE.md, i server MCP e le skills, ma tutto nel prompt di generazione si aggiunge al loro contesto dall'inizio.
* Spegni i compagni di squadra quando il loro lavoro è terminato. Ogni compagno di squadra attivo continua a consumare token finché non esce o la sessione termina.
* I team di agenti sono disabilitati per impostazione predefinita. Imposta `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` nel tuo [settings.json](/docs/it/settings) o nell'ambiente per abilitarli. Vedi [abilita i team di agenti](/docs/it/agent-teams#enable-agent-teams).

<h2 id="reduce-token-usage">
  Riduci l'utilizzo dei token
</h2>

I costi dei token si ridimensionano con la dimensione del contesto: più contesto Claude elabora, più token utilizzi. Claude Code ottimizza automaticamente i costi attraverso il [prompt caching](/docs/it/prompt-caching), che riduce i costi per il contenuto ripetuto come i prompt di sistema, e l'auto-compaction, che riassume la cronologia della conversazione quando ci si avvicina ai limiti del contesto.

Le seguenti strategie ti aiutano a mantenere il contesto piccolo e ridurre i costi per messaggio.

<h3 id="manage-context-proactively">
  Gestisci il contesto in modo proattivo
</h3>

Utilizza `/usage` per controllare l'utilizzo attuale dei token, o [configura la tua linea di stato](/docs/it/statusline#context-window-usage) per visualizzarla continuamente.

* **Cancella tra i compiti**: Utilizza `/clear` per ricominciare da capo quando passi a lavori non correlati. Il contesto obsoleto spreca token su ogni messaggio successivo. Utilizza `/rename` prima di cancellare in modo da poter trovare facilmente la sessione in seguito, quindi `/resume` per tornare ad essa.
* **Aggiungi istruzioni di compaction personalizzate**: `/compact Focus on code samples and API usage` dice a Claude cosa preservare durante la sintesi.

Puoi anche personalizzare il comportamento della compaction nel tuo file CLAUDE.md nella radice del tuo progetto:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Scegli il modello giusto
</h3>

Sonnet gestisce bene la maggior parte dei compiti di codifica e costa meno di Opus. Riserva Opus per decisioni architettoniche complesse o ragionamento multi-step. Utilizza `/model` per cambiare modello a metà sessione, o imposta un valore predefinito in `/config`. Per semplici compiti subagent, specifica `model: haiku` nella tua [configurazione subagent](/docs/it/sub-agents#choose-a-model).

<h3 id="reduce-mcp-server-overhead">
  Riduci l'overhead del server MCP
</h3>

Le definizioni degli strumenti MCP sono [rinviate per impostazione predefinita](/docs/it/mcp#scale-with-mcp-tool-search), quindi solo i nomi degli strumenti entrano nel contesto finché Claude non utilizza uno strumento specifico. Esegui `/context` per vedere cosa sta consumando spazio.

* **Preferisci gli strumenti CLI quando disponibili**: Strumenti come `gh`, `aws`, `gcloud` e `sentry-cli` sono ancora più efficienti dal punto di vista del contesto rispetto ai server MCP perché non aggiungono alcun elenco di strumenti per strumento. Claude può eseguire comandi CLI direttamente.
* **Disabilita i server inutilizzati**: Esegui `/mcp` per vedere i server configurati e disabilita quelli che non stai utilizzando attivamente.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Installa plugin di intelligenza del codice per i linguaggi tipizzati
</h3>

I [plugin di intelligenza del codice](/docs/it/discover-plugins#code-intelligence) danno a Claude una navigazione precisa dei simboli invece della ricerca basata su testo, riducendo le letture di file non necessarie quando si esplora codice sconosciuto. Una singola chiamata "vai alla definizione" sostituisce quello che altrimenti potrebbe essere un grep seguito dalla lettura di più file candidati. I server di linguaggio installati segnalano anche gli errori di tipo automaticamente dopo le modifiche, quindi Claude cattura gli errori senza eseguire un compilatore.

<h3 id="offload-processing-to-hooks-and-skills">
  Offload dell'elaborazione agli hook e alle skills
</h3>

Gli [hook](/docs/it/hooks) personalizzati possono pre-elaborare i dati prima che Claude li veda. Invece di Claude che legge un file di log di 10.000 righe per trovare errori, un hook può cercare `ERROR` e restituire solo le righe corrispondenti, riducendo il contesto da decine di migliaia di token a centinaia.

Una [skill](/docs/it/skills) può dare a Claude la conoscenza del dominio in modo che non debba esplorare. Ad esempio, una skill "codebase-overview" potrebbe descrivere l'architettura del tuo progetto, le directory chiave e le convenzioni di denominazione. Quando Claude invoca la skill, ottiene questo contesto immediatamente invece di spendere token leggendo più file per comprendere la struttura.

Ad esempio, questo hook PreToolUse filtra l'output del test per mostrare solo i fallimenti:

<Tabs>
  <Tab title="settings.json">
    Aggiungi questo al tuo [settings.json](/docs/it/settings#settings-files) per eseguire l'hook prima di ogni comando Bash:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    L'hook chiama questo script. Crea la cartella con `mkdir -p ~/.claude/hooks`, salva lo script sottostante come `~/.claude/hooks/filter-test-output.sh` e rendilo eseguibile con `chmod +x ~/.claude/hooks/filter-test-output.sh`. Controlla se il comando è un test runner e lo modifica per mostrare solo i fallimenti:

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"updatedInput\":{\"command\":\"$filtered_cmd\"}}}"
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

<h3 id="move-instructions-from-claude-md-to-skills">
  Sposta le istruzioni da CLAUDE.md alle skills
</h3>

Il tuo file [CLAUDE.md](/docs/it/memory) viene caricato nel contesto all'inizio della sessione. Se contiene istruzioni dettagliate per flussi di lavoro specifici (come revisioni PR o migrazioni di database), quei token sono presenti anche quando stai facendo lavori non correlati. Le [skills](/docs/it/skills) si caricano su richiesta solo quando invocate, quindi spostare le istruzioni specializzate nelle skills mantiene il tuo contesto di base più piccolo. Mira a mantenere CLAUDE.md sotto 200 righe includendo solo gli elementi essenziali.

<h3 id="adjust-extended-thinking">
  Regola il pensiero esteso
</h3>

Il pensiero esteso è abilitato per impostazione predefinita perché migliora significativamente le prestazioni su compiti complessi di pianificazione e ragionamento. I token di pensiero vengono fatturati come token di output, e il budget predefinito può essere decine di migliaia di token per richiesta a seconda del modello. Per compiti più semplici dove il ragionamento profondo non è necessario, puoi ridurre i costi abbassando il [livello di sforzo](/docs/it/model-config#adjust-effort-level) con `/effort` o in `/model`, disabilitando il pensiero in `/config`, o, su modelli con un [budget di pensiero fisso](/docs/it/model-config#adaptive-reasoning-and-fixed-thinking-budgets), abbassando il budget impostando la [variabile di ambiente](/docs/it/env-vars) `MAX_THINKING_TOKENS`, ad esempio `MAX_THINKING_TOKENS=8000`. I modelli di ragionamento adattivo ignorano i budget diversi da zero, quindi utilizza i livelli di sforzo lì. La disabilitazione del pensiero non è disponibile su Fable 5, che utilizza sempre il pensiero esteso.

<h3 id="delegate-verbose-operations-to-subagents">
  Delega le operazioni dettagliate ai subagent
</h3>

L'esecuzione di test, il recupero della documentazione o l'elaborazione di file di log possono consumare un contesto significativo. Delega questi ai [subagent](/docs/it/sub-agents#isolate-high-volume-operations) in modo che l'output dettagliato rimanga nel contesto del subagent mentre solo un riassunto ritorna alla tua conversazione principale.

<h3 id="manage-agent-team-costs">
  Gestisci i costi del team di agenti
</h3>

I team di agenti utilizzano approssimativamente 7 volte più token rispetto alle sessioni standard quando i compagni di squadra vengono eseguiti in plan mode, perché ogni compagno di squadra mantiene la propria finestra di contesto ed esegue come un'istanza Claude separata. Mantieni i compiti del team piccoli e autonomi per limitare l'utilizzo dei token per compagno di squadra. Vedi [team di agenti](/docs/it/agent-teams) per i dettagli.

<h3 id="write-specific-prompts">
  Scrivi prompt specifici
</h3>

Richieste vaghe come "migliora questa codebase" attivano una scansione ampia. Richieste specifiche come "aggiungi la convalida dell'input alla funzione di accesso in auth.ts" permettono a Claude di lavorare in modo efficiente con letture di file minime.

<h3 id="work-efficiently-on-complex-tasks">
  Lavora in modo efficiente su compiti complessi
</h3>

Per lavori più lunghi o complessi, queste abitudini aiutano a evitare token sprecati andando nella direzione sbagliata:

* **Utilizza plan mode per compiti complessi**: Premi Shift+Tab per entrare in [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) prima dell'implementazione. Claude esplora la codebase e propone un approccio per la tua approvazione, prevenendo la rielaborazione costosa quando la direzione iniziale è sbagliata.
* **Correggi la rotta presto**: Se Claude inizia a andare nella direzione sbagliata, premi Escape per fermarti immediatamente. Utilizza `/rewind` o doppio tocco Escape per ripristinare la conversazione e il codice a un checkpoint precedente.
* **Fornisci target di verifica**: Includi casi di test, incolla screenshot o definisci l'output previsto nel tuo prompt. Quando Claude può verificare il suo lavoro, cattura i problemi prima che tu debba richiedere correzioni.
* **Testa in modo incrementale**: Scrivi un file, testalo, quindi continua. Questo cattura i problemi presto quando sono economici da risolvere.

<h2 id="background-token-usage">
  Utilizzo dei token in background
</h2>

Claude Code utilizza token per alcune funzionalità in background anche quando inattivo:

* **Sintesi della conversazione**: Processi in background che riassumono le conversazioni precedenti per la funzione `claude --resume`
* **Elaborazione dei comandi**: Alcuni comandi come `/usage` possono generare richieste per controllare lo stato

Questi processi in background consumano una piccola quantità di token (in genere meno di \$0,04 per sessione) anche senza interazione attiva.

<h2 id="understanding-changes-in-claude-code-behavior">
  Comprensione dei cambiamenti nel comportamento di Claude Code
</h2>

Claude Code riceve regolarmente aggiornamenti che possono cambiare il funzionamento delle funzionalità, inclusa la segnalazione dei costi. Esegui `claude --version` per controllare la tua versione attuale. Per domande specifiche sulla fatturazione, contatta il supporto di Anthropic tramite il tuo [account Console](https://platform.claude.com/login).
