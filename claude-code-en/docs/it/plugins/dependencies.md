> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dipendenze dei plugin

> Dichiara i plugin da cui il tuo plugin dipende, con intervalli di versione come ^1.2, e scopri come Claude Code installa, risolve e elimina le dipendenze.

Una dipendenza di plugin è un altro plugin su cui il tuo plugin si basa, ad esempio uno il cui server MCP o skill chiami. Ogni dipendenza traccia l'ultima versione fornita dal suo marketplace a meno che tu non dichiari un vincolo di versione, un intervallo di versione semantica come `^2.0` o `~2.1.0` che hai testato.

Questa pagina è per gli autori di plugin che dichiarano dipendenze in `plugin.json` e per i manutentori del marketplace che taggano i rilasci.

<Note>
  Questi casi sono coperti su altre pagine:

  * **Installazione di un plugin che ha dipendenze**: vedi [Gestisci plugin installati](/docs/it/plugins/install#manage-installed-plugins)
  * **Lettura di un errore di dipendenza**: vedi [Errori di dipendenza](/docs/it/plugins/troubleshooting#dependency-errors)
  * **Dichiarazione dei pacchetti npm e Bun di cui il codice del tuo plugin ha bisogno**: vedi [Dipendenze dei pacchetti Node.js](/docs/it/plugins/loading#node-js-package-dependencies)
</Note>

Per aggiungere un vincolo, inizia da [Dichiara una dipendenza con un vincolo di versione](#declare-a-dependency-with-a-version-constraint). Se mantieni un plugin da cui altri dipendono, [tagga i tuoi rilasci](#tag-plugin-releases-for-version-resolution) in modo che i loro vincoli possano risolversi.

<h2 id="declare-dependencies">
  Dichiara dipendenze
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />Senza un vincolo di versione, una dipendenza si sposta a ogni nuovo rilascio che il suo marketplace pubblica la prossima volta che gli utenti aggiornano. Se quel rilascio rinomina uno strumento MCP che il tuo plugin chiama, il tuo plugin si interrompe per tutti coloro che aggiornano.

Con un vincolo come `~2.1.0` su una dipendenza da una fonte supportata da git, gli utenti che hanno il tuo plugin installato continuano a ricevere patch `2.1.x` della dipendenza e non si spostano mai a `2.2`. Per aggiornare secondo il tuo programma, testa una versione più recente e quindi pubblica una nuova versione del tuo plugin con un vincolo più ampio.

<h3 id="declare-a-dependency-with-a-version-constraint">
  Dichiara una dipendenza con un vincolo di versione
</h3>

Elenca le dipendenze nell'array `dependencies` del file `.claude-plugin/plugin.json` del tuo plugin. Il seguente manifest dichiara una dipendenza senza versione e una dipendenza vincolata:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

Una voce può essere una stringa: il nome del plugin da solo, come `"audit-logger"` in questo manifest, o `"name@marketplace"` per risolverlo in un altro marketplace. Con una stringa semplice, il tuo plugin dipende da qualsiasi versione fornita dal marketplace di quel plugin.

Per impostare un vincolo di versione, usa un oggetto con questi campi, ognuno una stringa:

| Campo         | Descrizione                                                                                                                                                                                                                                                                                                              |
| :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Il nome del plugin della dipendenza, come appare nella sua voce del marketplace. Claude Code lo cerca nello stesso marketplace del plugin dichiarante a meno che tu non imposti `marketplace`. Obbligatorio.                                                                                                             |
| `version`     | Un [intervallo di versione semantica](https://github.com/npm/node-semver#ranges) come `~2.1.0`, `^2.0`, `>=1.4`, o `=2.1.0`. La dipendenza si installa al tag git più alto che soddisfa questo intervallo, quindi il manutentore della dipendenza deve [taggare i rilasci](#tag-plugin-releases-for-version-resolution). |
| `marketplace` | Un marketplace diverso in cui risolvere `name`. Una lista di autorizzazione controlla le dipendenze tra marketplace, descritte in [Dipendi da un plugin di un altro marketplace](#depend-on-a-plugin-from-another-marketplace).                                                                                          |

Un intervallo non corrisponde a versioni pre-release come `2.0.0-beta.1` a meno che tu non accetti con un suffisso pre-release come `^2.0.0-0`.

<h3 id="bundle-plugins-for-a-team">
  Raggruppa plugin per un team
</h3>

Per consentire agli ingegneri di installare un set curato di plugin con un comando, pubblica un plugin il cui manifest contiene un `name` e un array `dependencies`. Un manifest di plugin ha bisogno solo di `name`, quindi questo è un plugin valido, e installarlo installa ogni dipendenza.

Ad esempio, un team di piattaforma può pubblicare bundle specifici per ruolo in un marketplace interno in modo che gli ingegneri eseguano un `claude plugin install` invece di installare ogni plugin separatamente:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

Per aggiungere un plugin al set standard in seguito, pubblica una nuova versione di `backend-standard` con la dipendenza aggiuntiva. Quando il marketplace non [auto-aggiorna per impostazione predefinita](/docs/it/plugins/loading#which-marketplaces-and-plugins-auto-update), gli ingegneri attivano l'auto-aggiornamento per il marketplace o aggiornano manualmente:

* **Attiva l'auto-aggiornamento per il marketplace**: il prossimo auto-aggiornamento sposta il bundle alla nuova versione e installa tutte le dipendenze che aggiunge.
* **Aggiorna manualmente**: esegui `claude plugin update backend-standard` in una shell, quindi `/reload-plugins` in una sessione aperta per installare le dipendenze appena aggiunte.

Per i passaggi lato ingegnere, vedi [Mantieni i plugin aggiornati](/docs/it/plugins/install#keep-plugins-updated).

Per distribuire un bundle a tutti in un'organizzazione, un amministratore lo aggiunge a `enabledPlugins` nelle impostazioni gestite. Vedi [Pre-installa e richiedi plugin](/docs/it/plugins/org#pre-install-and-require-plugins).

<h3 id="depend-on-a-plugin-from-another-marketplace">
  Dipendi da un plugin di un altro marketplace
</h3>

Per impostazione predefinita, Claude Code non installa una dipendenza da un marketplace diverso da quello del plugin dichiarante, a meno che l'utente non abbia già quella dipendenza installata e abilitata nello stesso ambito. Questo valore predefinito impedisce a un marketplace di installare silenziosamente plugin da una fonte che l'utente non ha revisionato.

Per consentire l'installazione, aggiungi il nome del marketplace di destinazione a `allowCrossMarketplaceDependenciesOn` nel file `marketplace.json` del marketplace root. Il marketplace root è quello che ospita il plugin che l'utente sta installando. Si applica solo la lista di autorizzazione del marketplace root.

Il seguente `marketplace.json` consente a `deploy-kit` di dipendere da un plugin di `your-shared-marketplace`:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

Se `allowCrossMarketplaceDependenciesOn` manca o non include il marketplace di destinazione, Claude Code non installa la dipendenza. Quando la dipendenza è dichiarata nella voce del marketplace, l'installazione stessa viene rifiutata con un messaggio che inizia con `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` e nomina il campo da impostare. Quando è dichiarata in `plugin.json`, l'installazione si completa senza la dipendenza e il tuo plugin non riesce a caricarsi.

Il controllo della lista di autorizzazione non si applica a una dipendenza che è già abilitata. Se un utente installa `audit-logger` da `your-shared-marketplace` da solo per primo, nello stesso ambito, `deploy-kit` si installa quindi senza alcun cambiamento alla lista di autorizzazione.

<h3 id="test-a-plugin-and-its-dependency-locally">
  Testa un plugin e la sua dipendenza localmente
</h3>

Se stai sviluppando un plugin e il plugin da cui dipende allo stesso tempo, avvia Claude Code dalla tua shell e carica entrambi con [`--plugin-dir`](/docs/it/plugins/cli-reference#flags-that-load-a-plugin-for-one-session):

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

La copia locale della dipendenza soddisfa la voce di dipendenza del tuo plugin, quindi non hai bisogno di installare la dipendenza dal suo marketplace.

* **Nessun `version` necessario**: il `plugin.json` locale non ha bisogno di un `version` neanche, perché un [vincolo di versione](#declare-a-dependency-with-a-version-constraint) non viene controllato rispetto a una copia locale.
* **Voci che nominano un marketplace**: una voce che nomina un marketplace corrisponde anche alla copia locale su Claude Code v2.1.242 o successivo.

Fino a quando non installi la dipendenza dal suo marketplace, il tuo plugin smette di caricarsi ogni volta che la copia locale è disabilitata o assente:

* **Hai disabilitato la copia locale**: il tuo plugin è disabilitato al prossimo caricamento del plugin, con un errore che termina con `is disabled — enable it or remove the dependency`. Quando l'errore nomina la dipendenza come `<name>@inline`, quell'identificatore si riferisce alla copia `--plugin-dir`.
* **Hai avviato una sessione senza il flag `--plugin-dir` della dipendenza**: l'errore segnala che la dipendenza non è installata. Passa il flag di nuovo, o installa la dipendenza dal suo marketplace.

Quando entrambi i plugin sono in una cartella padre, puoi passare quella cartella a `--plugin-dir` una volta. Se la cartella non è essa stessa un plugin, Claude Code carica ogni cartella figlio che ha un `.claude-plugin/plugin.json`. Richiede Claude Code v2.1.265 o successivo.

<h2 id="tag-plugin-releases-for-version-resolution">
  Rilascia un plugin da cui altri dipendono
</h2>

Se mantieni un plugin da cui altri plugin dipendono con un vincolo di versione, tagga i suoi rilasci in modo che quei vincoli possano risolversi. Un vincolo si risolve rispetto ai tag git nel repository che ospita il plugin. Tagga il repository a cui la [sorgente del plugin](/docs/it/plugins/marketplace-reference#plugin-sources) del plugin in `marketplace.json` punta:

* **Sorgente `github`, `url`, o `git-subdir`**: il repository del plugin stesso, quindi l'autore del plugin crea i tag
* **Percorso relativo come `./plugins/secrets-vault`**: il repository del marketplace, quindi il manutentore del marketplace crea i tag

<h3 id="create-a-release-tag">
  Crea un tag di rilascio
</h3>

Tagga ogni rilascio come `<plugin-name>--v<version>`, dove `<version>` corrisponde al campo `version` nel `plugin.json` di quel commit. Il prefisso plugin-name consente a un repository del marketplace di ospitare più plugin con cronologie di versione indipendenti.

Crea il tag dalla directory del plugin, con un remote `origin` configurato per ricevere il tag spinto, usando [`claude plugin tag`](/docs/it/plugins/cli-reference#plugin-tag):

```bash theme={null}
claude plugin tag --push
```

Il comando costruisce il nome del tag dal manifest del plugin. Prima di creare il tag, esegue questi controlli:

* Convalida il plugin
* Controlla che `plugin.json` e la voce del marketplace concordino sulla versione, quando la directory del plugin si trova all'interno di un checkout del marketplace
* Richiede un albero di lavoro pulito sotto la directory del plugin
* Rifiuta se il tag esiste già

Un'esecuzione riuscita stampa `Created tag secrets-vault--v2.1.0`. Con `--push`, stampa anche `Pushed to origin`. Senza `--push`, stampa il comando `git push` da eseguire tu stesso.

Passa `--dry-run` per vedere il piano senza creare nulla.

Il [riferimento `claude plugin tag`](/docs/it/plugins/cli-reference#plugin-tag) elenca i flag rimanenti.

Puoi anche eseguire `git tag secrets-vault--v2.1.0` direttamente, purché mantieni la `version` in `plugin.json` e nella voce del marketplace sincronizzate tu stesso.

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  Vincola una dipendenza che ha una sorgente non-git
</h3>

La risoluzione basata su tag si applica solo alle sorgenti supportate da git. Per una dipendenza con una sorgente di plugin `npm`, `archive`, o `command` [plugin source](/docs/it/plugins/marketplace-reference#plugin-sources), il vincolo non controlla quale versione viene recuperata. Viene comunque controllato quando il plugin si carica, e il plugin dipendente è disabilitato se la versione installata non lo soddisfa.

Per le sorgenti `npm`, `archive`, e `command`, la versione controllata è la `version` nel `plugin.json` della dipendenza. Impostane una lì prima di vincolare quella dipendenza, perché un `plugin.json` che non imposta alcuna versione non soddisfa alcun vincolo.

Claude Code non installa mai una dipendenza con una sorgente `command` da sola, quindi gli utenti [la installano per primi](/docs/it/plugins/marketplace-reference#command-plugin-source). Non esegue mai neanche il [`headersHelper`](/docs/it/plugins/host-marketplace#authenticate-archive-downloads) di una dipendenza, quindi gli utenti installano anche una dipendenza la cui voce del marketplace ne imposta una prima di installare il tuo plugin.

Oltre a `claude plugin install`, queste operazioni installano anche qualsiasi dipendenza dichiarata mancante, e i limiti `command` e `headersHelper` si applicano anche a loro:

* `/reload-plugins`
* Auto-aggiornamento del marketplace del plugin dipendente
* Ri-esecuzione di `claude plugin install` sul plugin dipendente
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  Come le dipendenze si comportano per i vostri utenti
</h2>

Queste sezioni descrivono come Claude Code risolve, controlla e combina i vincoli che dichiarate una volta che il vostro plugin è installato insieme ad altri.

<h3 id="how-a-constraint-resolves-against-tags">
  Come un vincolo si risolve rispetto ai tag
</h3>

Quando un utente installa un plugin che dichiara `{ "name": "secrets-vault", "version": "~2.1.0" }`, la dipendenza si installa dal tag `secrets-vault--v` più alto che soddisfa `~2.1.0` nel repository che ospita `secrets-vault`. Quando nessun tag soddisfa l'intervallo, l'installazione fallisce o utilizza la copia attuale del marketplace:

* **Plugin con il proprio repository**: l'installazione fallisce con un messaggio contenente `Dependency "secrets-vault@your-marketplace" has no git tag satisfying`.
* **Plugin referenziato da un percorso relativo**: l'installazione utilizza invece la copia attuale del marketplace, e il vincolo viene controllato quando il plugin si carica. Se quella copia è al di fuori dell'intervallo, il plugin dipendente rimane disabilitato e `claude plugin list` mostra `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`.

Per un plugin che il marketplace referenzia tramite un percorso relativo, un marketplace che avete aggiunto come percorso di cartella locale risolve anche i vincoli rispetto ai tag git di quella cartella, quando la cartella è un repository git. Questo richiede Claude Code v2.1.196 o successivo. Una cartella locale che non è un repository git non ha tag, quindi Claude Code installa la dipendenza dal contenuto attuale della cartella.

<h3 id="confirm-the-resolved-version">
  Confermare la versione risolta
</h3>

Per confermare quale versione un vincolo ha risolto, eseguite `claude plugin list` nella vostra shell. Una dipendenza risolta da tag mostra la sua versione con un suffisso di commit di 12 caratteri, come `2.1.0-8713c5b11005`.

I controlli dei vincoli utilizzano la versione del tag piuttosto che la `version` in `plugin.json`, anche se `plugin.json` a quel commit rimane indietro.

Se spostate forzatamente un tag a un commit diverso, l'installazione successiva recupera il contenuto di quel commit invece di riutilizzare una copia cache obsoleta. Consultate [Versions and updates](/docs/it/plugins/loading#versions-and-updates) per come la versione di un plugin diventa la sua chiave di cache.

<h3 id="combine-constraints-from-several-plugins">
  Combinare vincoli da più plugin
</h3>

Quando più plugin installati vincolano la stessa dipendenza, la dipendenza si risolve alla versione più alta che soddisfa tutti i loro intervalli. Le combinazioni comuni si risolvono così:

| Plugin A richiede | Plugin B richiede | Risultato                                                                                                                                        |
| :---------------- | :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `^2.0`            | `>=2.1`           | Un'installazione al tag `2.x` più alto a o sopra `2.1.0`. Entrambi i plugin si caricano.                                                         |
| `~2.1`            | `~3.0`            | L'installazione del plugin B fallisce con un messaggio `has conflicting version requirements`. Il plugin A e la dipendenza rimangono come erano. |
| `=2.1.0`          | nessuno           | La dipendenza rimane a `2.1.0`. L'aggiornamento automatico salta le versioni più recenti mentre il plugin A è installato.                        |

L'aggiornamento automatico recupera una dipendenza vincolata al tag git più alto che soddisfa l'intervallo di ogni plugin installato, piuttosto che alla versione più recente del marketplace. Se gli intervalli dei plugin installati non si sovrappongono, l'aggiornamento automatico lascia quella dipendenza alla sua versione attuale, e la scheda **Errors** di `/plugin` mostra una voce che nomina il plugin vincolante. Se si sovrappongono ma nessun tag rientra nell'intervallo, l'aggiornamento automatico recupera la copia attuale del marketplace e salta l'aggiornamento quando la `version` di quella copia cade al di fuori dell'intervallo di qualsiasi plugin installato.

Quando un utente disinstalla l'ultimo plugin che vincola una dipendenza, la dipendenza non è più vincolata a un intervallo di versione e riprende a tracciare la sua voce del marketplace al prossimo aggiornamento.

<h2 id="see-also">
  Vedi anche
</h2>

* [`claude plugin prune`](/docs/it/plugins/cli-reference#plugin-prune): rimuovi le dipendenze auto-installate che nessun plugin ha più bisogno
* [Ospita un marketplace](/docs/it/plugins/host-marketplace): canali di rilascio e raccomandazione di altri plugin
