> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sicurezza e affidabilità dei plugin

> Decidi se fidarti di un plugin prima di installarlo, da ciò che un plugin può fare sulla tua macchina a come esaminarlo e rimuoverlo.

Un plugin Claude Code che installi può eseguire codice arbitrario sulla tua macchina con i tuoi privilegi utente.

Installi un plugin da un marketplace, che è il catalogo da cui Claude Code lo recupera. Alcuni nomi di marketplace sono [riservati ai marketplace di Anthropic](#marketplace-tiers), e ogni altro marketplace è di terze parti. Il nome di un marketplace ti dice chi pubblica il catalogo, non cosa fa ogni plugin in esso, quindi [esamina un plugin prima di installarlo](#review-a-plugin-before-you-install) da qualsiasi marketplace provenga.

Leggi questa pagina se stai decidendo se installare un plugin, o se esamini gli strumenti prima che il tuo team possa usarli.

<Note>
  Questi casi sono trattati su altre pagine:

  * **Modello di sicurezza di Claude Code**: vedi [Sicurezza](/docs/it/security)
  * **Limitare o richiedere plugin per un'organizzazione**: vedi [Gestisci plugin per la tua organizzazione](/docs/it/plugins/org)
  * **I plugin `security-guidance` o `claude-security`**: questa pagina non riguarda loro. Vedi [`security-guidance`](/docs/it/security-guidance) e [`claude-security`](/docs/it/claude-security)
</Note>

Inizia con [cosa può fare un plugin](#understand-what-a-plugin-can-do) e [quali marketplace sono di Anthropic](#marketplace-tiers), quindi [esamina il plugin prima di installarlo](#review-a-plugin-before-you-install).

<h2 id="understand-what-a-plugin-can-do">
  Comprendi cosa può fare un plugin
</h2>

Un plugin può contenere contenuti che eseguono codice sulla tua macchina con i tuoi privilegi utente e contenuti che entrano nel contesto di Claude come istruzioni, quindi [esamina un plugin prima di installarlo](#review-a-plugin-before-you-install). Ecco cosa può fare un plugin installato:

* **Hooks**: gli [hooks](/docs/it/hooks) di un plugin vengono eseguiti come comandi shell in punti del ciclo di vita di Claude Code, come prima o dopo una chiamata a uno strumento.
* **Server MCP e LSP**: Claude Code si connette ai [server MCP](/docs/it/mcp) che un plugin abilitato dichiara e fornisce a Claude i loro strumenti. Un server MCP stdio viene eseguito come un processo che Claude Code avvia sulla tua macchina. Claude Code avvia anche i language server che il plugin dichiara.
* **Directory `bin/`**: Claude Code aggiunge la directory `bin/` di ogni plugin abilitato al `PATH` della shell dello strumento Bash, quindi i comandi Bash di Claude possono eseguire qualsiasi eseguibile lì.
* **Skills, comandi e agenti**: questi entrano nel contesto di Claude come istruzioni, quindi influenzano ciò che Claude fa con gli strumenti che ha già.
* **Aggiornamenti**: quando l'aggiornamento automatico è attivo per il marketplace da cui hai installato un plugin, Claude Code aggiorna quel plugin in background, quindi i file che hai esaminato possono cambiare su disco. [Quando viene eseguito l'aggiornamento automatico](/docs/it/plugins/loading#when-auto-update-runs) ha i tempi. Per attivare o disattivare l'aggiornamento automatico per marketplace, vedi [Keep plugins updated](/docs/it/plugins/install#keep-plugins-updated).

Le [regole di autorizzazione](/docs/it/permissions) e la [sandbox](/docs/it/sandboxing) di Claude Code coprono le chiamate agli strumenti che Claude fa, non il codice che un plugin esegue da solo:

* **Hooks e processi server**: gli hook dei comandi eseguono comandi shell con i tuoi permessi utente completi. Claude Code esegue hook e server MCP al di fuori della sandbox.
* **Chiamate agli strumenti di Claude**: una chiamata a uno degli strumenti MCP del plugin e un comando Bash che esegue un eseguibile dalla `bin/` del plugin sono chiamate agli strumenti, quindi le tue regole di autorizzazione si applicano a loro.

L'installazione di un plugin lo abilita anche, a meno che il suo manifest o la voce del marketplace non imposti [`defaultEnabled: false`](/docs/it/plugins/install#choose-an-install-scope) e tu non l'abbia abilitato tu stesso.

Per rimuovere un plugin di cui non ti fidi più, vedi [Remove a plugin you no longer trust](#remove-a-plugin-you-no-longer-trust).

<h2 id="marketplace-tiers">
  Identifica i marketplace di Anthropic per nome
</h2>

Il nome di un marketplace lo colloca in uno di tre livelli: ufficiale, community o di terze parti. Claude Code accetta i nomi ufficiali e community solo per i marketplace provenienti da repository `github.com/anthropics/`, quindi un marketplace di terze parti non può presentarsi come uno di Anthropic. Un marketplace che un collega o la tua organizzazione pubblica è di terze parti.

La tabella elenca quali nomi rientrano in ogni livello:

| Livello     | Quali marketplace                                                                               |
| :---------- | :---------------------------------------------------------------------------------------------- |
| Ufficiale   | I [nomi ufficiali del marketplace](#official-marketplace-names), come `claude-plugins-official` |
| Community   | `claude-community`, `claude-plugins-community` e `healthcare`                                   |
| Terze parti | Ogni altro marketplace                                                                          |

Dove il catalogo `claude-community` fissa un plugin a un commit SHA, cosa che fa per quasi ogni voce, Claude Code rifiuta di installare un commit diverso.

<h3 id="official-marketplace-names">
  Nomi ufficiali del marketplace
</h3>

Questi nomi di marketplace compongono il livello ufficiale:

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

Per come i marketplace ufficiali, community e demo differiscono e dove sfogliare cosa elenca ognuno, vedi [Anthropic's marketplaces](/docs/it/plugins/anthropic-marketplaces).

<h2 id="review-a-plugin-before-you-install">
  Esamina un plugin prima di installarlo
</h2>

Prima di installare un plugin, guarda cosa aggiunge e da dove viene.

<Steps>
  <Step title="Controlla la fonte del marketplace">
    Nella tua shell, esegui `claude plugin marketplace list` per stampare la fonte da cui è stato aggiunto ogni marketplace, come un repository GitHub o una directory.
  </Step>

  <Step title="Leggi il riquadro dei dettagli">
    In una sessione Claude Code, esegui `/plugin` e seleziona il plugin. Il riquadro dei dettagli mostra una sezione **Will install** che elenca i comandi, gli agenti, le skills, gli hooks e i server MCP e LSP del plugin. Per un plugin per il quale Anthropic non ha dati di componenti pubblicati, la sezione mostra ciò che la voce del marketplace dichiara, o una nota: `Components will be discovered at installation` per un plugin archiviato all'interno del marketplace, o `Component summary not available for remote plugin` per uno recuperato da altrove.
  </Step>

  <Step title="Leggi la fonte del plugin">
    Nel riquadro dei dettagli, seleziona **Open homepage** o **View on GitHub** sotto le opzioni di installazione. Se il riquadro non offre nessuno dei due, apri il repository del marketplace che hai trovato nel primo passaggio. Trova la directory del plugin lì. La sezione **Will install** mostra che esiste un hook ma non cosa esegue, quindi leggi questi file nella directory del plugin:

    * **`hooks/hooks.json`**: il comando che ogni hook esegue
    * **`.mcp.json`**: il comando o l'URL di ogni server
    * **`bin/`**: ogni file nella directory
  </Step>

  <Step title="Elenca cosa contiene il plugin">
    Clona il repository che contiene la directory del plugin, quindi esegui `claude --plugin-dir <plugin directory> plugin details <plugin name>` nella tua shell per vedere cosa Claude Code trova in esso. Il comando legge i file del plugin senza avviare una sessione e stampa un `Component inventory` che elenca le skills e i comandi del plugin, gli agenti, gli hook con l'evento di ogni hook, e i server MCP e LSP.
  </Step>
</Steps>

Dopo aver installato un plugin, esegui `claude plugin details <plugin name>` nella tua shell per stampare lo stesso `Component inventory` per la copia installata sotto `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`.

<h3 id="remove-a-plugin-you-no-longer-trust">
  Rimuovi un plugin di cui non ti fidi più
</h3>

Nella tua shell, esegui [`claude plugin uninstall <plugin>`](/docs/it/plugins/cli-reference#plugin-uninstall) con lo `--scope` in cui l'hai installato. Quindi controlla cosa ha rimosso la disinstallazione e cosa ha lasciato:

* **Dati persistenti**: quando era l'ultimo scope in cui il plugin era installato, la disinstallazione elimina anche la directory dei dati persistenti del plugin, a meno che non passi `--keep-data`.
* **File memorizzati nella cache**: i file del plugin rimangono su disco sotto `~/.claude/plugins/cache/` per 14 giorni prima che una [scansione di background li rimuova](/docs/it/plugins/loading#cleanup-of-previous-versions). Dopo aver disinstallato l'ultimo plugin, le directory orfane rimangono fino a quando non installi un altro. Per eliminare i file ora, rimuovi tu stesso la directory del plugin sotto `~/.claude/plugins/cache/<marketplace>/<plugin>/`.
* **Il marketplace**: se non ti fidi nemmeno del proprietario del marketplace, [rimuovi il marketplace](/docs/it/plugins/install#manage-marketplaces) anche, che disinstalla ogni plugin che hai installato da esso.

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Riconosci quando Claude Code rifiuta o avverte
</h2>

Il riquadro dei dettagli che apri dalla scheda **Discover** o **Marketplaces** in `/plugin` mostra lo stesso avviso di fiducia per ogni plugin. Claude Code rifiuta invece di avvertire in casi come quelli sotto [Untrusted marketplace sources and failed integrity checks](#untrusted-marketplace-sources-and-failed-integrity-checks).

<h3 id="trust-warning-before-you-install">
  Avviso di fiducia prima di installare
</h3>

L'avviso legge lo stesso indipendentemente da quale marketplace provenga il plugin:

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

Se la tua organizzazione imposta `pluginTrustMessage` nelle [managed settings](/docs/it/plugins/org), Claude Code aggiunge quel testo all'avviso.

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  Fonti di marketplace non affidabili e controlli di integrità falliti
</h3>

Claude Code rifiuta di caricare un marketplace o di installare un plugin in questi casi, ognuno con il suo messaggio di errore:

* **Fonte di marketplace non affidabile**: quando un marketplace utilizza un nome ufficiale o community ma la sua fonte è al di fuori di `github.com/anthropics/`, Claude Code smette di caricare il marketplace e i plugin che hai installato da esso. L'errore è [Marketplace is registered from an untrusted source](/docs/it/errors#marketplace-is-registered-from-an-untrusted-source).
* **Integrità dell'archivio**: quando una voce del marketplace fissa un'[origine `archive`](/docs/it/plugins/marketplace-reference#archive-plugin-source) a un digest `sha256` e il digest del file scaricato non corrisponde, Claude Code rifiuta l'installazione. L'errore è [Plugin archive integrity check failed](/docs/it/errors#plugin-archive-integrity-check-failed).

Il pin `sha256` è separato dal pin del commit SHA del catalogo community, che seleziona il commit git da controllare.

<h2 id="enforce-plugin-controls-for-your-organization">
  Applica i controlli dei plugin per la tua organizzazione
</h2>

Con [managed settings](/docs/it/plugins/org), un amministratore può applicare questi controlli dei plugin:

* Allowlist o blocklist delle fonti del marketplace
* Forzare l'abilitazione dei plugin
* Disattivare i flag `--plugin-dir` e `--plugin-url` e la variabile `CLAUDE_CODE_PLUGIN_DIRS`
* Limitare gli hook a quelli dalle managed settings e dai plugin forzati
* Impedire ai plugin dagli account claude.ai dei membri di caricarsi in Claude Code, con [`syncClaudeAiPlugins`](/docs/it/plugins/org#control-matrix)

La [matrice di controllo](/docs/it/plugins/org#control-matrix) dice cosa fa e non copre ogni chiave.

<h2 id="find-plugins-in-telemetry">
  Trova i plugin nella telemetria
</h2>

Se la tua organizzazione esporta gli [eventi OpenTelemetry](/docs/it/monitoring-usage) di Claude Code al suo backend, i [livelli del marketplace](#marketplace-tiers) decidono quali nomi di plugin appaiono lì:

* **[Plugin loaded event](/docs/it/monitoring-usage#plugin-loaded-event)**: l'evento segnala i nomi ufficiali del plugin e del marketplace come sono. Per i livelli community e terze parti, `plugin.name` e `marketplace.name` sono la stringa letterale `third-party` a meno che non imposti `OTEL_LOG_TOOL_DETAILS=1`.
* **Scope del plugin**: lo `plugin.scope` dell'evento caricato segnala ancora da dove proviene il plugin, come `org` per un plugin che le tue managed settings abilitano o `user-local` per qualsiasi altro plugin di terze parti. L'[evento plugin loaded](/docs/it/monitoring-usage#plugin-loaded-event) elenca ogni valore.
* **[Plugin installed event](/docs/it/monitoring-usage#plugin-installed-event)**: a meno che non imposti `OTEL_LOG_TOOL_DETAILS=1`, l'evento omette i campi del nome per i plugin non ufficiali invece di segnalare `third-party`.
* **[Claude Code Analytics API](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**: Claude Code segnala i plugin dai livelli ufficiali e community per nome e segnala ogni altro plugin come `third-party`.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Gestire i plugin per la vostra organizzazione](/docs/it/plugins/org): limitare da quali marketplace gli utenti possono installare e richiedere quelli di cui vi fidate
* [Installare e gestire i plugin](/docs/it/plugins/install): esaminare il riquadro dei dettagli di un plugin prima di scegliere un ambito
* [Marketplace di Anthropic](/docs/it/plugins/anthropic-marketplaces): quali nomi di marketplace sono di Anthropic
* [Sicurezza](/docs/it/security): il modello di sicurezza proprio di Claude Code
