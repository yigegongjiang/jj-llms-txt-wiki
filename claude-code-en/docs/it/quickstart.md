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
* Un [abbonamento Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team o Enterprise), un account [Claude Console](https://console.anthropic.com/) o accesso tramite un [provider cloud supportato](/docs/it/third-party-integrations)

<Note>
  Questa guida copre il CLI del terminale. Claude Code è disponibile anche sul [web](https://claude.ai/code), come [app desktop](/docs/it/desktop), in [VS Code](/docs/it/vs-code) e [IDE JetBrains](/docs/it/jetbrains), in [Slack](/docs/it/slack) e in CI/CD con [GitHub Actions](/docs/it/github-actions) e [GitLab](/docs/it/gitlab-ci-cd). Vedi [tutte le interfacce](/docs/it/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Passaggio 1: Installa Claude Code
</h2>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
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

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

<h2 id="step-2-log-in-to-your-account">
  Passaggio 2: Accedi al tuo account
</h2>

Claude Code richiede un account per essere utilizzato. Avvia una sessione interattiva con il comando `claude` e ti verrà chiesto di effettuare l'accesso al primo utilizzo:

```bash theme={null}
claude
```

Per gli account Claude subscription o Console, segui i prompt per completare l'autenticazione nel tuo browser. Per cambiare account in seguito o effettuare nuovamente l'autenticazione, digita `/login` all'interno della sessione in esecuzione:

```text theme={null}
/login
```

Puoi accedere utilizzando uno di questi tipi di account:

* [Claude Pro, Max, Team o Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (consigliato)
* [Claude Console](https://console.anthropic.com/) (accesso API con crediti prepagati). Al primo accesso, uno spazio di lavoro "Claude Code" viene creato automaticamente nella Console per il tracciamento centralizzato dei costi.
* [Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry](/docs/it/third-party-integrations) (provider cloud aziendali)
* Un [gateway di app Claude](/docs/it/claude-apps-gateway) auto-ospitato, se la tua organizzazione ne esegue uno: il tuo amministratore pre-configura l'URL del gateway, e `/login` si apre direttamente sulla schermata **Cloud gateway** per consentire l'accesso con SSO aziendale

Una volta effettuato l'accesso, le tue credenziali vengono archiviate e non dovrai accedere di nuovo.

<h2 id="step-3-start-your-first-session">
  Passaggio 3: Avvia la tua prima sessione
</h2>

Apri il tuo terminale in qualsiasi directory del progetto e avvia Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Vedrai il prompt di Claude Code con la versione, il modello attuale e la directory di lavoro mostrati sopra. Digita `/help` per i comandi disponibili o `/resume` per continuare una conversazione precedente.

<Tip>
  Dopo aver effettuato l'accesso (Passaggio 2), le tue credenziali vengono archiviate sul tuo sistema. Scopri di più in [Gestione delle credenziali](/docs/it/authentication#credential-management).
</Tip>

<h2 id="step-4-ask-your-first-question">
  Passaggio 4: Fai la tua prima domanda
</h2>

Iniziamo con la comprensione della tua base di codice. Prova uno di questi comandi:

```text theme={null}
cosa fa questo progetto?
```

Claude analizzerà i tuoi file e fornirà un riepilogo. Puoi anche fare domande più specifiche:

```text theme={null}
quali tecnologie utilizza questo progetto?
```

```text theme={null}
dov'è il punto di ingresso principale?
```

```text theme={null}
spiega la struttura delle cartelle
```

Puoi anche chiedere a Claude informazioni sulle sue stesse capacità:

```text theme={null}
cosa può fare Claude Code?
```

```text theme={null}
come creo skill personalizzate in Claude Code?
```

```text theme={null}
Claude Code può funzionare con Docker?
```

<Note>
  Claude Code legge i file del tuo progetto secondo le necessità. Non devi aggiungere manualmente il contesto.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Passaggio 5: Fai il tuo primo cambio di codice
</h2>

Ora facciamo in modo che Claude Code faccia un po' di codifica vera. Prova un'attività semplice:

```text theme={null}
aggiungi una funzione hello world al file principale
```

Claude Code farà:

1. Trovare il file appropriato
2. Mostrarti le modifiche proposte
3. Chiedere la tua approvazione
4. Effettuare la modifica

<Note>
  Claude Code chiede sempre il permesso prima di modificare i file. Puoi approvare i singoli cambiamenti o abilitare la modalità "Accetta tutto" per una sessione.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Passaggio 6: Usa Git con Claude Code
</h2>

Claude Code rende le operazioni Git conversazionali:

```text theme={null}
quali file ho modificato?
```

```text theme={null}
esegui il commit delle mie modifiche con un messaggio descrittivo
```

Puoi anche richiedere operazioni Git più complesse:

```text theme={null}
crea un nuovo branch chiamato feature/quickstart
```

```text theme={null}
mostrami gli ultimi 5 commit
```

```text theme={null}
aiutami a risolvere i conflitti di merge
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Passaggio 7: Correggi un bug o aggiungi una funzionalità
</h2>

Claude è abile nel debug e nell'implementazione di funzionalità.

Descrivi quello che vuoi in linguaggio naturale:

```text theme={null}
aggiungi la convalida dell'input al modulo di registrazione dell'utente
```

O correggi i problemi esistenti:

```text theme={null}
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

```text theme={null}
refactorizza il modulo di autenticazione per utilizzare async/await invece di callback
```

**Scrivi test**

```text theme={null}
scrivi unit test per le funzioni della calcolatrice
```

**Aggiorna la documentazione**

```text theme={null}
aggiorna il README con le istruzioni di installazione
```

**Revisione del codice**

```text theme={null}
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
| `claude "task"`     | Esegui un'attività una tantum                                  | `claude "fix the build error"`      |
| `claude -p "query"` | Esegui una query una tantum, quindi esci                       | `claude -p "explain this function"` |
| `claude -c`         | Continua la conversazione più recente nella directory corrente | `claude -c`                         |
| `claude -r`         | Riprendi una conversazione precedente                          | `claude -r`                         |

**Comandi di sessione**

| Comando          | Cosa fa                                    | Esempio  |
| ---------------- | ------------------------------------------ | -------- |
| `/clear`         | Cancella la cronologia della conversazione | `/clear` |
| `/help`          | Mostra i comandi disponibili               | `/help`  |
| `/exit` o Ctrl+D | Esci da Claude Code                        | `/exit`  |

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

    ```text theme={null}
    1. crea una nuova tabella di database per i profili utente
    2. crea un endpoint API per ottenere e aggiornare i profili utente
    3. costruisci una pagina web che consenta agli utenti di visualizzare e modificare le loro informazioni
    ```
  </Accordion>

  <Accordion title="Lascia che Claude esplori prima">
    Prima di apportare modifiche, lascia che Claude comprenda il tuo codice:

    ```text theme={null}
    analizza lo schema del database
    ```

    ```text theme={null}
    costruisci una dashboard che mostra i prodotti che vengono restituiti più frequentemente dai nostri clienti nel Regno Unito
    ```
  </Accordion>

  <Accordion title="Risparmia tempo con le scorciatoie">
    * Digita `/` per vedere tutti i comandi e le skills
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
* **Community**: Unisciti al nostro [Discord](https://www.anthropic.com/discord) per suggerimenti e supporto
