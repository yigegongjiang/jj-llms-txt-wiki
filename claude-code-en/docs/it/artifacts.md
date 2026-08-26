> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Condividere l'output della sessione come artifact

> Gli artifact trasformano il lavoro di Claude Code in pagine live e interattive su claude.ai che puoi mantenere private, condividere con la tua organizzazione o pubblicare su un link pubblico.

<Note>
  Gli artifact sono disponibili sui piani Pro, Max, Team ed Enterprise e richiedono una sessione autenticata con [`/login`](/docs/it/setup#authenticate). Vedi [Disponibilità](#availability) per l'insieme completo dei requisiti.
</Note>

Un artifact è una pagina web live e interattiva che Claude Code pubblica dalla tua sessione a un URL privato su claude.ai. Lo apri in un browser e si aggiorna sul posto mentre la sessione continua. Condividilo dall'intestazione della pagina quando vuoi che qualcun altro lo veda. Ad esempio, usa un artifact per guidare un revisore attraverso una pull request con diff annotati, costruire una dashboard dai dati della sessione, o mantenere una timeline di investigazione che si riempie mentre Claude lavora.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Un artifact aperto in un browser su claude.ai/code/artifact. L'intestazione del visualizzatore mostra il titolo dell'artifact acme-funnel-fix, un pulsante Share e l'avatar dell'autore. Il menu Share è aperto con l'interruttore Always share latest version, un selettore di versione che legge Sharing version 2, un selettore di audience Everyone at Acme e un pulsante Copy link. Sotto l'intestazione, la pagina dell'artifact mostra due mockup mobili affiancati, un grafico a imbuto e una riga di schede metriche." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  When to use an artifact
</h2>

Usa un artifact quando il testo del terminale è il mezzo sbagliato per quello che Claude ha prodotto: output che è più facile da guardare e con cui interagire rispetto a leggere riga per riga. Claude costruisce la pagina da qualsiasi cosa la tua sessione possa raggiungere, inclusa la tua codebase e i dati che estrae attraverso i tuoi [strumenti connessi](/docs/it/mcp), quindi la pagina può mostrare cose che richiederebbero paragrafi per descrivere. Ad esempio, chiedi a Claude di:

* Guidare un revisore attraverso una pull request con diff annotati
* Renderizzare una dashboard dai dati che la sessione ha già estratto
* Disporre diverse opzioni di design o implementazione affiancate
* Mantenere una timeline di investigazione che si riempie mentre un'attività lunga viene eseguita
* Inviare a un collega un link invece di incollare l'output in Slack
* Pubblicare una bacheca di stato che [estrae dati freschi attraverso i connettori MCP](#pull-live-data-with-mcp-connectors) ogni volta che qualcuno la apre

Vedi [Cosa puoi costruire](#what-you-can-build) per i prompt che corrispondono a questi, e [Estrarre dati live con i connettori MCP](#pull-live-data-with-mcp-connectors) per il prompt della bacheca supportata dal connettore.

<h3 id="what-an-artifact-is-not">
  What an artifact is not
</h3>

Un artifact è una cattura del lavoro, non un'applicazione. È una singola pagina autonoma senza backend, quindi non può archiviare l'input del modulo o servire più route, e il suo unico percorso verso i dati esterni quando qualcuno la visualizza è [chiamare i connettori MCP](#pull-live-data-with-mcp-connectors). Per uno strumento interno ospitato con un backend, distribuiscilo sulla tua infrastruttura. Vedi [Vincoli della pagina](#page-constraints) per l'insieme completo dei limiti.

<h2 id="create-an-artifact">
  Create an artifact
</h2>

Claude può pubblicare un artifact da solo quando l'output si adatta a una pagina, oppure puoi chiederne uno direttamente. Per chiedere, nomina la funzione o descrivi l'output visivo che desideri in linguaggio semplice. Un buon candidato è qualsiasi cosa più facile da vedere che da leggere come testo, come un diff annotato, un grafico o un insieme di opzioni da confrontare. I prompt di seguito sono due esempi; vedi [Cosa puoi costruire](#what-you-can-build) per altri pattern.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

Claude scrive la pagina in un file HTML o Markdown nel tuo progetto, quindi la pubblica. Prima di pubblicare un nuovo artifact, Claude Code chiede il permesso; potrebbe dire qualcosa come `Claude wants to publish "Deploy failures by service" (deploy-failures.html) to a private page on claude.ai`. Ripubblicare un artifact che hai già approvato non richiede di nuovo il prompt.

Seleziona **Yes** per pubblicare. Claude stampa l'URL e il tuo browser si apre sulla nuova pagina. Premi `Ctrl+]` in qualsiasi momento per riaprire l'artifact più recente dal terminale.

Claude sceglie il titolo dell'artifact e un emoji per l'icona della scheda del browser. Entrambi appaiono nella tua [galleria di artifact](#share-an-artifact) su claude.ai e nei link condivisi, quindi chiedi a Claude di usare un titolo o un'icona specifica se ne vuoi uno.

Per impedire al browser di aprirsi automaticamente quando viene pubblicato un nuovo artifact, imposta `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` nel tuo ambiente.

Se Claude risponde che non può pubblicare, o scrive un file HTML locale senza un link, lo strumento non è abilitato per la tua sessione. Controlla i requisiti di [Disponibilità](#availability).

<h2 id="update-an-artifact">
  Update an artifact
</h2>

Chiedi a Claude di revisionare la pagina, o lascia che un'attività di lunga durata ripubblichi mentre fa progressi. Claude modifica il file sottostante e pubblica di nuovo allo stesso URL.

```text wrap theme={null}
Add a per-region breakdown below the summary chart and republish.
```

Chiunque abbia la pagina aperta vede l'aggiornamento sul posto. Ogni pubblicazione diventa una versione, e dal controllo **Share** nell'intestazione della pagina puoi scegliere quale versione vedono i visualizzatori.

Per aggiornare un artifact da una sessione diversa, dai a Claude l'URL dell'artifact e chiedigli di revisionarlo. Senza l'URL, una nuova sessione crea sempre un nuovo artifact piuttosto che aggiornarne uno esistente.

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="share-an-artifact">
  Condividere un artefatto
</h2>

Un nuovo artefatto è visibile solo a voi. Per condividerlo, aprite l'artefatto nel vostro browser e utilizzate il controllo **Share** nell'intestazione della pagina. L'intestazione vi nomina come autore dell'artefatto, quindi chiunque lo condividiate può vedere chi ha pubblicato la pagina. Inoltre, si collega alla vostra galleria su [claude.ai/code/artifacts](https://claude.ai/code/artifacts), che elenca ogni artefatto che avete creato.

Chi potete condividere dipende dal vostro piano:

* **All'interno della vostra organizzazione**: nei piani Team e Enterprise, concedete l'accesso a persone specifiche nella vostra organizzazione, oppure a tutti. I visualizzatori accedono a claude.ai come membri della vostra organizzazione per vedere la pagina.
* **Pubblicamente**: condividete un link che chiunque su internet può aprire, senza richiedere l'accesso a claude.ai. Nei piani Pro e Max, un link pubblico è l'unico modo per condividere un artefatto. Nei piani Team e Enterprise, la condivisione pubblica è disattivata finché un Owner non la [abilita per l'organizzazione](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Consentire a qualcuno di modificare insieme a voi
</h3>

Le persone con cui condividete sono visualizzatori per impostazione predefinita: vedono ogni versione che pubblicate ma non possono modificare la pagina. Nei piani Team e Enterprise, potete anche rendere qualcuno un editor. Nella finestra di dialogo di condivisione, aggiungete una persona e cambiate il suo ruolo da **viewer** a **editor**.

Un editor pubblica nuove versioni nello stesso modo in cui voi [aggiornate l'artefatto da un'altra sessione](#update-an-artifact): forniscono a Claude l'URL dell'artefatto nella loro sessione, e Claude estrae il contenuto attuale e ripubblica con le loro modifiche. Tutti coloro che hanno la pagina aperta vedono ogni aggiornamento in tempo reale.

<h2 id="pull-live-data-with-mcp-connectors">
  Estrarre dati live con i connettori MCP
</h2>

Un artifact può chiamare i [connettori MCP](/docs/it/mcp#use-mcp-servers-from-claude-ai) ogni volta che qualcuno lo visualizza, in modo che la pagina mostri dati attuali anziché uno snapshot della sessione che l'ha creato. Le chiamate ai connettori dagli artifact sono disponibili nei piani Pro, Max, Team ed Enterprise e richiedono Claude Code v2.1.209 o versioni successive. Nelle versioni precedenti, Claude pubblica la pagina con i dati che la sessione ha raccolto durante la creazione.

Per creare una pagina supportata da un connettore, nominate il connettore e i dati desiderati nel vostro prompt:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude dichiara quali connettori la pagina può chiamare come parte della pubblicazione, e la pagina non può chiamare connettori al di fuori di tale dichiarazione. Solo i connettori dal vostro account claude.ai si qualificano: Claude li nomina nella dichiarazione, e quando qualcuno visualizza la pagina, ogni chiamata [viene eseguita attraverso la connessione dell'account che visualizza](#how-connector-calls-work-for-viewers) a quel connettore. I server MCP locali che configurate in Claude Code, come i server da `.mcp.json`, possono fornire dati mentre Claude crea la pagina, ma la pagina pubblicata non può chiamarli.

La pagina recupera i dati quando si carica e può aggiornarsi a intervalli regolari o quando un visualizzatore utilizza un controllo di aggiornamento sulla pagina. Le risposte vengono memorizzate nella cache del browser del visualizzatore, quindi una pagina riaperta viene renderizzata dalle risposte memorizzate immediatamente, quindi si aggiorna con risultati freschi.

<h3 id="how-connector-calls-work-for-viewers">
  Come funzionano le chiamate ai connettori per i visualizzatori
</h3>

Quando una pagina pubblicata chiama un connettore, la chiamata utilizza l'account della persona che visualizza la pagina, non l'account della persona che l'ha pubblicata:

* **Ogni visualizzatore utilizza i propri connettori**: le chiamate passano attraverso gli strumenti connessi dell'account che visualizza, quindi due persone che aprono lo stesso dashboard possono vedere dati diversi a seconda di ciò a cui i loro account possono accedere. La pagina non vede mai le credenziali di nessuno; claude.ai effettua le chiamate per conto della pagina.
* **I visualizzatori approvano l'accesso per primo**: claude.ai chiede a ogni visualizzatore il permesso prima della prima chiamata al connettore della pagina. Un visualizzatore che rifiuta, o che non ha connesso un connettore che la pagina utilizza, vede comunque la pagina senza le sue sezioni live.
* **Le azioni utilizzano anche l'account del visualizzatore**: una pagina può offrire controlli che invocano strumenti connettore con effetti collaterali, come pubblicare un messaggio o aggiornare un problema. L'azione viene eseguita attraverso l'account di chiunque selezioni il controllo.

Quando pianificate di condividere una pagina supportata da un connettore, chiedete a Claude di includere un messaggio di fallback in ogni sezione live che nomini il connettore di cui ha bisogno. Un visualizzatore che manca della connessione vede quindi cosa connettere invece di una sezione vuota.

Un artifact che chiama connettori non può essere condiviso a un link pubblico su nessun piano. Nei piani Team ed Enterprise, potete mantenerlo privato o [condividerlo all'interno della vostra organizzazione](#share-an-artifact). Nei piani Pro e Max, dove un link pubblico è l'unico modo per condividere, un artifact supportato da connettore rimane privato per voi.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  La pagina non mostra dati live per un visualizzatore
</h3>

Quando una pagina supportata da un connettore viene renderizzata ma le sue sezioni live rimangono vuote per qualcuno con cui l'avete condivisa, esaminate queste cause:

* **Il visualizzatore non ha connesso il connettore**: i connettori sono per account, quindi ogni visualizzatore ha bisogno della propria connessione a ogni connettore che la pagina chiama. Possono aggiungerne uno in **Impostazioni > Connettori** su claude.ai, quindi ricaricare la pagina.
* **Il visualizzatore ha rifiutato la richiesta di permesso**: un rifiuto dura per il resto di quel caricamento della pagina. Ricaricare la pagina riporta la richiesta di permesso.
* **Le chiamate ai connettori sono disattivate per l'organizzazione**: un Proprietario controlla l'interruttore [**Abilita connettori artifact**](#control-connector-calls-from-artifacts) nelle impostazioni di amministrazione.

<h2 id="what-you-can-build">
  What you can build
</h2>

Un artifact è una singola pagina HTML, quindi qualsiasi cosa tu possa esprimere in HTML, CSS e JavaScript inline è nell'ambito. I pattern di seguito si presentano più spesso.

<h3 id="walk-through-a-change">
  Walk through a change
</h3>

Chiedi una pagina che renderizzi un diff o un cambiamento di design con annotazioni accanto alle righe rilevanti, in modo che i revisori possano leggere il tuo ragionamento accanto al codice invece di ricostruirlo da una descrizione.

```text wrap theme={null}
Make an artifact that walks through this PR. Render the diff with margin annotations and color-code findings by severity.
```

<h3 id="compare-alternatives">
  Compare alternatives
</h3>

Chiedi diverse varianti su una pagina in modo da poter valutarle l'una contro l'altra. Questo funziona per layout, copy, forme API o piani di implementazione.

```text wrap theme={null}
Make an artifact with four distinctly different layouts for the settings panel. Vary density and grouping, and lay them out as a grid with a one-line tradeoff under each.
```

<h3 id="tune-with-interactive-controls">
  Tune with interactive controls
</h3>

Chiedi slider, interruttori o campi di input associati a quello che stai regolando, in modo da poter esplorare i valori direttamente invece di descriverli.

```text wrap theme={null}
Build an artifact with sliders for the easing curve, duration, and delay so I can try values on this transition. Show the animation live as I move them.
```

<h3 id="bring-the-result-back-to-your-session">
  Bring the result back to your session
</h3>

Un artifact può agire come un editor leggero per una decisione che poi restituisci a Claude. Chiedi un controllo di esportazione che produca testo che puoi incollare nel terminale, in modo che il risultato dell'interazione con la pagina fluisca di nuovo nella sessione invece di rimanere sulla pagina.

```text wrap theme={null}
Make a triage board artifact with each open issue as a draggable card across Now, Next, Later, and Cut columns. Add a "Copy as prompt" button that gives me the final ordering to paste back here.
```

<h3 id="track-work-in-progress">
  Track work in progress
</h3>

Chiedi a Claude di mantenere un artifact aggiornato mentre un'attività lunga viene eseguita, in modo che chiunque abbia il link possa seguire senza leggere il terminale.

```text wrap theme={null}
Turn this migration plan into a checklist artifact. Check items off as you complete them and add a note for anything you skip.
```

<h2 id="improve-the-visual-design">
  Migliora la progettazione visiva
</h2>

A partire da Claude Code v2.1.183, Claude applica una skill di design integrata quando costruisce un artifact, quindi le pagine ottengono una tavolozza deliberata, tipografia e layout senza prompt extra. Quella skill cerca anche un sistema di design esistente nel tuo progetto prima di scegliere il suo. Per mantenere gli artifact coerenti con il branding del tuo prodotto, registra i tuoi token di design dove Claude può trovarli, come il [CLAUDE.md](/docs/it/memory) del progetto o un file di tema nel tuo repository:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude tratta il tuo sistema di design come priorità più alta rispetto alle sue scelte, e il tuo prompt come priorità più alta rispetto a entrambi. L'intestazione e il formato sopra sono un esempio; qualsiasi elenco chiaro di colori, font e spaziatura funziona.

<h2 id="page-constraints">
  Vincoli della pagina
</h2>

Ogni artifact è una singola pagina autonoma. Claude Code avvolge il file che pubblichi in una shell di documento HTML e lo serve secondo una rigorosa Content Security Policy (CSP), che modella quello che la pagina può fare.

| Constraint           | Effect                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| No external requests | La CSP blocca script, fogli di stile, font e immagini caricate da qualsiasi altro host, insieme a `fetch`, XHR e chiamate WebSocket. Claude incorpora CSS e JavaScript e incorpora immagini come data URI in modo che la pagina si renderizzi senza alcuna richiesta esterna. [Le chiamate Connector](#pull-live-data-with-mcp-connectors) sono l'eccezione: la pagina le passa a claude.ai, che effettua la chiamata di rete stessa. |
| No backend           | Un artifact è una pagina statica. Non può archiviare dati inviati tramite un modulo o autenticare i visualizzatori stesso. L'unico modo per recuperare dati quando qualcuno lo visualizza è [chiamare i connector MCP](#pull-live-data-with-mcp-connectors), non un'API propria.                                                                                                                                                      |
| Single page          | I link relativi non si risolvono, perché nulla viene distribuito insieme alla pagina. Per contenuti multi-sezione, Claude utilizza ancore in-page piuttosto che file separati.                                                                                                                                                                                                                                                        |
| Source file types    | Il file pubblicato deve essere `.html`, `.htm` o `.md`. I file Markdown si renderizzano come HTML stilizzato.                                                                                                                                                                                                                                                                                                                         |
| Rendered size        | La pagina renderizzata deve essere 16 MiB o più piccola. Le immagini incorporate di grandi dimensioni sono la causa usuale quando una pubblicazione fallisce per dimensione.                                                                                                                                                                                                                                                          |

Generare un artifact utilizza token di output come qualsiasi altra risposta, e una pagina stilizzata è più intensiva di token rispetto allo stesso contenuto come testo di terminale. CSS inline, JavaScript per controlli interattivi e soprattutto immagini incorporate come data URI sono i principali contributori. Per ridurre il costo dei token di un artifact:

* Preferisci SVG, o HTML e CSS, per i diagrammi rispetto alle immagini raster incorporate
* Ometti l'interattività che non ti serve
* Fai in modo che la pagina riassuma grandi set di dati piuttosto che incorporarli completamente

<h2 id="availability">
  Availability
</h2>

Gli artifact richiedono ogni condizione di seguito. Quando una non è soddisfatta, Claude scrive un file HTML locale o dice che non può pubblicare.

| Requirement         | Available when                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan                | Pro, Max, Team o Enterprise. Sui piani Pro e Max, gli artifact sono privati per te fino a quando non li condividi, e non si applica alcuna gestione amministrativa. Sui piani Team, gli artifact sono abilitati per impostazione predefinita. Sui piani Enterprise, un Owner [li abilita](#manage-artifacts-for-your-organization) nelle impostazioni admin di claude.ai.                                                                                                                          |
| Authentication      | La sessione è supportata da un account claude.ai: accedi con `/login` nell'app CLI o desktop. Le sessioni Claude Tag sono autenticate attraverso l'identità dell'agente, quindi non è necessario alcun passaggio. Le sessioni che utilizzano una chiave API, [gateway token](/docs/it/llm-gateway) o credenziale del provider cloud non possono pubblicare.                                                                                                                                             |
| Model provider      | Anthropic API. Non disponibile su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) o [Microsoft Foundry](/docs/it/microsoft-foundry).                                                                                                                                                                                                                                                                                                                        |
| Organization policy | Le chiavi di crittografia gestite dal cliente (CMEK), HIPAA e [Zero Data Retention](/docs/it/zero-data-retention) non sono abilitate per l'organizzazione.                                                                                                                                                                                                                                                                                                                                              |
| Surface             | Claude Code CLI versione 2.1.183 o successiva, o l'app desktop Claude versione 1.13576.0 o successiva. Le sessioni [Claude Tag](https://claude.com/docs/claude-tag/overview) possono anche pubblicare artifact quando sia Claude Tag che gli artifact sono abilitati per l'organizzazione. Disabilitato per impostazione predefinita in contesti [Agent SDK](/docs/it/agent-sdk/overview), GitHub Action e MCP-server, e quando [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars) è impostato. |

<h2 id="disable-artifacts">
  Disable artifacts
</h2>

Per disattivare gli artifact per le tue sessioni indipendentemente dall'impostazione della tua organizzazione, usa uno qualsiasi di:

| Method                               | Setting                                  |
| :----------------------------------- | :--------------------------------------- |
| [Settings file](/docs/it/settings)        | `"disableArtifact": true`                |
| [Environment variable](/docs/it/env-vars) | `CLAUDE_CODE_DISABLE_ARTIFACT=1`         |
| [Permission rule](/docs/it/permissions)   | Aggiungi `Artifact` a `permissions.deny` |

<h2 id="manage-artifacts-for-your-organization">
  Manage artifacts for your organization
</h2>

I proprietari sui piani Team e Enterprise controllano gli artifact dalle [impostazioni admin di claude.ai](https://claude.ai/admin-settings/claude-code). Il contenuto dell'artifact è archiviato su infrastruttura gestita da Anthropic ed è visibile solo ai membri autenticati dell'organizzazione che pubblica, a meno che l'artifact non sia [condiviso pubblicamente](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Enable or disable artifacts
</h3>

Per abilitare o disabilitare gli artifact per l'intera organizzazione, vai a **Settings > Claude Code > Capabilities** e usa l'interruttore **Artifacts**. Sui piani Enterprise con controllo dell'accesso basato su ruoli, puoi inoltre limitare gli artifact a ruoli specifici: vai a **Settings > Roles**, modifica un ruolo e imposta l'autorizzazione **Artifacts** nel gruppo **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Control connector calls from artifacts
</h3>

Le [chiamate ai connettori dagli artifact](#pull-live-data-with-mcp-connectors) hanno il loro interruttore dedicato, separato dall'interruttore **Artifacts** che attiva o disattiva gli artifact. Vai a [**Settings > Capabilities**](https://claude.ai/admin-settings/capabilities) e usa l'interruttore **Enable artifact connectors**. Lo stesso interruttore governa le chiamate ai connettori dagli artifact creati nelle conversazioni di claude.ai, motivo per cui si trova sotto **Settings > Capabilities** piuttosto che sotto **Settings > Claude Code**.

<h3 id="control-public-sharing">
  Control public sharing
</h3>

La condivisione pubblica è disattivata per impostazione predefinita sui piani Team e Enterprise, quindi i membri possono condividere gli artifact solo all'interno dell'organizzazione finché un proprietario non la attiva. Per consentire ai membri di pubblicare artifact su link pubblici che chiunque può visualizzare senza accedere, vai a **Settings > Claude Code > Capabilities** e attiva **External sharing** sotto l'interruttore **Artifacts**. Disattivarla di nuovo blocca l'accesso tramite link pubblici esistenti senza modificare il pubblico di ogni artifact; l'accesso riprende se lo riabiliti.

<h3 id="set-a-retention-policy">
  Set a retention policy
</h3>

Per impostare quanto tempo gli artifact vengono conservati prima dell'eliminazione automatica, vai a **Settings > Data & privacy controls**. Puoi impostare periodi di conservazione separati per gli artifact che sono ancora privati al loro autore e gli artifact che sono stati condivisi.

<h3 id="review-the-audit-log">
  Review the audit log
</h3>

La pubblicazione, la condivisione e l'eliminazione di un artifact appaiono ciascuna nel registro di audit della tua organizzazione sotto i tipi di evento `claude_artifact_*`, la stessa famiglia utilizzata per gli artifact creati nelle conversazioni di claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Allowlist the viewer domain
</h3>

Il visualizzatore su claude.ai carica ogni artifact da un'origine sandbox `*.claudeusercontent.com`. Se la tua organizzazione limita l'accesso alla rete in uscita, aggiungi quel dominio alla tua allowlist insieme a `claude.ai`. Vedi [Requisiti di accesso alla rete](/docs/it/network-config#network-access-requirements) per l'elenco completo.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  List and delete artifacts with the Compliance API
</h3>

La [Compliance API](https://docs.claude.com/en/api/compliance) fornisce endpoint per elencare gli artifact di un'organizzazione, recuperare il contenuto di una versione specifica e eliminare un artifact:

| Method   | Endpoint                                                            |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

Per gli schemi di richiesta e risposta, vedi il [riferimento della Compliance API](https://docs.claude.com/en/api/compliance/code/artifacts).

<h2 id="related-resources">
  Risorse correlate
</h2>

* Sfoglia [pattern di prompting e workflow](/docs/it/prompt-library) che si abbinano agli artifact
* Trasforma un prompt di artifact che riutilizzi in una [skill](/docs/it/skills) in modo da poterlo invocare come comando
* [Connetti server MCP](/docs/it/mcp) in modo che Claude possa estrarre dati in un artifact mentre lo crea
