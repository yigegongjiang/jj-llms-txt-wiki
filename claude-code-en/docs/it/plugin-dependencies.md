> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Vincola le versioni delle dipendenze dei plugin

> Dichiara vincoli di versione sulle dipendenze dei plugin e raggruppa un set di plugin curato dietro un'unica installazione.

Un plugin può dipendere da altri plugin elencandoli in `plugin.json` o nella sua voce del marketplace. Per impostazione predefinita, una dipendenza traccia la versione più recente disponibile, quindi un rilascio upstream può modificare la dipendenza del tuo plugin senza preavviso. I vincoli di versione consentono di mantenere una dipendenza a un intervallo di versione testato fino a quando non scegli di passare a una versione più recente.

Quando installi un plugin che dichiara dipendenze, Claude Code le risolve e le installa automaticamente ed elenca quali dipendenze sono state aggiunte alla fine dell'output di installazione. Se una dipendenza successivamente scompare, `/reload-plugins` e l'aggiornamento automatico dei plugin in background la reinstallano, a condizione che il suo marketplace sia già nei marketplace configurati. L'esecuzione nuova di `claude plugin install` sul plugin dipendente, o l'aggiunta di un marketplace con `claude plugin marketplace add`, risolve anche eventuali dipendenze mancanti in sospeso. Le dipendenze da un marketplace che non hai aggiunto rimangono non risolte.

Questa guida è per gli autori di plugin che dichiarano dipendenze in `plugin.json` e per i manutentori del marketplace che taggano i rilasci. Per installare plugin che hanno dipendenze, vedi [Scopri e installa plugin](/docs/it/discover-plugins). Per lo schema completo del manifest, vedi il [Riferimento dei plugin](/docs/it/plugins-reference).

<h2 id="why-constrain-dependency-versions">
  Perché vincolare le versioni delle dipendenze
</h2>

Considera un marketplace interno in cui due team pubblicano plugin. Il team della piattaforma mantiene `secrets-vault`, un server MCP che avvolge un backend di segreti. Il team di deploy mantiene `deploy-kit`, che chiama `secrets-vault` per recuperare le credenziali durante i deploy.

`deploy-kit` è testato rispetto a `secrets-vault` v2.1.0. Senza un vincolo di versione, la prossima volta che il team della piattaforma tagga un rilascio che rinomina uno strumento MCP, l'aggiornamento automatico sposta `secrets-vault` di ogni ingegnere alla nuova versione e `deploy-kit` si rompe.

Con un vincolo di versione, `deploy-kit` dichiara che ha bisogno di `secrets-vault` nell'intervallo `~2.1.0`. Gli ingegneri con `deploy-kit` installato rimangono sulla patch `2.1.x` più alta corrispondente. Il team di deploy esegue l'aggiornamento secondo il proprio programma pubblicando una nuova versione di `deploy-kit` con un vincolo più ampio.

<h2 id="declare-a-dependency-with-a-version-constraint">
  Dichiara una dipendenza con un vincolo di versione
</h2>

Elenca le dipendenze nell'array `dependencies` del file `.claude-plugin/plugin.json` del tuo plugin. Ogni voce è un nome di plugin o un oggetto con un vincolo di versione.

Il seguente manifest dichiara una dipendenza senza versione e una dipendenza vincolata:

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

Una voce può essere una stringa semplice con solo il nome del plugin, come `"audit-logger"` nell'esempio sopra, che dipende da qualsiasi versione fornita dal marketplace di quel plugin. Per un maggiore controllo, usa un oggetto con questi campi:

| Campo         | Tipo   | Descrizione                                                                                                                                                                                                                                                                                         |
| :------------ | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | string | Nome del plugin. Si risolve all'interno dello stesso marketplace del plugin dichiarante. Obbligatorio.                                                                                                                                                                                              |
| `version`     | string | Un [intervallo semver](https://github.com/npm/node-semver#ranges) come `~2.1.0`, `^2.0`, `>=1.4`, o `=2.1.0`. La dipendenza viene recuperata alla versione taggata più alta che soddisfa questo intervallo.                                                                                         |
| `marketplace` | string | Un marketplace diverso in cui risolvere `name`. Le dipendenze cross-marketplace sono bloccate a meno che il marketplace di destinazione non sia elencato in [`allowCrossMarketplaceDependenciesOn`](#depend-on-a-plugin-from-another-marketplace) nel file `marketplace.json` del marketplace root. |

Il campo `version` accetta qualsiasi espressione supportata dal pacchetto `semver` di Node, inclusi intervalli caret, tilde, hyphen e comparator. Le versioni pre-release come `2.0.0-beta.1` sono escluse a meno che il tuo intervallo non opti per un suffisso pre-release come `^2.0.0-0`.

<h2 id="bundle-plugins-for-a-team">
  Raggruppare i plugin per un team
</h2>

Oltre al `name` obbligatorio, un manifest di plugin può consistere solo in un array `dependencies`. L'installazione di questo plugin scarica tutte le dipendenze, il che lo rende un modo per raggruppare un set di plugin curato dietro un'unica installazione.

Ad esempio, un team di piattaforma può pubblicare bundle specifici per ruolo in un marketplace interno in modo che gli ingegneri eseguano un solo `claude plugin install` invece di installare ogni strumento separatamente:

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

L'installazione di `backend-standard` risolve e installa tutte e quattro le dipendenze.

Per aggiungere uno strumento al set standard in seguito, pubblicare una nuova versione di `backend-standard` con la dipendenza aggiuntiva. L'auto-aggiornamento è disabilitato per impostazione predefinita per i marketplace non Anthropic, quindi gli ingegneri acquisiscono la nuova versione in uno di due modi:

* Abilitare l'auto-aggiornamento per il marketplace in `/plugin`. Il prossimo auto-aggiornamento sposta il bundle alla nuova versione e installa tutte le dipendenze che aggiunge.
* Eseguire `claude plugin update backend-standard`, quindi `/reload-plugins` per installare le dipendenze appena aggiunte.

Per distribuire i bundle in tutta l'organizzazione, aggiungere il plugin bundle a `enabledPlugins` nelle [impostazioni gestite](/docs/it/settings#enabledplugins).

<h2 id="depend-on-a-plugin-from-another-marketplace">
  Dipendi da un plugin di un altro marketplace
</h2>

Per impostazione predefinita, Claude Code rifiuta di installare automaticamente una dipendenza che si trova in un marketplace diverso da quello del plugin che la dichiara. Questo impedisce a un marketplace di estrarre silenziosamente plugin da una fonte che non hai revisionato.

Per consentirlo, il manutentore del marketplace root aggiunge il nome del marketplace di destinazione a `allowCrossMarketplaceDependenciesOn` in `marketplace.json`. Il marketplace root è quello che ospita il plugin che l'utente sta installando; solo la sua lista di autorizzazione viene consultata, quindi la fiducia non si propaga attraverso i marketplace intermedi.

Il seguente `marketplace.json` consente a `deploy-kit` di dipendere da un plugin di `acme-shared`:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "allowCrossMarketplaceDependenciesOn": ["acme-shared"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "acme-shared" }
      ]
    }
  ]
}
```

Se il campo è mancante o non include il marketplace di destinazione, l'installazione fallisce con un errore `cross-marketplace` che nomina il campo da impostare. Gli utenti possono comunque installare manualmente la dipendenza per prima, il che soddisfa il vincolo senza modificare la lista di autorizzazione.

<h2 id="tag-plugin-releases-for-version-resolution">
  Tagga i rilasci dei plugin per la risoluzione della versione
</h2>

I vincoli di versione si risolvono rispetto ai tag git nel repository del marketplace. Affinché Claude Code trovi le versioni disponibili di una dipendenza, i rilasci del plugin upstream devono essere taggati usando una convenzione di denominazione specifica.

Tagga ogni rilascio come `{plugin-name}--v{version}`, dove `{version}` corrisponde al campo `version` nel file `plugin.json` di quel commit. Dalla directory del plugin, esegui:

```bash theme={null}
claude plugin tag --push
```

Il comando `claude plugin tag` deriva il nome del tag dal manifesto del plugin e dalla voce del marketplace che lo contiene. Prima di creare il tag, convalida il contenuto del plugin, verifica che `plugin.json` e la voce del marketplace concordino sulla versione, richiede un albero di lavoro pulito nella directory del plugin e rifiuta se il tag esiste già. Aggiungi `--dry-run` per vedere cosa verrebbe taggato senza crearlo. Eseguire direttamente `git tag secrets-vault--v2.1.0` è equivalente se mantieni `plugin.json` e la voce del marketplace sincronizzati da solo.

Il prefisso del nome del plugin consente a un repository del marketplace di ospitare più plugin con linee di versione indipendenti. Il separatore `--v` viene analizzato come una corrispondenza di prefisso sul nome completo del plugin, quindi i nomi dei plugin che contengono trattini vengono gestiti correttamente.

Quando installi un plugin che dichiara `{ "name": "secrets-vault", "version": "~2.1.0" }`, Claude Code elenca i tag del marketplace, filtra quelli che iniziano con `secrets-vault--v` e recupera la versione più alta che soddisfa `~2.1.0`. Se non esiste un tag corrispondente, il plugin dipendente viene disabilitato con un errore che elenca le versioni disponibili.

Un marketplace aggiunto come percorso di cartella locale risolve i tag nello stesso modo quando la cartella è un repository git. Questo richiede Claude Code v2.1.196 o successivo. In due casi Claude Code installa la dipendenza dal contenuto attuale della cartella:

* Le versioni precedenti non leggono i tag da un marketplace di cartella locale, quindi una dipendenza vincolata si carica solo se quella copia soddisfa l'intervallo.
* Una cartella locale che non è un repository git non ha tag, indipendentemente dalla versione.

Il semver del tag risolto viene registrato separatamente dalla `version` di `plugin.json`, quindi i controlli dei vincoli utilizzano il tag che è stato effettivamente recuperato anche se `plugin.json` in quel commit ha un valore obsoleto. Il nome della directory della cache per un'installazione risolta da tag include un suffisso SHA del commit di 12 caratteri, quindi se un manutentore sposta forzatamente un tag a un commit diverso, l'installazione successiva ottiene una directory della cache nuova invece di riutilizzare contenuti obsoleti.

<Note>
  Per le fonti del marketplace `npm`, il vincolo non controlla quale versione viene recuperata, poiché la risoluzione basata su tag si applica solo alle fonti supportate da git. Il vincolo viene comunque controllato al momento del caricamento e il plugin dipendente viene disabilitato con `dependency-version-unsatisfied` se la versione installata non lo soddisfa.
</Note>

<h2 id="how-constraints-interact">
  Come i vincoli interagiscono
</h2>

Quando più plugin installati vincolano la stessa dipendenza, Claude Code interseca i loro intervalli e risolve la dipendenza alla versione più alta che soddisfa tutti loro. La tabella seguente mostra come si risolvono le combinazioni comuni.

| Plugin A richiede | Plugin B richiede | Risultato                                                                                                                 |
| :---------------- | :---------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `^2.0`            | `>=2.1`           | Un'installazione al tag `2.x` più alto a o sopra `2.1.0`. Entrambi i plugin si caricano.                                  |
| `~2.1`            | `~3.0`            | L'installazione del plugin B fallisce con `range-conflict`. Plugin A e la dipendenza rimangono come erano.                |
| `=2.1.0`          | nessuno           | La dipendenza rimane a `2.1.0`. L'aggiornamento automatico salta le versioni più recenti mentre il plugin A è installato. |

L'aggiornamento automatico recupera una dipendenza vincolata al tag git più alto che soddisfa l'intervallo di ogni plugin installato, piuttosto che alla versione più recente del marketplace, quindi la dipendenza continua a ricevere aggiornamenti all'interno del suo intervallo consentito. Se nessun tag soddisfa tutti gli intervalli, l'aggiornamento automatico salta quella dipendenza e elenca il salto nella scheda Errori di `/plugin`, nominando il plugin vincolante.

Quando disinstalli l'ultimo plugin che vincola una dipendenza, la dipendenza non viene più mantenuta e riprende a tracciare la sua voce del marketplace al prossimo aggiornamento.

<h2 id="enable-or-disable-a-plugin-with-dependencies">
  Abilita o disabilita un plugin con dipendenze
</h2>

L'abilitazione di un plugin abilita anche i plugin da cui dipende, e la disabilitazione di un plugin è bloccata se un altro plugin abilitato ne ha ancora bisogno. Entrambi i comportamenti richiedono Claude Code v2.1.143 o successivo. Le versioni precedenti abilitano o disabilitano solo il plugin denominato e visualizzano un errore `dependency-unsatisfied` al prossimo caricamento.

Quando abiliti un plugin, Claude Code abilita anche le sue dipendenze nello stesso ambito. Se una dipendenza ha le sue dipendenze, Claude Code abilita anche quelle. Il messaggio di successo elenca cos'altro è stato abilitato insieme al plugin che hai denominato. Se una dipendenza non può essere abilitata, il comando rifiuta e ti dice cosa sta bloccando e come risolvere:

| Condizione                                                                                            | Risultato                                                                                                                        |
| :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| Una dipendenza non è installata                                                                       | L'abilitazione fallisce e stampa il comando `claude plugin install` per ogni dipendenza mancante.                                |
| Una dipendenza è bloccata dalla politica dei plugin della tua organizzazione                          | L'abilitazione fallisce e nomina la dipendenza bloccata.                                                                         |
| Una dipendenza è impostata su `false` a un ambito con precedenza più alta dell'ambito di destinazione | L'abilitazione fallisce. Abilita la dipendenza a quell'ambito, o passa `--scope` per scrivere lì.                                |
| Tutte le dipendenze sono installate e consentite                                                      | L'abilitazione ha successo e scrive `true` per il plugin e ogni dipendenza che non era già abilitata all'ambito di destinazione. |

Questo vale anche quando una dipendenza imposta [`defaultEnabled: false`](/docs/it/plugins-reference#default-enablement) nel suo manifest, perché Claude Code scrive un `true` esplicito per essa. Lo stesso vale all'installazione: una dipendenza richiamata per soddisfare un plugin attivo si installa con `true` indipendentemente dal suo valore predefinito.

Quando disabiliti un plugin, Claude Code rifiuta se un altro plugin abilitato ne dipende ancora. L'errore nomina i plugin che ne dipendono e ti fornisce un comando concatenato che li disabilita nell'ordine corretto, terminando con quello che hai richiesto.

Ad esempio, se `deploy-kit` dipende da `secrets-vault`, la disabilitazione di `secrets-vault` da sola fallisce con un output simile al seguente:

```text theme={null}
secrets-vault is still required by deploy-kit. Disable that plugin first, or
disable everything together: claude plugin disable deploy-kit@acme-tools && claude plugin disable secrets-vault@acme-tools
```

Copia il comando concatenato dall'errore per disabilitare l'intero set in un unico passaggio.

<h2 id="remove-orphaned-auto-installed-dependencies">
  Rimuovi le dipendenze auto-installate orfane
</h2>

Le dipendenze auto-installate rimangono su disco dopo che i plugin che le hanno installate vengono disinstallati, nel caso in cui tu voglia reinstallare un plugin dipendente o desideri continuare a utilizzare la dipendenza direttamente. Per pulirle, esegui `claude plugin prune` per elencare le dipendenze auto-installate che non hanno più alcun plugin installato che le richiede e rimuoverle dopo un prompt di conferma. Questo richiede Claude Code v2.1.121 o successivo.

```bash theme={null}
claude plugin prune
```

Per impostazione predefinita, prune opera a livello di utente. Usa `--scope project` o `--scope local` per indirizzare un ambito diverso. Passa `--dry-run` per elencare cosa verrebbe rimosso senza modificare nulla. Passa `-y` per saltare il prompt di conferma. Quando stdin o stdout non è un terminale, prune elenca gli orfani e esce senza rimuoverli a meno che non venga passato `-y`.

Per eseguire prune come parte di una disinstallazione, passa `--prune` a `claude plugin uninstall`. Dopo aver rimosso il plugin denominato, Claude Code scansiona e rimuove eventuali dipendenze auto-installate che sono ora orfane. I plugin che hai installato tu stesso non vengono mai eliminati, solo quelli installati automaticamente attraverso l'array `dependencies` di un altro plugin.

Ad esempio, per disinstallare `deploy-kit` e pulire le dipendenze che lascia dietro:

```bash theme={null}
claude plugin uninstall deploy-kit --prune
```

<h2 id="resolve-dependency-errors">
  Risolvi gli errori delle dipendenze
</h2>

I problemi di dipendenza emergono in `claude plugin list` e nell'interfaccia `/plugin`. Claude Code disabilita il plugin interessato fino a quando non risolvi l'errore. La tabella seguente elenca gli errori più comuni e come risolverli.

| Errore                           | Significato                                                                                                                                                                                                                                                                       | Come risolvere                                                                                                                                                                                                                                                                           |
| :------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `dependency-unsatisfied`         | Una dipendenza dichiarata non è installata, oppure è installata ma disabilitata.                                                                                                                                                                                                  | Esegui il comando `claude plugin install` mostrato nel messaggio di errore. Se il marketplace della dipendenza non è ancora configurato, aggiungilo con `claude plugin marketplace add` e Claude Code risolve la dipendenza automaticamente. Se la dipendenza è disabilitata, abilitala. |
| `range-conflict`                 | I requisiti di versione per una dipendenza non possono essere combinati. Il messaggio di errore nomina la causa: nessuna versione soddisfa tutti gli intervalli, un intervallo non è una sintassi semver valida, o gli intervalli combinati sono troppo complessi da intersecare. | Disinstalla o aggiorna uno dei plugin in conflitto, correggi qualsiasi stringa `version` non valida, semplifica le catene `\|\|` lunghe, o chiedi all'autore upstream di ampliare il suo vincolo.                                                                                        |
| `dependency-version-unsatisfied` | La versione della dipendenza installata è al di fuori dell'intervallo dichiarato di questo plugin.                                                                                                                                                                                | Esegui `claude plugin install <dependency>@<marketplace>` per ri-risolvere la dipendenza rispetto a tutti i vincoli attuali.                                                                                                                                                             |
| `no-matching-tag`                | Il repository della dipendenza non ha un tag `{name}--v*` che soddisfa l'intervallo.                                                                                                                                                                                              | Verifica che l'upstream abbia taggato i rilasci usando la convenzione sopra, o rilassa il tuo intervallo.                                                                                                                                                                                |

Per controllare questi errori a livello di programmazione, esegui `claude plugin list --json` e leggi il campo `errors` su ogni plugin.

<h2 id="see-also">
  Vedi anche
</h2>

* [Crea plugin](/docs/it/plugins): costruisci plugin con skills, agent e hooks
* [Crea e distribuisci un marketplace di plugin](/docs/it/plugin-marketplaces): ospita plugin per il tuo team
* [Riferimento dei plugin](/docs/it/plugins-reference#plugin-manifest-schema): lo schema completo di `plugin.json`
* [Gestione delle versioni](/docs/it/plugins-reference#version-management): come la versione di un plugin viene risolta e utilizzata come chiave di cache
