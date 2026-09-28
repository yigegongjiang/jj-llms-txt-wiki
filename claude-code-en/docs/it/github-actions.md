> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Esegui Claude Code nei flussi di lavoro di GitHub Actions per rispondere alle menzioni @claude, automatizzare attività e trasformare issue in pull request

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) è un'azione GitHub che esegue Claude Code all'interno dei flussi di lavoro del tuo repository. Menziona `@claude` in un commento di pull request o issue per fare in modo che Claude analizzi il codice, implementi modifiche e spinga commit. Puoi anche fornire al Claude Code GitHub Action un prompt per eseguirlo automaticamente su qualsiasi evento GitHub. Usalo per trasformare issue in pull request, correggere bug da un commento o automatizzare attività ricorrenti.

Diversi prodotti condividono il nome Claude Code. Questa pagina copre l'integrazione del flusso di lavoro `claude-code-action`, che configuri con file di flusso di lavoro nel tuo repository. Per i prodotti correlati, vedi:

* [Code Review](/docs/it/code-review): revisione automatica su ogni pull request, senza scrivere un flusso di lavoro
* [Claude Code nel cloud](/docs/it/claude-code-on-the-web): sessioni di Claude Code che vengono eseguite su infrastruttura cloud invece che sulla tua macchina
* [Claude Agent SDK](/docs/it/agent-sdk/overview): automazione personalizzata al di fuori di GitHub Actions. Il Claude Code GitHub Action è costruito sull'SDK
* [GitHub Enterprise Server](/docs/it/github-enterprise-server): Claude Code con GitHub auto-ospitato

<h2 id="setup">
  Setup
</h2>

Puoi configurare il Claude Code GitHub Action in uno di due modi:

* **Setup rapido**: esegui `/install-github-app` da Claude Code. Claude Code installa l'app GitHub, aggiunge il tuo secret di autenticazione e prepara la pull request del flusso di lavoro per te
* **Setup manuale**: installa l'app, aggiungi il secret e copia il file del flusso di lavoro nel tuo repository da solo. Usa questo percorso quando non esegui Claude Code localmente, quando il comando fallisce o quando vuoi il controllo completo dei file di flusso di lavoro

Per entrambi i percorsi, hai bisogno dell'accesso amministratore al repository.

<h3 id="quick-setup">
  Setup rapido
</h3>

`/install-github-app` funziona solo con repository github.com. Se il git remote del tuo repository è su gitlab.com o bitbucket.org, il comando stampa un avviso ed esce invece di avviare la configurazione. Per eseguire Claude Code da pipeline GitLab, vedi [Claude Code GitLab CI/CD](/docs/it/gitlab-ci-cd).

Prima di iniziare, installa la [GitHub CLI](https://cli.github.com) e autenticala con `gh auth login`. Claude Code la cerca e ti avverte se manca.

Apri `claude` nel repository che vuoi connettere, esegui `/install-github-app` e segui i prompt. Claude Code installa l'app GitHub di Claude, quindi configura un secret di autenticazione per i flussi di lavoro:

* Se Claude Code ha già una chiave API, la riutilizza e offre di mantenere il secret `ANTHROPIC_API_KEY` esistente del repository se uno è già impostato
* Altrimenti, scegli tra creare un token di lunga durata con la tua sottoscrizione Claude e incollare una chiave API

Claude Code salva le credenziali come secret del repository, denominato `ANTHROPIC_API_KEY` per una chiave API o `CLAUDE_CODE_OAUTH_TOKEN` per un token di sottoscrizione.

Claude Code quindi spinge un ramo con i file di flusso di lavoro che selezioni, già impostati per usare quel secret, e apre GitHub nel tuo browser con una pull request pronta per essere creata. Crea e unisci quella pull request, e `@claude` funziona nel repository.

Se selezioni il flusso di lavoro di revisione, Claude pubblica ogni revisione sulla pull request stessa, come commento inline su ogni problema che trova o come un commento di riepilogo quando non ne trova nessuno. Claude salta alcune pull request, come le bozze. L'[esempio del flusso di lavoro di revisione](#run-a-skill) usa la stessa skill e le elenca. Prima della v2.1.229, Claude scriveva la sua revisione solo nel log di esecuzione del flusso di lavoro.

Per aggiornare un flusso di lavoro di revisione che una versione precedente ha generato, fai uno dei seguenti:

* Esegui `/install-github-app` di nuovo. Quando il repository ha già un `claude.yml`, seleziona **Update workflow file with latest version**. Claude Code spinge copie fresche dei file di flusso di lavoro a un nuovo ramo e apre la pull request, come un primo install.
* Aggiungi l'argomento `--comment` e la riga `claude_args` dall'[esempio del flusso di lavoro di revisione](#run-a-skill) al file archiviato da solo, che mantiene qualsiasi altra modifica che hai fatto ad esso.

Dopo aver installato l'app GitHub, Claude Code chiede se continuare con la configurazione di GitHub Actions. Scegli **Skip for now** per fermarti con solo l'app GitHub installata. Esegui `/install-github-app` di nuovo in seguito per completare i passaggi del flusso di lavoro e del secret.

<Note>
  * Quando installi l'app GitHub, le concedi diverse autorizzazioni. Vedi [Autorizzazioni dell'app GitHub](#github-app-permissions) per l'insieme completo
  * Lo setup rapido funziona con l'API Claude e le sottoscrizioni Claude. Se usi Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, vedi [Usa Claude Code GitHub Actions con provider cloud](/docs/it/github-actions-cloud-providers)
</Note>

<h3 id="manual-setup">
  Setup manuale
</h3>

Per configurare il Claude Code GitHub Action senza eseguire `/install-github-app`, installa l'app, aggiungi un secret e copia un file di flusso di lavoro da solo:

<Steps>
  <Step title="Installa l'app GitHub di Claude">
    Installa l'[app GitHub di Claude](https://github.com/apps/claude) nel tuo repository. Il Claude Code GitHub Action si basa su tre delle autorizzazioni dell'app:

    * **Contents**: lettura e scrittura, in modo che Claude possa modificare i file del repository
    * **Issues**: lettura e scrittura, in modo che Claude possa rispondere ai problemi
    * **Pull requests**: lettura e scrittura, in modo che Claude possa creare PR e spingere modifiche

    Durante l'installazione, concedi anche autorizzazioni che altre funzionalità Claude usano. Vedi [Autorizzazioni dell'app GitHub](#github-app-permissions) per l'insieme completo.
  </Step>

  <Step title="Aggiungi un secret di autenticazione">
    Aggiungi uno dei seguenti secret al tuo repository, a seconda di come ti autentichi. Vedi la guida di GitHub su [utilizzo dei secret in GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    * `ANTHROPIC_API_KEY`: una chiave API Claude dalla [Claude Console](https://platform.claude.com)
    * `CLAUDE_CODE_OAUTH_TOKEN`: un token OAuth che si autentica con la tua sottoscrizione Claude, disponibile su piani Pro, Max, Team ed Enterprise. Generane uno eseguendo `claude setup-token` localmente. Vedi [Genera un token di lunga durata](/docs/it/authentication#generate-a-long-lived-token)

    Nei file di flusso di lavoro, passa il secret all'input corrispondente: `anthropic_api_key` per una chiave API, o `claude_code_oauth_token` per un token OAuth.
  </Step>

  <Step title="Copia il file del flusso di lavoro">
    Copia [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) nella directory `.github/workflows/` del tuo repository. Il file è un flusso di lavoro funzionante, non solo un esempio. Come archiviato, Claude risponde ogni volta che qualcuno menziona `@claude` in un issue o pull request, autenticandosi con il secret `ANTHROPIC_API_KEY`. Se hai aggiunto `CLAUDE_CODE_OAUTH_TOKEN` invece, cambia la riga `anthropic_api_key` del flusso di lavoro in `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.
  </Step>
</Steps>

<Tip>
  Dopo la configurazione, testa il Claude Code GitHub Action taggando `@claude` in un commento di issue o PR.
</Tip>

<h3 id="set-up-for-an-organization">
  Configura per un'organizzazione
</h3>

Con lo setup rapido o lo setup manuale, configuri un repository alla volta. Per implementare il Claude Code GitHub Action in un'organizzazione:

* Installa l'[app GitHub di Claude](https://github.com/apps/claude) una volta a livello di organizzazione, scegliendo tutti i repository o un elenco selezionato
* Archivia il secret di autenticazione come secret di Actions a livello di organizzazione in modo che ogni repository non abbia bisogno della sua copia
* Aggiungi il file di flusso di lavoro a ogni repository che dovrebbe eseguire il Claude Code GitHub Action, o definisci il job una volta come [flusso di lavoro riutilizzabile](https://docs.github.com/en/actions/using-workflows/reusing-workflows) che ogni repository chiama

Per un secret condiviso tra repository, autenticati con una chiave API dalla [Claude Console](https://platform.claude.com) piuttosto che un token OAuth, poiché un token OAuth è legato alla sottoscrizione della persona che ha eseguito `claude setup-token`.

Per evitare di archiviare un secret di lunga durata del tutto, autenticati tramite workload identity federation, dove il Claude Code GitHub Action scambia il token GitHub OpenID Connect (OIDC) del flusso di lavoro per l'accesso all'API Claude tramite un account di servizio della Claude Console. Imposta questi input:

* `anthropic_federation_rule_id`: l'ID della regola di federazione, `fdrl_...`
* `anthropic_organization_id`: il tuo ID organizzazione Anthropic
* `anthropic_service_account_id`: l'ID dell'account di servizio, `svac_...`. Opzionale, poiché la regola di federazione che crei nella Console già mira a un account di servizio
* `anthropic_workspace_id`: l'ID dell'area di lavoro, `wrkspc_...`. Opzionale quando la regola di federazione mira a un'unica area di lavoro

Concedi al flusso di lavoro il permesso `id-token: write`, che il Claude Code GitHub Action necessita per lo scambio di federazione anche quando passi il tuo `github_token`. Vedi la [guida di configurazione del Claude Code GitHub Action](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) per la configurazione lato Console.

Per domande sulla gestione dei dati e la conservazione in una revisione della sicurezza, vedi [utilizzo dei dati](/docs/it/data-usage) e [sicurezza](/docs/it/security).

<h3 id="uninstall">
  Disinstalla
</h3>

Per rimuovere il Claude Code GitHub Action, annulla ogni pezzo della configurazione che si applica alla tua installazione:

* **File di flusso di lavoro**: elimina i flussi di lavoro che usano `anthropics/claude-code-action` da `.github/workflows/`. Se hai usato lo setup rapido, cerca `claude.yml` e, se hai selezionato il flusso di lavoro di revisione, `claude-code-review.yml`. Con i flussi di lavoro eliminati, il Claude Code GitHub Action non viene più eseguito
* **Secret**: elimina il secret `ANTHROPIC_API_KEY` o `CLAUDE_CODE_OAUTH_TOKEN` dal repository e dai secret di Actions a livello di organizzazione se l'hai [condiviso tra repository](#set-up-for-an-organization). Se elimini un secret, le credenziali che conteneva rimangono valide. Per ritirare completamente una chiave API, elimina anche la chiave nella [Claude Console](https://platform.claude.com)
* **App GitHub**: disinstalla l'app GitHub di Claude nelle impostazioni del tuo repository o organizzazione sotto GitHub Apps, ma solo se non la usi per un'altra funzionalità Claude, come Code Review o web auto-fix

Se hai configurato un [provider cloud](/docs/it/github-actions-cloud-providers), elimina anche i secret del provider, come `AWS_ROLE_TO_ASSUME`, i secret `GCP_*` o i secret `AZURE_*`, e disinstalla l'app GitHub personalizzata insieme ai suoi secret `APP_ID` e `APP_PRIVATE_KEY`.

<h3 id="github-app-permissions">
  Autorizzazioni dell'app GitHub
</h3>

L'[app GitHub di Claude](https://github.com/apps/claude) è condivisa da ogni funzionalità Claude che si integra con GitHub, incluso il Claude Code GitHub Action, [Code Review](/docs/it/code-review) e [auto-fix per pull request](/docs/it/claude-code-on-the-web#auto-fix-pull-requests) in sessioni cloud. Un'app GitHub ha un singolo set di autorizzazioni che copre tutte le sue funzionalità, quindi l'insieme include alcune autorizzazioni che il Claude Code GitHub Action non usa.

Quando installi l'app, concedi le seguenti autorizzazioni:

| Autorizzazione   | Accesso             |
| ---------------- | ------------------- |
| Actions          | Lettura e scrittura |
| Checks           | Lettura e scrittura |
| Contents         | Lettura e scrittura |
| Discussions      | Lettura e scrittura |
| Issues           | Lettura e scrittura |
| Members          | Lettura             |
| Metadata         | Lettura             |
| Pull requests    | Lettura e scrittura |
| Repository hooks | Lettura e scrittura |
| Statuses         | Lettura             |
| Workflows        | Lettura e scrittura |

L'insieme di autorizzazioni può anche cambiare prima delle funzionalità che lo usano. Quando l'app richiede un'autorizzazione che non aveva prima, GitHub chiede al proprietario dell'account di approvarla, a un proprietario dell'organizzazione per un'installazione dell'organizzazione, e l'installazione mantiene le sue vecchie autorizzazioni finché non lo fanno. Ad esempio, quando l'accesso di Actions cambia da lettura a scrittura, l'app può rieseguire i flussi di lavoro piuttosto che solo visualizzare esecuzioni e log, quindi GitHub chiede al proprietario di approvare il cambiamento.

Quando installi l'app, accetti il suo set di autorizzazioni completo. GitHub non ti consente di accettare un sottoinsieme. Se la tua organizzazione richiede solo le autorizzazioni che il Claude Code GitHub Action usa, crea un'app GitHub personalizzata con Contents, Issues e Pull requests invece, seguendo la [guida di configurazione del Claude Code GitHub Action](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md). Un'app personalizzata copre solo il Claude Code GitHub Action. Code Review e web auto-fix richiedono ancora l'app ufficiale.

Per dettagli su come il Claude Code GitHub Action limita ciò che Claude può fare con queste autorizzazioni, vedi la [documentazione sulla sicurezza](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h2 id="interactive-and-automation-modes">
  Modalità interattiva e automazione
</h2>

Il Claude Code GitHub Action rileva come eseguire dalla tua configurazione del flusso di lavoro:

* **Modalità interattiva**: quando il flusso di lavoro non fornisce alcun input `prompt`, Claude attende la frase trigger, `@claude` per impostazione predefinita, in un commento di issue o pull request, in una revisione di pull request o nel corpo o titolo di un issue appena aperto, quindi risponde a quella richiesta. L'avanzamento e i risultati appaiono come commento sull'issue o PR che ha attivato.
* **Modalità automazione**: quando il flusso di lavoro fornisce un input `prompt`, Claude viene eseguito senza attendere una menzione, soggetto solo ai [controlli su chi può attivare le esecuzioni](#who-can-trigger-runs). Per impostazione predefinita, i risultati appaiono nel log di esecuzione del flusso di lavoro piuttosto che in un commento. Claude può pubblicare sull'issue o pull request quando il prompt lo dirige e ha uno strumento che può pubblicare, come nell'[esempio di code-review](#run-a-skill).

<h3 id="who-can-trigger-runs">
  Chi può attivare le esecuzioni
</h3>

In entrambe le modalità, il Claude Code GitHub Action esegue due controlli sull'attore che attiva prima che Claude inizi, e l'esecuzione fallisce quando uno dei due controlli lo rifiuta:

* **Accesso in scrittura**: su eventi di issue e pull request, l'utente che attiva deve avere accesso in scrittura al repository. Per consentire utenti specifici senza accesso in scrittura, imposta `allowed_non_write_users` e passa il tuo input `github_token`. Gli eventi che nessun utente crea, come un trigger `schedule`, saltano questo controllo.
* **Attore umano**: su ogni evento, il Claude Code GitHub Action rifiuta un attore bot a meno che non lo elenchi in `allowed_bots`, che impedisce ai bot di attivare Claude in un ciclo. Questo controllo si applica anche alle esecuzioni programmate, che GitHub attribuisce a un utente del repository, di solito quello che ha modificato per ultimo il programma `cron` del flusso di lavoro. Se quell'utente è un bot, elencalo in `allowed_bots`.

<h2 id="example-use-cases">
  Casi d'uso di esempio
</h2>

La [directory degli esempi](https://github.com/anthropics/claude-code-action/tree/main/examples) contiene flussi di lavoro pronti all'uso per diversi scenari.

Gli esempi in questa pagina mostrano l'autenticazione con chiave API. Se ti autentichi con una sottoscrizione Claude, sostituisci la riga `anthropic_api_key` in qualsiasi esempio con `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.

<h3 id="respond-to-claude-mentions">
  Rispondi alle menzioni @claude
</h3>

Questo flusso di lavoro esegue il Claude Code GitHub Action in modalità interattiva, in modo che Claude risponda ogni volta che qualcuno menziona `@claude` in un commento di issue o PR.

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

Le parti di questo flusso di lavoro che non sono boilerplate:

* `id-token: write`: richiesto per l'autenticazione dell'app GitHub predefinita del Claude Code GitHub Action
* `actions: read`: consente a Claude di leggere i risultati CI su PR
* `actions/checkout`: fornisce a Claude una copia locale del repository su cui lavorare
* `if`: impedisce ai runner di avviarsi su commenti che non menzionano `@claude`. Il Claude Code GitHub Action controlla anche la frase trigger stessa prima di rispondere

Una volta che il flusso di lavoro è in atto, menziona `@claude` in qualsiasi commento di issue o PR con una richiesta:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude risponde in un commento sullo stesso issue o PR e lo aggiorna mentre lavora.

<h3 id="run-a-skill">
  Esegui una skill
</h3>

L'input `prompt` accetta un'invocazione di [skill](/docs/it/skills) così come testo semplice:

* Per una skill nella directory `.claude/skills/` del tuo repository, esegui `actions/checkout` prima del passo `anthropics/claude-code-action` in modo che i file della skill siano disponibili sul runner, quindi passa `/skill-name` come `prompt`.
* Per una skill inclusa in un [plugin](/docs/it/plugins/overview), installa il plugin con gli input `plugin_marketplaces` e `plugins`, quindi passa lo `/plugin-name:skill-name` con namespace come `prompt`. L'input `plugins` accetta `plugin-name@marketplace-name`, dove il nome del marketplace proviene dal manifesto del marketplace stesso piuttosto che dall'URL del suo repository.

Il seguente flusso di lavoro installa il plugin `code-review` ed esegue la sua skill quando una pull request viene aperta, aggiornata, riaperta o contrassegnata come pronta per la revisione. Esegue lo stesso plugin del flusso di lavoro di revisione dallo setup rapido. Usa un flusso di lavoro come questo quando vuoi controllare il prompt, il modello e i trigger da solo. Per revisioni automatiche senza mantenere un file di flusso di lavoro, vedi [Code Review](/docs/it/code-review). Su repository pubblici, GitHub trattiene i secret dalle esecuzioni attivate da pull request di fork, quindi la revisione viene eseguita solo su pull request da rami nello stesso repository.

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

Due righe in questo flusso di lavoro controllano dove va la revisione:

* **`--comment`**: Claude pubblica la sua revisione sulla pull request, come commento inline su ogni problema che trova o come un commento di riepilogo quando non ne trova nessuno. Senza di esso, Claude non pubblica nulla e leggi i risultati nel log di esecuzione del flusso di lavoro.
* **`claude_args`**: mantieni questa riga anche se il frontmatter `allowed-tools` della skill stessa nomina lo stesso strumento, perché il Claude Code GitHub Action avvia il server MCP che pubblica commenti inline solo quando `--allowedTools` in `claude_args` lo nomina.

Claude salta le pull request in bozza e chiuse, le pull request che giudica non aver bisogno di una revisione, come quelle automatizzate o banali, e le pull request che hanno già un commento da Claude.

<h3 id="run-on-a-schedule">
  Esegui su una pianificazione
</h3>

Con un input `prompt`, il Claude Code GitHub Action viene eseguito in modalità automazione su qualsiasi evento GitHub, inclusa una pianificazione cron. Per un prompt in testo semplice, Claude non ha accesso a shell o API GitHub finché non concedi gli strumenti che il prompt necessita, con `--allowedTools` in `claude_args` o una regola [`permissions.allow`](/docs/it/permissions#permission-rule-syntax) nell'input `settings`. Se invochi una skill invece, Claude può usare gli strumenti che il suo frontmatter [`allowed-tools`](/docs/it/skills#pre-approve-tools-for-a-skill) concede. GitHub esegue i flussi di lavoro pianificati solo dal ramo predefinito e, nei repository pubblici, disabilita la pianificazione dopo 60 giorni senza attività del repository.

Questo flusso di lavoro genera un rapporto nel log di esecuzione del flusso di lavoro alle 09:00 UTC ogni giorno. La sua riga `claude_args` [passa argomenti CLI](#pass-cli-arguments) che selezionano il modello e consentono due strumenti MCP GitHub. Claude legge commit e issue tramite l'API GitHub con quegli strumenti, quindi puoi omettere il passo di checkout:

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  Best practice
</h2>

<h3 id="define-project-standards-in-claude-md">
  Definisci gli standard del progetto in CLAUDE.md
</h3>

Crea un file `CLAUDE.md` nella radice del tuo repository per definire linee guida di stile del codice, criteri di revisione, regole specifiche del progetto e pattern preferiti. Claude segue queste linee guida quando crea PR e risponde alle richieste. Vedi la [documentazione sulla memoria](/docs/it/memory) per i dettagli.

<h3 id="protect-your-credentials">
  Proteggi le tue credenziali
</h3>

<Warning>
  Non eseguire mai il commit di chiavi API o token OAuth direttamente nel tuo repository. Archivia sempre i secret di GitHub e fai riferimento ad essi nei flussi di lavoro, ad esempio `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.
</Warning>

Concedi al flusso di lavoro solo le autorizzazioni di cui ha bisogno e rivedi le modifiche di Claude prima di unire.

Per una guida completa sulla sicurezza inclusa autorizzazioni e autenticazione, vedi la [documentazione sulla sicurezza di Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h3 id="manage-costs">
  Gestisci i costi
</h3>

Ogni esecuzione consuma due tipi di risorse:

* **Minuti di GitHub Actions**: il Claude Code GitHub Action viene eseguito su runner ospitati da GitHub, che consumano i tuoi minuti di GitHub Actions. Vedi la [documentazione sulla fatturazione di GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) per i prezzi e i limiti di minuti.
* **Token API**: ogni interazione consuma token in base alla lunghezza dei prompt e delle risposte, alla complessità dell'attività e alla dimensione della codebase. Vedi la [pagina dei prezzi di Claude](https://claude.com/platform/api) per i tassi di token attuali. Se ti autentichi con un token OAuth, le esecuzioni usano la tua sottoscrizione Claude invece della fatturazione API.

Puoi abbassare entrambi i tipi di costo fornendo a Claude un contesto più chiaro e limitando quanto lavoro ogni esecuzione può fare:

* Scrivi richieste specifiche `@claude` in modo che Claude abbia bisogno di meno turni per finire
* Usa template di issue per fornire contesto in anticipo
* Mantieni il tuo `CLAUDE.md` conciso, poiché Claude lo legge su ogni esecuzione
* Imposta `--max-turns` in `claude_args` per limitare le iterazioni
* Imposta timeout a livello di flusso di lavoro per evitare job fuori controllo
* Usa i controlli di concorrenza di GitHub per limitare le esecuzioni parallele

Per il tracciamento dell'utilizzo in tutta la tua organizzazione, vedi il [dashboard di analisi](/docs/it/analytics) e il [monitoraggio](/docs/it/monitoring-usage). Per come viene misurato e fatturato l'utilizzo, vedi [costi](/docs/it/costs).

<h2 id="use-a-cloud-provider">
  Usa un provider cloud
</h2>

Per impostazione predefinita, il Claude Code GitHub Action chiama direttamente l'API Claude con la tua chiave API o token OAuth. Per instradare l'inferenza attraverso il tuo account cloud invece, imposta l'input per il tuo provider e segui [Usa Claude Code GitHub Actions con provider cloud](/docs/it/github-actions-cloud-providers):

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Con tutti e tre i provider, ti autentichi tramite federazione di identità OIDC invece di una chiave API Claude, quindi non archivi credenziali cloud statiche nel tuo repository.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude non risponde ai comandi @claude
</h3>

* Verifica che l'app GitHub sia installata sul repository
* Controlla che i flussi di lavoro siano abilitati per il repository
* Assicurati che la tua chiave API o token OAuth sia impostato nei secret del repository
* Conferma che il commento contenga `@claude` come parola completa, non `/claude` o `@claude-bot`
* Conferma che l'utente che commenta ha accesso in scrittura al repository. Vedi [Chi può attivare le esecuzioni](#who-can-trigger-runs) per le eccezioni

<h3 id="ci-not-running-on-claude’s-commits">
  CI non in esecuzione sui commit di Claude
</h3>

* GitHub non attiva i flussi di lavoro sui commit effettuati con il `GITHUB_TOKEN` predefinito. Se passi `github_token: ${{ secrets.GITHUB_TOKEN }}` al Claude Code GitHub Action, rimuovilo in modo che si autentichi come l'app GitHub di Claude, o passa un token di app personalizzato invece
* Controlla che i trigger del flusso di lavoro CI includano gli eventi che i push di Claude producono, come `push` o `pull_request`

<h3 id="authentication-errors">
  Errori di autenticazione
</h3>

* Conferma che la chiave API o il token OAuth sia valido testando localmente con `claude` prima di eseguire il debug del flusso di lavoro
* Per Bedrock, Agent Platform e Foundry, vedi la [sezione troubleshooting](/docs/it/github-actions-cloud-providers#troubleshooting) della pagina del provider cloud

Per altre soluzioni, vedi le [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) del Claude Code GitHub Action.

<h2 id="advanced-configuration">
  Configurazione avanzata
</h2>

<h3 id="action-parameters">
  Parametri dell'action
</h3>

Questi sono gli input più comunemente usati. Ognuno corrisponde a una chiave `with:` nel passo `anthropics/claude-code-action`.

| Parametro                 | Descrizione                                                                                                                                                                       | Richiesto                                                                                                                                                                                     |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`                  | Istruzioni per Claude, come testo semplice o un'invocazione di [skill](/docs/it/skills). Quando omesso, Claude risponde alla [frase trigger](#interactive-and-automation-modes) invece | No                                                                                                                                                                                            |
| `claude_args`             | Argomenti CLI passati a Claude Code                                                                                                                                               | No                                                                                                                                                                                            |
| `anthropic_api_key`       | Chiave API Claude                                                                                                                                                                 | Per l'API Claude, a meno che non usi `claude_code_oauth_token` o [federazione di identità del carico di lavoro](#set-up-for-an-organization). Non usato per Bedrock, Agent Platform o Foundry |
| `claude_code_oauth_token` | Token OAuth per l'autenticazione con una sottoscrizione Claude, generato con `claude setup-token`                                                                                 | No                                                                                                                                                                                            |
| `github_token`            | Token per le operazioni GitHub. Quando omesso, il Claude Code GitHub Action si autentica come l'app GitHub di Claude                                                              | No                                                                                                                                                                                            |
| `plugin_marketplaces`     | Elenco separato da newline degli URL Git del marketplace dei plugin                                                                                                               | No                                                                                                                                                                                            |
| `plugins`                 | Elenco separato da newline dei nomi dei plugin da installare prima dell'esecuzione                                                                                                | No                                                                                                                                                                                            |
| `settings`                | Impostazioni di Claude Code, come stringa JSON o percorso a un file JSON di impostazioni                                                                                          | No                                                                                                                                                                                            |
| `trigger_phrase`          | Frase trigger a cui Claude risponde. Predefinito: `@claude`                                                                                                                       | No                                                                                                                                                                                            |
| `use_bedrock`             | Usa Amazon Bedrock invece dell'API Claude                                                                                                                                         | No                                                                                                                                                                                            |
| `use_vertex`              | Usa Google Cloud's Agent Platform invece dell'API Claude                                                                                                                          | No                                                                                                                                                                                            |
| `use_foundry`             | Usa Microsoft Foundry invece dell'API Claude                                                                                                                                      | No                                                                                                                                                                                            |

Per l'elenco completo degli input, vedi il [riferimento di configurazione](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs) del Claude Code GitHub Action.

<h3 id="pass-cli-arguments">
  Passa argomenti CLI
</h3>

Il parametro `claude_args` accetta qualsiasi [argomento CLI di Claude Code](/docs/it/cli-reference):

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Argomenti comuni:

* `--max-turns`: limita il numero di turni di conversazione
* `--model`: modello da usare, ad esempio `claude-sonnet-5`. Senza questo argomento, il Claude Code GitHub Action usa il [modello predefinito](/docs/it/model-config) di Claude Code
* `--mcp-config`: percorso alla [configurazione MCP](/docs/it/mcp)
* `--allowedTools`: elenco separato da virgole degli strumenti consentiti. L'alias `--allowed-tools` funziona anche
* `--debug`: abilita l'output di debug

<h2 id="upgrade-from-beta">
  Aggiorna dalla versione beta
</h2>

Se i tuoi flussi di lavoro fanno ancora riferimento a `anthropics/claude-code-action@beta`, aggiornali a v1:

1. Cambia `@beta` a `@v1` nella riga `uses`
2. Rimuovi l'input `mode`, poiché il Claude Code GitHub Action ora [rileva la modalità automaticamente](#interactive-and-automation-modes)
3. Sostituisci `direct_prompt` con `prompt`
4. Sposta le opzioni CLI come `max_turns` e `model` in `claude_args`. `custom_instructions` non ha un flag con lo stesso nome e diventa `--append-system-prompt`

Per il mapping completo degli input e gli esempi prima e dopo, vedi la [guida di migrazione](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md).

<h2 id="what’s-next">
  Cosa c'è dopo
</h2>

* [Usa Claude Code GitHub Actions con provider cloud](/docs/it/github-actions-cloud-providers): instrada l'inferenza attraverso Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry
* [Riferimento di configurazione](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs): l'elenco completo degli input dell'action
* [Directory degli esempi](https://github.com/anthropics/claude-code-action/tree/main/examples): flussi di lavoro pronti all'uso per più scenari
* [Code Review](/docs/it/code-review): revisione automatica della pull request senza mantenere un file di flusso di lavoro
