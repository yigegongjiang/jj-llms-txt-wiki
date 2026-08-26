> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Consigliare plugin per la vostra organizzazione

> Aggiungere un blocco di rilevanza alle voci dei plugin del marketplace in modo che Claude Code li suggerisca quando il lavoro di un utente corrisponde.

Se gestite un marketplace di plugin per la vostra organizzazione, potete fare in modo che Claude Code suggerisca plugin specifici agli utenti in base a ciò su cui stanno lavorando. Aggiungete un blocco `relevance` alla voce di un plugin in `marketplace.json`, quindi inserite il marketplace nella lista consentita delle impostazioni gestite. Quando la sessione di un utente corrisponde a uno dei segnali dichiarati, Claude Code mostra un suggerimento di installazione per quel plugin.

I suggerimenti dichiarati dal marketplace sono opt-in per ogni marketplace tramite [impostazioni gestite](/docs/it/settings#settings-files). Nessuna dichiarazione `relevance` di un marketplace produce suggerimenti finché un amministratore non la aggiunge alla lista consentita, incluso il marketplace ufficiale di Anthropic. Claude Code include anche un suggerimento integrato che è indipendente da questa lista consentita; questo suggerimento e tutti i suggerimenti dichiarati dal marketplace sono disabilitati quando [`spinnerTipsEnabled`](/docs/it/settings#available-settings) è impostato su `false`.

Questa funzione richiede Claude Code v2.1.152 o successivo. I client più vecchi ignorano il campo `relevance`.

Questa pagina è per gli operatori del marketplace e gli amministratori aziendali. Se state cercando di installare plugin, consultate [Scoprire e installare plugin](/docs/it/discover-plugins).

<h2 id="how-it-works">
  Come funziona
</h2>

Ogni voce di plugin in `marketplace.json` può contenere un oggetto `relevance`. L'oggetto nomina un argomento e uno o più segnali. Un segnale è un modello che Claude Code testa rispetto alla sessione corrente, come la directory di lavoro o i file che Claude ha letto.

La corrispondenza dei segnali avviene localmente sulla macchina dell'utente. La corrispondenza non aggiunge traffico di rete e non segnala quali segnali hanno corrisposto, o i loro valori, ad Anthropic o all'operatore del marketplace.

Quando un segnale corrisponde e il plugin non è già installato, Claude Code mostra il plugin in tre posizioni:

* **Suggerimento spinner**: un messaggio "Stai lavorando con *argomento*? Installa il plugin *plugin*" con il comando `/plugin install` appare sotto lo spinner mentre Claude sta rispondendo.
* **Suggerimento all'inizio della sessione**: se il segnale `cwd` corrisponde alla directory di lavoro, una notifica `plugin suggestion: <name>@<marketplace> · /plugin` su una riga appare prima del primo turno. Questa superficie richiede Claude Code v2.1.153 o successivo.
* **Scheda Discover di `/plugin`**: il plugin è fissato in cima all'elenco Discover con un'annotazione come "suggerito per questa directory" o "suggerito per i comandi stripe". Questa superficie richiede Claude Code v2.1.154 o successivo.

Il suggerimento spinner e la notifica all'inizio della sessione fanno parte del sistema di suggerimenti spinner. Entrambi sono disabilitati quando l'utente o il progetto imposta `spinnerTipsEnabled` su `false`, o quando è configurato un `spinnerTipsOverride` personalizzato con `excludeDefault`. Il pin della scheda Discover è indipendente dalle impostazioni dei suggerimenti.

Claude Code non installa mai un plugin automaticamente. L'utente conferma sempre.

<h2 id="add-relevance-to-a-plugin-entry">
  Aggiungere rilevanza a una voce di plugin
</h2>

Aggiungete un oggetto `relevance` alla voce del plugin nel vostro `marketplace.json`. L'esempio seguente dichiara che il plugin `terraform-helpers` è rilevante quando Claude legge un file `.tf` o quando Claude esegue `terraform`:

```json theme={null}
{
  "name": "acme-corp-plugins",
  "owner": { "name": "Acme Platform Team" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Acme conventions and helpers for Terraform",
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

Un plugin con un blocco `relevance` ma senza segnali corrispondenti si comporta come qualsiasi altra voce del marketplace. Appare nell'elenco Discover nella sua posizione normale e non viene mai visualizzato come suggerimento spinner.

<h2 id="field-reference">
  Riferimento dei campi
</h2>

<h3 id="relevance">
  `relevance`
</h3>

| Campo     | Tipo   | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :-------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Facoltativo. La frase che riempie "Stai lavorando con *argomento*?" nel suggerimento spinner. Spesso il nome del prodotto, ad esempio `Stripe`. Utilizzate un dominio come `design` quando il nome del plugin non si legge naturalmente come argomento. Per impostazione predefinita, il nome del plugin con ogni segmento di trattino maiuscolo. La notifica all'inizio della sessione non utilizza questo valore. Massimo 64 caratteri. |
| `signals` | object | Corrispondenti che determinano quando il plugin è rilevante. Almeno un segnale è richiesto affinché il plugin sia suggeribile. Consultate la tabella sottostante.                                                                                                                                                                                                                                                                         |

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

| Campo          | Tipo             | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Modelli Glob abbinati alla directory di lavoro della sessione. Abbinati come percorso assoluto e, quando all'interno di un repository git, come percorso relativo alla radice del repository. Normalizzati con barra in avanti e senza distinzione tra maiuscole e minuscole. Ogni modello corrisponde alla directory stessa e a tutto ciò che contiene, quindi `infra`, `infra/`, e `infra/**` si comportano in modo identico. Questo è l'unico segnale che può corrispondere all'inizio della sessione, prima del primo turno. Massimo 10 modelli di 256 caratteri ciascuno.                                                                                                                                                                                                                                                                                                                                                                                     |
| `cli`          | array of strings | Nomi di comandi da comandi shell che Claude ha eseguito in questa sessione, ad esempio `["stripe"]`. Si applica su ogni piattaforma: i comandi eseguiti su Windows tramite PowerShell o Git Bash vengono registrati allo stesso modo. Claude Code registra un nome di comando per ogni invocazione dello strumento shell: il primo token dopo eventuali assegnazioni di variabili di ambiente iniziali e `sudo`. I comandi composti contribuiscono solo con il loro comando iniziale, quindi `cd infra && terraform plan` registra `cd`, non `terraform`. Corrispondenza esatta. Massimo 10 voci di 64 caratteri ciascuna.                                                                                                                                                                                                                                                                                                                                         |
| `hosts`        | array of strings | Nomi host visti in URL `http://` o `https://` nei comandi Bash di questa sessione, ad esempio `["api.stripe.com"]`. Solo il nome host in minuscolo: nessuno schema, porta o percorso. Corrispondenza esatta senza distinzione tra maiuscole e minuscole. Massimo 20 voci di 128 caratteri ciascuna.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `filesRead`    | array of strings | Modelli Glob abbinati ai percorsi dei file che Claude ha letto in questa sessione, ad esempio `["**/*.tf"]`. Normalizzati con barra in avanti e senza distinzione tra maiuscole e minuscole. Massimo 10 modelli di 256 caratteri ciascuno.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `manifestDeps` | array of objects | Dipendenze dichiarate nei manifesti di pacchetto che Claude ha letto in questa sessione. Ogni voce è `{ "file": "...", "pattern": "..." }`, dove `file` è un'espressione regolare abbinata al percorso del file manifest come registrato nello stato della sessione, tipicamente un percorso assoluto, e `pattern` è un'espressione regolare abbinata al contenuto di quel file. Ancorate `file` alla fine, ad esempio `[/\\\\]package\\.json$` in forma con escape JSON, perché un modello ancorato all'inizio non corrisponde mai a un percorso assoluto. I percorsi non sono normalizzati per il separatore per questo segnale, quindi i percorsi Windows utilizzano barre rovesciate. I file manifest più grandi di 512 KB vengono saltati. Entrambi i valori sono stringhe di origine JavaScript `RegExp` di al massimo 256 caratteri. `file` corrisponde senza distinzione tra maiuscole e minuscole. `pattern` è sensibile alle maiuscole. Massimo 10 voci. |

I segnali `cli`, `hosts`, `filesRead` e `manifestDeps` necessitano della cronologia della sessione, quindi possono corrispondere solo al suggerimento spinner e alla scheda Discover. Solo `cwd` può corrispondere all'inizio della sessione. I segnali `filesRead` e `manifestDeps` testano lo stato del file registrato della sessione, che include anche i file che Claude ha scritto o modificato e i file di memoria `CLAUDE.md` caricati automaticamente.

L'esempio seguente utilizza `manifestDeps` per suggerire un plugin Stripe una volta che Claude ha letto un `package.json` che dipende da `stripe`. Il modello `file` utilizza `[/\\\\]` in modo che corrisponda sia ai separatori di percorso con barra in avanti che con barra rovesciata, e `\\.` in modo che il punto sia letterale. In JSON, ogni barra rovesciata nell'espressione regolare è scritta due volte.

```json theme={null}
{
  "name": "stripe-helpers",
  "source": "./plugins/stripe-helpers",
  "relevance": {
    "topic": "Stripe",
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"stripe\"\\s*:"
        }
      ]
    }
  }
}
```

<Note>
  I campi sconosciuti sotto `relevance` e `relevance.signals` vengono ignorati al momento del caricamento in modo che i client Claude Code più vecchi continuino a caricare il vostro marketplace. Eseguite `claude plugin validate` per visualizzarli come avvisi.
</Note>

<h2 id="enable-suggestions-in-managed-settings">
  Abilitare i suggerimenti nelle impostazioni gestite
</h2>

Dichiarare `relevance` in `marketplace.json` non è sufficiente di per sé. Un amministratore deve inserire il marketplace nella lista consentita nelle [impostazioni gestite](/docs/it/settings#settings-files) prima che i suoi suggerimenti vengano visualizzati agli utenti.

Aggiungete il nome del marketplace a `pluginSuggestionMarketplaces`. Per qualsiasi marketplace diverso dal marketplace ufficiale di Anthropic, dichiarate anche la fonte del marketplace nelle stesse impostazioni gestite, sia come voce di quel nome in `extraKnownMarketplaces` che come voce in `strictKnownMarketplaces`. Il nome inserito nella lista consentita viene ignorato se il marketplace registrato sulla macchina proviene da una fonte diversa. Questo impedisce a una fonte non correlata di registrarsi con un nome inserito nella lista consentita per avere i suoi plugin suggeriti in tutta l'organizzazione.

Il seguente `managed-settings.json` registra un marketplace dell'organizzazione da un repository GitHub e abilita i suoi suggerimenti:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-corp-plugins": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Il marketplace ufficiale è esente dal requisito di dichiarazione della fonte perché il suo nome può registrarsi solo dalla fonte ufficiale di Anthropic. Inserire il nome nella lista consentita è sufficiente:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

Consultate il [riferimento delle impostazioni](/docs/it/settings) per `pluginSuggestionMarketplaces` e [`extraKnownMarketplaces`](/docs/it/settings#extraknownmarketplaces) per i dettagli completi della configurazione.

<h2 id="what-the-user-sees">
  Cosa vede l'utente
</h2>

Quando un segnale corrisponde durante una sessione, il suggerimento spinner recita:

```text theme={null}
Stai lavorando con Terraform? Installa il plugin terraform-helpers:
/plugin install terraform-helpers@acme-corp-plugins
```

All'inizio della sessione, un segnale `cwd` corrispondente visualizza la notifica su una riga:

```text theme={null}
plugin suggestion: terraform-helpers@acme-corp-plugins · /plugin
```

Il suggerimento di un determinato plugin appare al massimo una volta ogni tre sessioni tra il suggerimento spinner e la notifica all'inizio della sessione combinati, e nessuno dei due si ripete una volta che il plugin è installato. La notifica all'inizio della sessione inoltre smette di apparire dopo che il suggerimento è stato mostrato due volte.

Nella scheda Discover di `/plugin`, il plugin è fissato sopra gli altri risultati con un'annotazione che nomina il segnale corrispondente, come `suggested for this directory` o `suggested for terraform commands`. La scheda Discover fissa un determinato plugin una volta; le visite successive lo elencano in ordine normale. Il pin della scheda Discover richiede Claude Code v2.1.154 o successivo. Su v2.1.152 appare solo il suggerimento spinner; la notifica all'inizio della sessione viene aggiunta in v2.1.153.

<h2 id="validate-your-marketplace">
  Convalidare il vostro marketplace
</h2>

Eseguite `claude plugin validate` sulla directory del vostro marketplace per controllare il blocco `relevance` prima della pubblicazione:

```
claude plugin validate ./my-marketplace
```

Il validatore segnala le chiavi sconosciute sotto `relevance` e `relevance.signals` come avvisi, contrassegna un valore `relevance` che non è un oggetto e rifiuta una voce `signals.hosts` che include uno schema, una porta o un percorso.

<h2 id="see-also">
  Vedere anche
</h2>

* [Creare e distribuire un marketplace di plugin](/docs/it/plugin-marketplaces): costruire il marketplace che ospita i vostri plugin
* [Consigliare il vostro plugin dalla vostra CLI](/docs/it/plugin-hints): richiedere agli utenti dalla vostra CLI invece che dai segnali di sessione di Claude Code
* [Impostazioni](/docs/it/settings): riferimento completo per `pluginSuggestionMarketplaces` e `extraKnownMarketplaces`
