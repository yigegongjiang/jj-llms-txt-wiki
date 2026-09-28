> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Consigliare plugin per la tua organizzazione

> Aggiungi un blocco di rilevanza alle voci dei plugin del marketplace in modo che Claude Code li suggerisca quando il lavoro di un utente corrisponde, e inserisci il marketplace nella whitelist nelle impostazioni gestite.

Claude Code può suggerire l'installazione di un plugin dal marketplace della tua organizzazione quando la sessione di un utente corrisponde ai segnali che definisci per quel plugin. I segnali includono la directory di lavoro, i file che Claude ha letto e i comandi che Claude ha eseguito. Li definisci aggiungendo un blocco `relevance` alla voce del plugin in `marketplace.json`.

Un operatore del marketplace scrive le voci `relevance`. Un amministratore quindi inserisce il marketplace nella whitelist nelle impostazioni gestite. Gli utenti non vedono alcun suggerimento da un marketplace finché non è inserito nella whitelist.

<Note>
  Questi casi sono trattati in altre pagine:

  * **Vuoi installare plugin**: vedi [Installare e gestire plugin](/docs/it/plugins/install)
  * **Vuoi disattivare i suggerimenti**: vedi [Comprendere come funziona la rilevanza dei plugin](#understand-how-plugin-relevance-works)
</Note>

Inizia con le sezioni per il tuo ruolo:

* **Operatori del marketplace**: leggi [come funzionano i suggerimenti](#understand-how-plugin-relevance-works), quindi [aggiungi rilevanza a una voce di plugin](#add-relevance-to-a-plugin-entry) e [convalida il tuo marketplace](#validate-your-marketplace)
* **Amministratori**: [abilita i suggerimenti nelle impostazioni gestite](#enable-suggestions-in-managed-settings)

<h2 id="understand-how-plugin-relevance-works">
  Understand how plugin relevance works
</h2>

Ogni voce di plugin in `marketplace.json` può includere un oggetto `relevance`. L'oggetto nomina un argomento e uno o più segnali. Un segnale è un pattern che Claude Code testa rispetto alla sessione corrente, come la directory di lavoro o i file che Claude ha letto.

La corrispondenza dei segnali avviene localmente sulla macchina dell'utente e non aggiunge traffico di rete. Claude Code non segnala quali segnali corrispondono o i loro valori ad Anthropic o all'operatore del marketplace.

Quando un segnale corrisponde e il plugin non è già installato, Claude Code suggerisce il plugin in questi luoghi:

* **Spinner tip**: un messaggio con il comando `/plugin install` appare sotto lo spinner mentre Claude sta rispondendo.
* **Notifica all'avvio della sessione**: se un segnale `cwd` corrisponde alla directory di lavoro, una notifica di una riga appare prima che l'utente invii un primo messaggio.
* **Scheda `/plugin` Discover**: il plugin è fissato in cima all'elenco Discover.

[Anteprima di ciò che l'utente vede](#preview-what-the-user-sees) mostra il testo esatto di ciascuno e con quale frequenza si ripetono.

Claude Code non installa mai il plugin automaticamente. L'utente conferma sempre.

Lo spinner tip e la notifica all'avvio della sessione smettono entrambi di apparire quando l'utente o il progetto imposta [`spinnerTipsEnabled`](/docs/it/settings-reference#spinnertipsenabled) su `false`, o quando un [`spinnerTipsOverride`](/docs/it/settings-reference#spinnertipsoverride) con `excludeDefault` sostituisce i suggerimenti incorporati. Il pin della scheda Discover non è interessato da nessuna delle due impostazioni.

<h2 id="add-relevance-to-a-plugin-entry">
  Add relevance to a plugin entry
</h2>

Aggiungi un oggetto `relevance` alla voce del plugin nel tuo `marketplace.json`. L'esempio seguente dichiara che il plugin `terraform-helpers` è rilevante quando Claude legge un file `.tf` o esegue `terraform`:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

Mentre nessuno dei suoi segnali corrisponde, il plugin mantiene la sua posizione normale nell'elenco Discover e non appare come uno spinner tip.

Per controllare il blocco prima della pubblicazione, [convalida il tuo marketplace](#validate-your-marketplace).

<h2 id="field-reference">
  Field reference
</h2>

L'oggetto `relevance` e il suo oggetto annidato `signals` accettano i campi nelle tabelle seguenti.

I client più vecchi caricano ancora un marketplace che utilizza campi `relevance` che non riconoscono, perché i campi sconosciuti sotto `relevance` e `relevance.signals` vengono ignorati al momento del caricamento. Un campo riconosciuto il cui valore supera il suo limite nel [riferimento ai campi](#field-reference) invalida l'intera voce del plugin, e gli utenti non possono installare quel plugin dal marketplace finché non lo correggi; `claude plugin validate` segnala gli stessi limiti.

<h3 id="relevance">
  `relevance`
</h3>

| Field     | Type   | Description                                                                                                                                                                                |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Facoltativo. La frase che riempie "Lavori con *topic*?" nello spinner tip. Per impostazione predefinita, il nome del plugin con ogni segmento di trattino maiuscolo. Massimo 64 caratteri. |
| `signals` | object | Matcher che determinano quando il plugin è rilevante. Claude Code suggerisce il plugin solo se è impostato almeno un segnale. Vedi [`relevance.signals`](#relevance-signals).              |

Il `topic` è spesso il nome del prodotto, ad esempio `Terraform`. Usa un dominio come `design` quando il nome del plugin non suona naturale come argomento.

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

L'oggetto `signals` accetta i seguenti campi.

| Field          | Type             | Description                                                                                                                                                                                                                                                                     | Limit                                                                                                |
| :------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Pattern Glob abbinati alla directory di lavoro della sessione. Vedi [corrispondenza della directory di lavoro](#working-directory-matching).                                                                                                                                    | 10 pattern di 256 caratteri ciascuno                                                                 |
| `cli`          | array of strings | Nomi di comandi da comandi shell che Claude ha eseguito questa sessione, ad esempio `["terraform"]`. Corrispondenza esatta. Vedi [corrispondenza del nome del comando](#command-name-matching).                                                                                 | 10 voci di 64 caratteri ciascuna                                                                     |
| `hosts`        | array of strings | Nomi host visti in URL `http://` o `https://` in comandi Bash questa sessione, ad esempio `["registry.terraform.io"]`. Solo nome host nudo in minuscolo: nessuno schema, porta o percorso. Corrispondenza esatta senza distinzione tra maiuscole e minuscole.                   | 20 voci di 128 caratteri ciascuna                                                                    |
| `filesRead`    | array of strings | Pattern Glob abbinati ai percorsi dei file che Claude ha letto questa sessione, ad esempio `["**/*.tf"]`. Normalizzato con barra in avanti e senza distinzione tra maiuscole e minuscole.                                                                                       | 10 pattern di 256 caratteri ciascuno                                                                 |
| `manifestDeps` | array of objects | Dipendenze dichiarate nei manifesti di pacchetto che Claude ha letto questa sessione. Ogni voce è `{ "file": "...", "pattern": "..." }`, dove entrambi i valori sono espressioni regolari. Vedi [corrispondenza delle dipendenze del manifesto](#manifest-dependency-matching). | 10 voci, ogni valore al massimo 256 caratteri. I file manifesto più grandi di 512 KB vengono saltati |

I segnali `filesRead` e `manifestDeps` corrispondono anche ai file che Claude ha scritto o modificato questa sessione e ai file di memoria `CLAUDE.md` caricati automaticamente del progetto.

<h4 id="working-directory-matching">
  Working directory matching
</h4>

`cwd` è l'unico segnale che può corrispondere all'avvio della sessione, prima che l'utente invii un primo messaggio.

Claude Code abbina ogni pattern `cwd` come segue:

* Il pattern viene abbinato alla directory di lavoro come percorso assoluto. Quando la sessione è all'interno di un repository git, viene anche abbinata al percorso della directory di lavoro relativo alla radice del repository.
* La corrispondenza è normalizzata con barra in avanti e senza distinzione tra maiuscole e minuscole.
* Ogni pattern corrisponde alla directory stessa e a tutto ciò che contiene, quindi `infra`, `infra/` e `infra/**` si comportano in modo identico.

<h4 id="command-name-matching">
  Command name matching
</h4>

Claude Code registra un nome di comando per ogni comando shell che Claude esegue: il primo token dopo eventuali assegnazioni di variabili di ambiente iniziali e `sudo`. I comandi composti contribuiscono solo al loro comando iniziale, quindi `cd infra && terraform plan` registra `cd`, non `terraform`.

<h4 id="manifest-dependency-matching">
  Manifest dependency matching
</h4>

Ogni voce `manifestDeps` accoppia due stringhe di origine JavaScript `RegExp`:

* `file`: abbinato senza distinzione tra maiuscole e minuscole rispetto al percorso del file manifesto. Il percorso è in genere assoluto, quindi ancorare il pattern alla fine piuttosto che all'inizio. I percorsi non sono normalizzati per il separatore per questo segnale, quindi i percorsi Windows utilizzano barre rovesciate.
* `pattern`: abbinato con distinzione tra maiuscole e minuscole rispetto al contenuto di quel file.

L'esempio seguente utilizza `manifestDeps` per suggerire il tuo plugin una volta che Claude ha letto un `package.json` che dipende dal pacchetto npm del tuo SDK, denominato `your-sdk` qui.

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

In questo esempio, il pattern `file` utilizza `[/\\\\]` in modo che corrisponda sia ai separatori di percorso con barra in avanti che con barra rovesciata, e `\\.` in modo che il punto sia letterale. In JSON, ogni barra rovesciata nell'espressione regolare è scritta due volte.

<h2 id="validate-your-marketplace">
  Validate your marketplace
</h2>

Nella tua shell, esegui `claude plugin validate` rispetto alla directory del tuo marketplace per controllare il blocco `relevance` prima della pubblicazione:

```bash theme={null}
claude plugin validate ./my-marketplace
```

Il validatore segnala errori e avvisi sul blocco `relevance`, inclusi questi:

* Segnala chiavi sconosciute sotto `relevance` e `relevance.signals` come avvisi
* Contrassegna un valore `relevance` che non è un oggetto
* Rifiuta una voce `signals.hosts` che include uno schema, una porta o un percorso

Ogni risultato viene stampato con il percorso del campo che riguarda, e l'output termina con `Validation passed`, `Validation passed with warnings` o `Validation failed`.

<h2 id="enable-suggestions-in-managed-settings">
  Enable suggestions in managed settings
</h2>

Gli utenti non vedono alcun suggerimento da un marketplace finché un amministratore non lo inserisce nella whitelist nelle [impostazioni gestite](/docs/it/plugins/org), anche quando il suo `marketplace.json` dichiara `relevance`.

Per inserire un marketplace nella whitelist, modifica le tue impostazioni gestite come segue:

* Aggiungi il nome del marketplace a `pluginSuggestionMarketplaces`.
* Per qualsiasi marketplace diverso dal marketplace ufficiale di Anthropic, dichiara anche l'origine del marketplace, come voce di quel nome in [`extraKnownMarketplaces`](/docs/it/plugins/org#require-a-marketplace-and-its-plugins) o come voce in [`strictKnownMarketplaces`](/docs/it/plugins/org#allowlist-with-strictknownmarketplaces).

Su una macchina in cui il marketplace non è registrato, o è registrato con il nome inserito nella whitelist da un'origine diversa, non appare alcun suggerimento da esso. Il controllo dell'origine impedisce a un'origine non correlata di registrarsi con un nome inserito nella whitelist per ottenere i suoi plugin suggeriti in tutta la tua organizzazione.

Il seguente `managed-settings.json` registra un marketplace dell'organizzazione da un repository GitHub e abilita i suoi suggerimenti:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

Il nome del marketplace ufficiale può registrarsi solo dall'origine ufficiale di Anthropic, quindi non ha bisogno di una dichiarazione di origine. Per il marketplace ufficiale, inserisci nella whitelist solo il nome:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

<h2 id="preview-what-the-user-sees">
  Preview what the user sees
</h2>

Quando il segnale `relevance` di un plugin corrisponde durante una sessione, il suggerimento sotto lo spinner legge:

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

Quando un segnale `cwd` corrisponde all'avvio della sessione, la notifica di una riga legge:

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

Nella scheda `/plugin` Discover, il plugin è fissato sopra gli altri risultati con un'annotazione che nomina il segnale corrispondente, come `suggested for this directory` o `suggested for terraform commands`.

Claude Code limita la frequenza con cui suggerisce un determinato plugin:

* Il suggerimento appare al massimo una volta ogni tre sessioni tra lo spinner tip e la notifica all'avvio della sessione combinati.
* La notifica all'avvio della sessione smette di apparire una volta che lo spinner tip e la notifica hanno mostrato il plugin un totale combinato di due volte.
* Né lo spinner tip né la notifica all'avvio della sessione si ripetono una volta che il plugin è installato.
* La scheda Discover fissa il plugin la prima volta che l'utente apre la scheda mentre i segnali del plugin corrispondono. Claude Code lo registra in `~/.claude.json`, quindi ogni volta successiva che l'utente apre `/plugin` su quella macchina, il plugin appare in ordine normale.

<h2 id="see-also">
  See also
</h2>

* [Host a marketplace](/docs/it/plugins/host-marketplace): esegui il marketplace che ospita i tuoi plugin
* [Marketplace reference](/docs/it/plugins/marketplace-reference#plugin-entries): ogni campo che una voce di plugin accetta
* [Recommend your plugin from your CLI](/docs/it/plugins/cli-hints): richiedi agli utenti dalla tua CLI invece che dai segnali di sessione di Claude Code
* [Manage plugins for your organization](/docs/it/plugins/org): `extraKnownMarketplaces`, `strictKnownMarketplaces` e il resto delle chiavi della politica dei plugin
