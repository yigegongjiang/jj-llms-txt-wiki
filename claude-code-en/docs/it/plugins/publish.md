> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pubblica e distribuisci un plugin

> Pubblica un plugin Claude Code attraverso il tuo marketplace personale o il marketplace della comunità di Anthropic, con una checklist di pre-rilascio e come gli utenti ricevono gli aggiornamenti.

Pubblicare un plugin Claude Code significa inserirlo in un marketplace, un catalogo JSON che elenca i plugin e dove recuperare ciascuno di essi, in modo che altre persone possano installarlo per nome e ricevere i tuoi aggiornamenti. Puoi gestire il tuo marketplace personale o inviare il tuo plugin al marketplace della comunità di Anthropic. Per condividere un plugin senza pubblicarlo, invia alle persone la directory del plugin o un `.zip` di esso da caricare da soli.

Questa pagina è per l'autore di un plugin funzionante che è pronto a condividerlo.

<Note>
  Questi casi sono trattati su altre pagine:

  * **Il tuo plugin non è ancora finito**: inizia con [Crea un plugin](/docs/it/plugins/create)
  * **Mantieni una CLI o SDK con un plugin in un marketplace ufficiale**: vedi [Consiglia il tuo plugin dalla tua CLI](/docs/it/plugins/cli-hints)
</Note>

Inizia con [Scegli come distribuire](#choose-how-to-distribute) per confrontare le opzioni di distribuzione. Se conosci già il tuo percorso, vai a [Prepara il tuo plugin per il rilascio](#prepare-your-plugin-for-release), quindi segui la sezione del tuo percorso per sapere cosa dire ai tuoi utenti e come ricevono i tuoi aggiornamenti.

<h2 id="choose-how-to-distribute">
  Scegli come distribuire
</h2>

Scegli un'opzione di distribuzione in base a chi ha bisogno di installare il plugin:

| Percorso                                                                        | Chi può installare                                                                               | Cosa ti serve                                                                                    | Gli utenti ricevono i tuoi aggiornamenti automaticamente? |
| :------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- | :-------------------------------------------------------- |
| [Nessun marketplace](#share-a-plugin-without-a-marketplace)                     | Le persone a cui invii la cartella del plugin o un `.zip` di esso                                | La cartella del plugin                                                                           | Nessuno. Caricano la copia che hai inviato                |
| [Il tuo marketplace personale](#publish-through-your-own-marketplace)           | Chiunque possa raggiungere il repository, che può essere uno privato che il tuo team può clonare | Un repository git o altro host con un `.claude-plugin/marketplace.json` che elenca il tuo plugin | Disattivato                                               |
| [Marketplace della comunità di Anthropic](#submit-to-the-community-marketplace) | Chiunque aggiunga `anthropics/claude-plugins-community`                                          | Un invio attraverso il modulo di invio della directory dei plugin                                | Disattivato                                               |

L'aggiornamento automatico è un'impostazione per marketplace sul lato dell'utente che recupera le nuove versioni in background.

<h2 id="prepare-your-plugin-for-release">
  Prepara il tuo plugin per il rilascio
</h2>

Il nome, la versione, la convalida e un'installazione da un marketplace decidono se un rilascio funziona per le persone che lo installano. Controllali prima del primo rilascio e di nuovo prima di ogni rilascio successivo.

<Steps>
  <Step title="Scegli un nome permanente">
    Gli utenti installano, abilitano e configurano il tuo plugin per `name@marketplace`, quindi un plugin rinominato è un plugin diverso per ogni installazione esistente. Scegli un nome in kebab-case come `deploy-helper`, perché `claude plugin validate` avverte su altre forme, e trattalo come permanente. Imposta `displayName` in `plugin.json` per l'etichetta che gli utenti vedono.
  </Step>

  <Step title="Decidi come versione">
    Se imposti `version` in `plugin.json` e successivamente esegui il push di commit senza modificarla, `claude plugin update` stampa `<name> is already at the latest version (1.0.0).` e gli utenti mantengono la copia precedente. Incrementa `version` ad ogni rilascio, oppure omettila in un marketplace ospitato su git in modo che Claude Code utilizzi invece lo SHA del commit. Vedi [Versioni e aggiornamenti](/docs/it/plugins/loading#versions-and-updates).
  </Step>

  <Step title="Convalida">
    Nel tuo shell, esegui `claude plugin validate --strict ./your-plugin`. Un'esecuzione pulita stampa `✔ Validation passed`.

    * **In CI**: mantieni `--strict`, che inoltre fa fallire l'esecuzione con codice di uscita 1 su avvisi come un campo manifest sconosciuto o una `version` mancante. Rimuovi `--strict` se hai scelto di omettere `version` nel passaggio precedente.
    * **Percorsi**: la convalida segnala i percorsi dei componenti che non iniziano con `./`. All'interno dei comandi hook e delle configurazioni del server MCP, fai riferimento ai file come `${CLAUDE_PLUGIN_ROOT}/...`. Vedi [regole dei percorsi](/docs/it/plugins/manifest-reference#path-rules).
  </Step>

  <Step title="Installalo da un marketplace locale">
    Nel tuo shell, aggiungi un marketplace locale che elenca il plugin con `claude plugin marketplace add ./path-to-marketplace`, installa il plugin da esso e avvia una sessione per confermare che si carica.

    * Per il marketplace più piccolo che funziona, vedi [Crea un marketplace](/docs/it/plugins/create-marketplace).
    * Per sapere se un'installazione carica la tua directory di origine o una copia in cache, vedi [Plugin in-place e copiati](/docs/it/plugins/loading#in-place-and-copied-plugins).
  </Step>

  <Step title="Compila i metadati che gli utenti vedono">
    Imposta `description`, `author`, `homepage` e `repository` in `plugin.json`, e aggiungi un `README.md` alla radice del plugin. `homepage` deve essere analizzato come un URL. Il [riferimento manifest](/docs/it/plugins/manifest-reference#fields) elenca ogni campo.
  </Step>

  <Step title="Esegui la tua suite di eval">
    Se hai una suite di eval, esegui `claude plugin eval` nel tuo shell. Esegue i casi di test del plugin e valuta i risultati, il che cattura le regressioni quando modifichi il plugin. Vedi [Testa i plugin con eval](/docs/it/plugin-evals).
  </Step>
</Steps>

<h2 id="share-a-plugin-without-a-marketplace">
  Condividi un plugin senza un marketplace
</h2>

Se il plugin è in un repository git, le persone possono clonarlo e caricare il checkout, oppure avviare Claude Code dal loro shell con `--plugin-url` puntato a un `.zip` che allega a un rilascio. Per ottenere la tua prossima versione, eseguono il pull o scaricano di nuovo. Se non è in un repository, invia loro la directory o un `.zip` di esso. Lo caricano in uno di due modi:

* **Per una sessione**: avviano Claude Code dal loro shell con `claude --plugin-dir ./deploy-helper`, dove il percorso è il clone, la cartella decompressa o il `.zip` stesso. Vedi [Flag che caricano un plugin per una sessione](/docs/it/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
* **Per ogni sessione**: spostano la directory del plugin, con il suo `.claude-plugin/plugin.json`, sotto `~/.claude/skills/` in modo che Claude Code lo [carichi in ogni sessione](/docs/it/plugins/loading#find-where-a-plugin-came-from).

Aggiungere un `.claude-plugin/marketplace.json` allo stesso repository è ciò che consente alle persone di installare per nome e aggiornare con un comando; vedi [Pubblica attraverso il tuo marketplace personale](#publish-through-your-own-marketplace).

<h3 id="ship-a-plugin-with-your-own-tool">
  Spedisci un plugin con il tuo strumento personale
</h3>

Se mantieni una CLI o SDK, pubblica il plugin in un marketplace e fai in modo che il tuo programma di installazione o il messaggio post-installazione esegua o stampi i due comandi di cui un utente ha bisogno: `claude plugin marketplace add <source>`, quindi `claude plugin install <name>@<marketplace>`. Per la scoperta in-sessione quando qualcuno utilizza il tuo strumento, vedi [Consiglia il tuo plugin dalla tua CLI](/docs/it/plugins/cli-hints).

<h2 id="publish-through-your-own-marketplace">
  Pubblica attraverso il tuo marketplace personale
</h2>

Il tuo marketplace personale è un file `.claude-plugin/marketplace.json` che elenca il tuo plugin, aggiunto a un repository git. Una volta che il file è nel repository, il plugin è pubblicato, senza modulo di invio. Puoi mantenere il file nel repository del plugin stesso o in uno separato.

<h3 id="add-the-marketplace-file-to-your-repository">
  Aggiungi il file marketplace al tuo repository
</h3>

Per pubblicare dal repository del plugin stesso, salva il file marketplace accanto a `plugin.json` in `.claude-plugin/`, con una voce il cui `source` è `"./"`, la radice del repository. Dai alla voce lo stesso `name` di `plugin.json`, secondo [Mantieni il nome della voce e il nome del manifest uguali](/docs/it/plugins/create-marketplace#keep-the-entry-name-and-the-manifest-name-the-same):

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

Nel tuo shell, esegui `claude plugin validate .` nel repository per controllare il file prima di eseguire il push.

[Crea un marketplace](/docs/it/plugins/create-marketplace) copre il layout con diversi plugin in un repository.

<h3 id="control-who-can-install">
  Controlla chi può installare
</h3>

Chiunque possa clonare il repository può installare da esso, quindi se il repository è privato, anche il marketplace è privato. Per host diversi da un repository git, vedi [Ospita un marketplace](/docs/it/plugins/host-marketplace). Per raggiungere tutti in un'azienda, incluse le persone che non usano git, vedi [Distribuisci a un'intera azienda](/docs/it/plugins/host-marketplace#roll-out-to-a-whole-company).

<h3 id="tell-users-how-to-install">
  Comunica agli utenti come installare
</h3>

Comunica ai tuoi utenti di aggiungere il marketplace e quindi installare il plugin dal loro shell, sostituendo la fonte e i nomi con i tuoi:

* Aggiungi il marketplace una volta: `claude plugin marketplace add your-org/your-marketplace`, dove l'argomento è una scorciatoia GitHub `owner/repo`, un URL o un percorso
* Installa il plugin: `claude plugin install deploy-helper@your-marketplace`
* Oppure fai entrambi da dentro una sessione: `/plugin install deploy-helper --marketplace your-org/your-marketplace`. Richiede Claude Code v2.1.275 o successivo. Vedi [Aggiungi un marketplace e installa in un comando](/docs/it/plugins/install#add-a-marketplace-and-install-in-one-command)

<h3 id="ship-updates-to-users">
  Spedisci aggiornamenti agli utenti
</h3>

Gli utenti ricevono un rilascio quando lo chiedono o quando l'aggiornamento automatico è attivato per il tuo marketplace:

* **Su richiesta**: `claude plugin update deploy-helper@your-marketplace` nel shell dell'utente aggiorna il marketplace e installa la nuova copia quando la versione del tuo plugin è cambiata
* **Aggiornamento automatico**: disattivato per impostazione predefinita per il tuo marketplace. Vedi [Attiva l'aggiornamento automatico](/docs/it/plugins/host-marketplace#turn-on-auto-update). Una volta attivato, fa lo stesso di `claude plugin update` con un ritardo dopo l'avvio della sessione

[Installa plugin](/docs/it/plugins/install) copre i comandi lato utente, e [quando viene eseguito l'aggiornamento automatico](/docs/it/plugins/loading#when-auto-update-runs) copre i tempi.

<h2 id="submit-to-the-community-marketplace">
  Invia al marketplace della comunità
</h2>

Il marketplace della comunità di Anthropic, `claude-community`, è il marketplace pubblico che elenca i plugin inviati attraverso il modulo di invio della directory dei plugin.

Gli utenti aggiungono il marketplace della comunità in una sessione Claude Code con `/plugin marketplace add anthropics/claude-plugins-community` e installano da esso come `@claude-community`.

Per come il marketplace della comunità differisce dal marketplace ufficiale, vedi [Marketplace di Anthropic](/docs/it/plugins/anthropic-marketplaces).

Per inviare il tuo plugin al marketplace della comunità, utilizza uno dei moduli in-app:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Il modulo claude.ai richiede un'organizzazione Team o Enterprise e l'autorizzazione Directory, che i Proprietari detengono per impostazione predefinita. Gli autori individuali che non fanno parte di un'organizzazione Team o Enterprise possono utilizzare il modulo Console.

Nel tuo shell, esegui `claude plugin validate ./your-plugin` localmente prima di inviare, sostituendo `./your-plugin` con il percorso della tua directory plugin. Quando la convalida passa, Claude Code stampa `✔ Validation passed`, o `✔ Validation passed with warnings` se ci sono avvisi. Gli avvisi non fanno fallire la convalida; aggiungi `--strict` per trattarli come errori.

I plugin elencati appaiono nel catalogo [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), nella maggior parte dei casi fissati a uno SHA di commit specifico.

Può esserci un ritardo tra l'invio e la comparsa del tuo plugin in `marketplace.json`. Per verificare se il tuo plugin è già installabile, cerca il suo nome nel [catalogo della comunità](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

Il marketplace ufficiale, `claude-plugins-official`, non accetta invii attraverso questi moduli. Se lavori con un contatto partner di Anthropic, chiedi loro informazioni su un'inserzione nel marketplace ufficiale.

<h2 id="ship-updates-renames-and-removals">
  Spedisci aggiornamenti, ridenominazioni e rimozioni
</h2>

<h3 id="release-a-new-version">
  Rilascia una nuova versione
</h3>

Se pubblichi attraverso il tuo marketplace personale e il tuo `plugin.json` imposta `version`, incrementala e esegui il push. Gli utenti che eseguono `claude plugin update` o hanno l'aggiornamento automatico attivato ricevono la nuova versione, come descritto in [Spedisci aggiornamenti agli utenti](#ship-updates-to-users).

<h3 id="tag-a-release">
  Etichetta un rilascio
</h3>

Etichetta il rilascio in git quando altri plugin dichiarano un intervallo di versione sul tuo, perché quegli intervalli si risolvono rispetto ai tag. Altrimenti non hai bisogno di un tag.

Per etichettare, esegui `claude plugin tag` nel tuo shell dalla directory del plugin. Crea un tag `{name}--v{version}`. Aggiungi `--push` per inviare il tag a `origin`. Il [riferimento `plugin tag`](/docs/it/plugins/cli-reference#plugin-tag) elenca i suoi flag.

<h3 id="rename-or-remove-a-plugin">
  Rinomina o rimuovi un plugin
</h3>

Non modificare mai il `name` di un plugin pubblicato. Dopo una ridenominazione, gli utenti che lo hanno già installato perdono il plugin, perché la loro installazione è registrata sotto il vecchio nome. Una voce `renames` nel tuo file marketplace li migra invece. Cambia `displayName` quando vuoi un'etichetta diversa.

Se una ridenominazione è inevitabile, utilizza la mappa `renames` del file marketplace in modo che le installazioni esistenti migrino invece di fallire con [`Plugin "<name>" not found in marketplace`](/docs/it/plugins/troubleshooting#plugin-not-found-in-marketplace). Per rimuovere un plugin dal marketplace, o per i dettagli completi di `renames`, vedi [Rinomina o rimuovi un plugin](/docs/it/plugins/host-marketplace#rename-or-remove-a-plugin) nella pagina di hosting. Il [riferimento marketplace](/docs/it/plugins/marketplace-reference#top-level-fields) ha il campo.

<h2 id="declare-dependencies">
  Dichiara dipendenze
</h2>

Se il tuo plugin ha bisogno di un altro plugin dello stesso marketplace per essere abilitato, elencalo nell'array `dependencies` di `plugin.json`. Ogni voce è un nome semplice o un oggetto con un intervallo di versione semver. Quando un utente installa il tuo plugin, Claude Code installa e abilita anche la dipendenza.

[Dipendenze dei plugin](/docs/it/plugins/dependencies) copre la sintassi dell'intervallo, le dipendenze tra marketplace e come gli utenti potano le dipendenze di cui non hanno più bisogno.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Ospita e mantieni un marketplace](/docs/it/plugins/host-marketplace): rilascia nuove versioni e mantieni gli utenti aggiornati
* [Dipendenze dei plugin](/docs/it/plugins/dependencies): dichiara e versiona i plugin su cui il tuo dipende
* [Consiglia il tuo plugin dalla tua CLI](/docs/it/plugins/cli-hints): invita gli utenti Claude Code della tua CLI a installare il plugin
* [Misura il costo e l'utilizzo del plugin](/docs/it/plugins/measure): vedi quanto costa il tuo plugin nel contesto e se le persone lo usano
