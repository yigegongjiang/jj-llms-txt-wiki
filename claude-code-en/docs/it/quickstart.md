> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guida rapida

> Benvenuto in Claude Code!

Questa guida rapida ti permetterà di utilizzare l'assistenza alla codifica basata su IA in pochi minuti. Alla fine, comprenderai come utilizzare Claude Code per le attività di sviluppo comuni.

<h2 id="before-you-begin">
  Prima di iniziare
</h2>

Assicurati di avere:

* Un terminale o un prompt dei comandi aperto
  * Se non hai mai utilizzato il terminale prima, consulta la [guida del terminale](/docs/it/terminal-guide)
* Un progetto di codice con cui lavorare
* Un [abbonamento Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team o Enterprise), un account [Claude Console](https://platform.claude.com/) o accesso tramite un [provider cloud supportato](/docs/it/third-party-integrations)

<Note>
  Questa guida copre il CLI del terminale. Claude Code è disponibile anche sul [web](https://claude.ai/code), come [app desktop](/docs/it/desktop), in [VS Code](/docs/it/vs-code) e [IDE JetBrains](/docs/it/jetbrains), in [Slack](/docs/it/slack) e in CI/CD con [GitHub Actions](/docs/it/github-actions) e [GitLab](/docs/it/gitlab-ci-cd). Vedi [tutte le interfacce](/docs/it/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Passaggio 1: Installa Claude Code
</h2>

Per installare Claude Code, utilizza uno dei seguenti metodi:

<Tabs>
  <Tab title="Installazione nativa (consigliata)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    Se vedi `The token '&&' is not a valid statement separator`, sei in PowerShell, non in CMD. Se vedi `'irm' is not recognized as an internal or external command`, sei in CMD, non in PowerShell. Il tuo prompt mostra `PS C:\` quando sei in PowerShell e `C:\` senza il `PS` quando sei in CMD.

    Se il comando di installazione non riesce con `syntax error near unexpected token '<'`, un `403`, o un altro errore curl, consulta [Troubleshoot installation](/docs/it/troubleshoot-install#find-your-error) per abbinare l'errore a una soluzione e per metodi di installazione alternativi.

    [Git for Windows](https://git-scm.com/downloads/win) è consigliato su Windows nativo in modo che Claude Code possa utilizzare lo strumento Bash. Se Git for Windows non è installato, Claude Code utilizza PowerShell come strumento shell. Le configurazioni WSL non necessitano di Git for Windows.

    <Info>
      Le installazioni native si aggiornano automaticamente in background per mantenerti sulla versione più recente.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offre due cask. `claude-code` traccia il canale di rilascio stabile, che in genere è circa una settimana indietro e salta i rilasci con regressioni importanti. `claude-code@latest` traccia il canale più recente e riceve nuove versioni non appena vengono rilasciate.

    <Info>
      Le installazioni Homebrew non si aggiornano automaticamente. Esegui `brew upgrade claude-code` o `brew upgrade claude-code@latest`, a seconda di quale cask hai installato, per ottenere le funzionalità più recenti e le correzioni di sicurezza.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      Le installazioni WinGet non si aggiornano automaticamente. Esegui `winget upgrade Anthropic.ClaudeCode` periodicamente per ottenere le funzionalità più recenti e le correzioni di sicurezza.
    </Info>
  </Tab>
</Tabs>

Puoi anche installare con [apt, dnf, o apk](/docs/it/setup#install-with-linux-package-managers) su Debian, Fedora, RHEL e Alpine.

Per confermare che l'installazione ha funzionato, esegui:

```bash theme={null}
claude --version
```

Il comando stampa un numero di versione seguito da `(Claude Code)`.

<h2 id="step-2-log-in-to-your-account">
  Passaggio 2: Accedi al tuo account
</h2>

Claude Code richiede un account per essere utilizzato. Avvia una sessione interattiva con il comando `claude` e ti verrà chiesto di effettuare l'accesso al primo utilizzo:

```bash theme={null}
claude
```

Per gli account Claude subscription o Console, segui i prompt per completare l'autenticazione nel tuo browser. Se hai impostato la variabile di ambiente `ANTHROPIC_API_KEY`, Claude Code salta il prompt di accesso e ti chiede invece di approvare la chiave. Per cambiare account in seguito o effettuare nuovamente l'autenticazione, digita `/login` all'interno della sessione in esecuzione:

```text wrap theme={null}
/login
```

Puoi accedere utilizzando uno di questi tipi di account:

* [Claude Pro, Max, Team o Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (consigliato)
* [Claude Console](https://platform.claude.com/) (accesso API con crediti prepagati). Al primo accesso, uno spazio di lavoro "Claude Code" viene creato automaticamente nella Console per il tracciamento centralizzato dei costi.
* [Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry](/docs/it/third-party-integrations) (provider cloud aziendali)
* Un [gateway di app Claude](/docs/it/claude-apps-gateway) auto-ospitato, se la tua organizzazione ne esegue uno: il tuo amministratore pre-configura l'URL del gateway, e `/login` si apre direttamente sulla schermata **Cloud gateway** per consentire l'accesso con SSO aziendale

Una volta effettuato l'accesso, le tue credenziali vengono archiviate e non dovrai accedere di nuovo. Scopri di più in [Gestione delle credenziali](/docs/it/authentication#credential-management).

<h2 id="step-3-start-your-first-session">
  Passaggio 3: Avvia la tua prima sessione
</h2>

Apri il tuo terminale in qualsiasi directory del progetto e avvia Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Sostituisci `/path/to/your/project` con il percorso del progetto su cui desideri lavorare.

Vedrai il prompt di Claude Code con la versione, il modello attuale e la directory di lavoro mostrati sopra. Digita `/help` per i comandi disponibili o `/resume` per continuare una conversazione precedente.

<h2 id="step-4-ask-your-first-question">
  Passaggio 4: Fai la tua prima domanda
</h2>

Iniziamo con la comprensione della tua base di codice. Prova uno di questi comandi:

```text wrap theme={null}
cosa fa questo progetto?
```

Claude analizzerà i tuoi file e fornirà un riepilogo. Puoi anche fare domande più specifiche:

```text wrap theme={null}
quali tecnologie utilizza questo progetto?
```

```text wrap theme={null}
dov'è il punto di ingresso principale?
```

```text wrap theme={null}
spiega la struttura delle cartelle
```

Puoi anche chiedere a Claude informazioni sulle sue stesse capacità:

```text wrap theme={null}
cosa può fare Claude Code?
```

```text wrap theme={null}
come creo skill personalizzate in Claude Code?
```

```text wrap theme={null}
Claude Code può funzionare con Docker?
```

<Note>
  Claude Code legge i file del tuo progetto secondo le necessità. Non devi aggiungere manualmente il contesto.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Passaggio 5: Fai il tuo primo cambio di codice
</h2>

Ora facciamo in modo che Claude Code faccia un po' di codifica vera. Prova un'attività semplice:

```text wrap theme={null}
aggiungi una funzione hello world al file principale
```

Claude Code trova il file appropriato e ti mostra la modifica. Se chiede prima di apportare la modifica, seleziona **Sì** per approvare.

La modalità Auto è la [modalità di autorizzazione iniziale integrata](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) per le sessioni di terminale interattive sui piani Pro, Max e Team: un classificatore esamina le azioni invece di voi, e Claude modifica la maggior parte dei file ed esegue la maggior parte dei comandi senza chiedere. Su altri piani, la modalità Manual è la modalità di autorizzazione iniziale integrata. Per la sessione che avviate subito dopo l'installazione, vedere [Prima sessione dopo un'installazione o un aggiornamento](/docs/it/env-vars#first-session-after-an-install-or-upgrade).

<Note>
  Le vostre impostazioni o la vostra organizzazione possono impostare una modalità di autorizzazione iniziale diversa. [Quale modalità di autorizzazione una sessione inizia](/docs/it/permission-modes#which-mode-a-session-starts-in) elenca cosa lo fa. Premete `Shift+Tab` in qualsiasi momento per cambiare la modalità di autorizzazione della sessione in cui vi trovate.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Passaggio 6: Usa Git con Claude Code
</h2>

Claude Code rende le operazioni Git conversazionali:

```text wrap theme={null}
quali file ho modificato?
```

```text wrap theme={null}
esegui il commit delle mie modifiche con un messaggio descrittivo
```

Puoi anche richiedere operazioni Git più complesse:

```text wrap theme={null}
crea un nuovo branch chiamato feature/quickstart
```

```text wrap theme={null}
mostrami gli ultimi 5 commit
```

```text wrap theme={null}
aiutami a risolvere i conflitti di merge
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Passaggio 7: Correggi un bug o aggiungi una funzionalità
</h2>

Claude è abile nel debug e nell'implementazione di funzionalità.

Descrivi quello che vuoi in linguaggio naturale:

```text wrap theme={null}
aggiungi la convalida dell'input al modulo di registrazione dell'utente
```

O correggi i problemi esistenti:

```text wrap theme={null}
c'è un bug in cui gli utenti possono inviare moduli vuoti - correggilo
```

Claude Code farà:

* Individuare il codice rilevante
* Comprendere il contesto
* Implementare una soluzione
* Eseguire i test se disponibili

<h2 id="step-8-test-out-other-common-workflows">
  Passaggio 8: Prova altri flussi di lavoro comuni
</h2>

Ci sono diversi modi per lavorare con Claude:

**Refactoring del codice**

```text wrap theme={null}
refactorizza il modulo di autenticazione per utilizzare async/await invece di callback
```

**Scrivi test**

```text wrap theme={null}
scrivi unit test per le funzioni della calcolatrice
```

**Aggiorna la documentazione**

```text wrap theme={null}
aggiorna il README con le istruzioni di installazione
```

**Revisione del codice**

```text wrap theme={null}
rivedi le mie modifiche e suggerisci miglioramenti
```

<Tip>
  Parla a Claude come faresti con un collega utile. Descrivi quello che vuoi ottenere e ti aiuterà a raggiungerlo.
</Tip>

<h2 id="essential-commands">
  Comandi essenziali
</h2>

Ecco i comandi più importanti per l'uso quotidiano. I comandi shell vengono eseguiti dal vostro terminale per avviare o riprendere Claude Code. I comandi di sessione vengono eseguiti all'interno di Claude Code dopo l'avvio.

**Comandi shell**

| Comando             | Cosa fa                                                        | Esempio                             |
| ------------------- | -------------------------------------------------------------- | ----------------------------------- |
| `claude`            | Avvia la modalità interattiva                                  | `claude`                            |
| `claude "task"`     | Avvia la modalità interattiva con un prompt iniziale           | `claude "fix the build error"`      |
| `claude -p "query"` | Esegui una query una tantum, quindi esci                       | `claude -p "explain this function"` |
| `claude -c`         | Continua la conversazione più recente nella directory corrente | `claude -c`                         |
| `claude -r`         | Riprendi una conversazione precedente                          | `claude -r`                         |

**Comandi di sessione**

| Comando                    | Cosa fa                                    | Esempio  |
| -------------------------- | ------------------------------------------ | -------- |
| `/clear`                   | Cancella la cronologia della conversazione | `/clear` |
| `/help`                    | Mostra i comandi disponibili               | `/help`  |
| `/exit` o Ctrl+D due volte | Esci da Claude Code                        | `/exit`  |

Vedi il [riferimento CLI](/docs/it/cli-reference) per l'elenco completo dei comandi shell e il [riferimento comandi](/docs/it/commands) per l'elenco completo dei comandi di sessione.

<h2 id="pro-tips-for-beginners">
  Suggerimenti professionali per i principianti
</h2>

Per ulteriori informazioni, vedi [best practices](/docs/it/best-practices) e [flussi di lavoro comuni](/docs/it/common-workflows).

<AccordionGroup>
  <Accordion title="Sii specifico con le tue richieste">
    Invece di: "correggi il bug"

    Prova: "correggi il bug di accesso in cui gli utenti vedono una schermata vuota dopo aver inserito credenziali errate"
  </Accordion>

  <Accordion title="Usa istruzioni passo dopo passo">
    Suddividi i compiti complessi in passaggi:

    ```text wrap theme={null}
    1. crea una nuova tabella di database per i profili utente
    2. crea un endpoint API per ottenere e aggiornare i profili utente
    3. costruisci una pagina web che consenta agli utenti di visualizzare e modificare le loro informazioni
    ```
  </Accordion>

  <Accordion title="Lascia che Claude esplori prima">
    Prima di apportare modifiche, lascia che Claude comprenda il tuo codice:

    ```text wrap theme={null}
    analizza lo schema del database
    ```

    ```text wrap theme={null}
    costruisci una dashboard che mostra i prodotti che vengono restituiti più frequentemente dai nostri clienti nel Regno Unito
    ```
  </Accordion>

  <Accordion title="Risparmia tempo con le scorciatoie">
    * Digita `/` per vedere tutti i comandi e le skills disponibili
    * Usa Tab per il completamento dei comandi
    * Premi ↑ per la cronologia dei comandi
    * Premi `Shift+Tab` per ciclo tra le modalità di autorizzazione
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  Cosa fare dopo?
</h2>

Ora che hai imparato le nozioni di base, esplora funzionalità più avanzate:

<CardGroup cols={2}>
  <Card title="Come funziona Claude Code" icon="microchip" href="/docs/it/how-claude-code-works">
    Comprendi il loop agentico, gli strumenti integrati e come Claude Code interagisce con il tuo progetto
  </Card>

  <Card title="Best practices" icon="star" href="/docs/it/best-practices">
    Ottieni risultati migliori con prompt efficaci e configurazione del progetto
  </Card>

  <Card title="Flussi di lavoro comuni" icon="graduation-cap" href="/docs/it/common-workflows">
    Guide passo dopo passo per attività comuni
  </Card>

  <Card title="Estendi Claude Code" icon="puzzle-piece" href="/docs/it/features-overview">
    Personalizza con CLAUDE.md, skills, hooks, MCP e altro
  </Card>
</CardGroup>

<h2 id="getting-help">
  Ottenere aiuto
</h2>

* **In Claude Code**: Digita `/help` o chiedi "come faccio a..."
* **Documentazione**: Sei qui! Sfoglia altre guide
* **Corsi**: Segui [Claude Code 101](https://academy.claude.com/courses/claude-code-101) e altri corsi gratuiti a tuo ritmo su [Claude Academy](https://academy.claude.com/)
* **Community**: Unisciti al nostro [Discord](https://www.anthropic.com/discord) per suggerimenti e supporto
