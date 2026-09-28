> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare la modalità auto

> Comunica al classificatore della modalità auto quali repository, bucket e domini la tua organizzazione ritiene affidabili. Imposta il contesto dell'ambiente, sostituisci le regole di blocco e autorizzazione predefinite e ispeziona la tua configurazione effettiva con i sottocomandi CLI della modalità auto.

[Auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) consente a Claude Code di funzionare senza prompt di autorizzazione instradando ogni chiamata di strumento attraverso un classificatore che blocca qualsiasi cosa irreversibile, distruttiva o rivolta al di fuori del tuo ambiente. Le regole di negazione e richiesta esplicita vengono valutate prima del classificatore e continuano comunque a bloccare o richiedere. Utilizza il blocco di impostazioni `autoMode` per comunicare al classificatore quali repository, bucket e domini la tua organizzazione ritiene affidabili, in modo che smetta di bloccare le operazioni interne di routine.

<Note>
  Auto mode è disponibile a tutti gli utenti su ogni provider, inclusa l'API Anthropic, [Claude Platform su AWS](/docs/it/claude-platform-on-aws), Amazon Bedrock, Agent Platform di Google Cloud, Microsoft Foundry e sessioni [gateway di app Claude](/docs/it/claude-apps-gateway) con accesso. Se Claude Code segnala che la modalità auto non è disponibile per il tuo account, controlla i [requisiti completi](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), che coprono anche i modelli supportati e il controllo a livello di organizzazione sui piani Team ed Enterprise. Nella versione da v2.1.158 a v2.1.206, la modalità auto su Amazon Bedrock, Agent Platform di Google Cloud, Microsoft Foundry e sessioni gateway di app Claude richiedeva l'impostazione di `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 ha rimosso il requisito.
</Note>

Per impostazione predefinita, il classificatore si fida solo della directory di lavoro e dei remote configurati del repository corrente. Azioni come il push verso l'organizzazione di controllo del codice sorgente della tua azienda o la scrittura in un bucket cloud del team vengono bloccate finché non le aggiungi a `autoMode.environment`.

Per informazioni su come le sessioni finiscono in modalità auto e cosa blocca il classificatore per impostazione predefinita, consulta [auto mode nella pagina Permission modes](/docs/it/permission-modes#eliminate-prompts-with-auto-mode). Questa pagina è il riferimento di configurazione.

Questa pagina spiega come:

* [Aggiungere un checkpoint umano](#add-a-human-checkpoint) per push e pull request con `permissions.ask`
* [Scegliere dove impostare le regole](#where-the-classifier-reads-configuration) in CLAUDE.md, impostazioni utente e impostazioni gestite
* [Definire l'infrastruttura affidabile](#define-trusted-infrastructure) con `autoMode.environment`
* [Generare voci di ambiente](#generate-environment-entries) con `/auto-mode-setup`
* [Sostituire le regole di blocco e autorizzazione](#override-the-block-and-allow-rules) quando i valori predefiniti non si adattano alla tua pipeline
* [Modificare le regole da `/permissions`](#edit-rules-from-permissions) senza aprire un file di impostazioni
* [Instradare tutti i comandi shell attraverso il classificatore](#route-all-shell-commands-through-the-classifier) con `autoMode.classifyAllShell`
* [Ispezionare la tua configurazione effettiva](#inspect-the-defaults-and-your-effective-config) con i sottocomandi `claude auto-mode`
* [Esaminare i rifiuti](#review-denials) in modo da sapere cosa aggiungere successivamente

<h2 id="common-boundaries">
  Confini comuni
</h2>

Auto mode consente push a qualsiasi ramo del repository in cui stai lavorando, incluso il ramo predefinito, e la creazione di pull request per impostazione predefinita. Un ramo non predefinito il cui nome lo contrassegna come bersaglio di distribuzione o pubblicazione, come `production`, `release` o `gh-pages`, non è coperto da quel valore predefinito: il classificatore giudica un push lì in base ai suoi meriti, incluso come distribuzione di produzione. Il contenuto del push viene comunque controllato, quindi un force push, un segreto che entra nel commit o una modifica che invierebbe segreti al di fuori del repository quando CI o una pipeline di distribuzione lo esegue rimane bloccato.

<Info>Prima di v2.1.211, il classificatore consentiva push solo al tuo ramo di lavoro, ai rami creati da Claude e ai push di routine al ramo predefinito.</Info>

Se desideri un checkpoint umano prima dei comandi push e pull request di Claude, aggiungi regole di autorizzazione: le [ricette di seguito](#add-a-human-checkpoint) mantengono la modalità auto attiva per tutto il resto.

<h3 id="add-a-human-checkpoint">
  Aggiungere un checkpoint umano
</h3>

Il meccanismo più diretto è [`permissions.ask`](/docs/it/permissions#permission-rule-syntax). Le regole di richiesta con ambito di contenuto come quelle di seguito vengono valutate prima del classificatore e forzano sempre un prompt di autorizzazione, anche in modalità auto, perché una regola di richiesta esplicita è la tua intenzione dichiarata di essere richiesto per quell'azione. Aggiungi le regole nelle tue [impostazioni](/docs/it/settings#where-settings-live):

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

Questi regole corrispondono ai comandi che iniziano con `git push` o `gh pr create`. Un push che Claude scrive in un altro modo, come `git -C <dir> push` o `git -c <key>=<value> push`, [non corrisponde alla regola](/docs/it/permissions#bash-rule-limits), quindi non viene sottoposto a checkpoint. Per un checkpoint che ispeziona il testo completo del comando, aggiungi un [hook PreToolUse](/docs/it/hooks#pretooluse).

Scegli il meccanismo che corrisponde a quanto ferma deve essere la limitazione:

| Confine                                | Meccanismo                                                             | Comportamento in modalità auto                                                                                                                                                                                                                                        |
| :------------------------------------- | :--------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Richiedi prima dell'azione             | `permissions.ask`                                                      | Sempre richiede per un comando che corrisponde a una regola con ambito di contenuto come la ricetta di cui sopra. Il classificatore non può approvare automaticamente un'azione corrispondente.                                                                       |
| Non eseguire mai l'azione              | `permissions.deny`                                                     | Blocca prima che il classificatore venga consultato. Né il classificatore né l'intento dell'utente possono ignorarlo.                                                                                                                                                 |
| Confine una tantum per questa sessione | Dichiaralo nella conversazione, come "non fare push finché non rivedo" | Il classificatore blocca le azioni corrispondenti, ma il confine può andare perso se la [compattazione del contesto](/docs/it/costs#reduce-token-usage) rimuove il messaggio che lo ha dichiarato. Utilizza una regola di richiesta o negazione per una garanzia duratura. |

<h2 id="where-the-classifier-reads-configuration">
  Dove il classificatore legge la configurazione
</h2>

Il classificatore legge lo stesso contenuto [CLAUDE.md](/docs/it/memory) che Claude stesso carica, quindi un'istruzione come "non forzare mai il push" nel CLAUDE.md del tuo progetto guida sia Claude che il classificatore contemporaneamente. Inizia da lì per le convenzioni del progetto e le regole comportamentali.

Per le regole che si applicano tra i progetti, come l'infrastruttura affidabile o le regole di negazione a livello di organizzazione, utilizza il blocco di impostazioni `autoMode`. Il classificatore legge `autoMode` dai seguenti ambiti:

| Ambito                        | File                                            | Utilizzare per                                                 |
| :---------------------------- | :---------------------------------------------- | :------------------------------------------------------------- |
| Un sviluppatore               | `~/.claude/settings.json`                       | Infrastruttura affidabile personale                            |
| A livello di organizzazione   | [Managed settings](/docs/it/server-managed-settings) | Infrastruttura affidabile distribuita a tutti gli sviluppatori |
| Flag `--settings` o Agent SDK | JSON inline                                     | Override per invocazione per l'automazione                     |

Il classificatore non legge `autoMode` dalle impostazioni di progetto in `.claude/settings.json` o `.claude/settings.local.json`. Entrambi i file risiedono nella directory del repository, quindi un repository archiviato o un passaggio di compilazione potrebbe altrimenti iniettare le proprie regole di autorizzazione. Prima di v2.1.207, il classificatore leggeva anche `.claude/settings.local.json`; sposta qualsiasi blocco `autoMode` in quel file a `~/.claude/settings.json`. Escludere `.claude/settings.local.json` chiude anche il caso in cui un repository esegue il commit del file o uno strumento locale o un passaggio di compilazione lo scrive.

Le voci di ogni ambito vengono combinate. Uno sviluppatore può estendere `environment`, `allow`, `soft_deny` e `hard_deny` con voci personali ma non può rimuovere le voci fornite dalle impostazioni gestite. Poiché le regole di autorizzazione agiscono come eccezioni alle regole di blocco morbido all'interno del classificatore, una voce `allow` aggiunta da uno sviluppatore può sostituire una voce `soft_deny` dell'organizzazione: la combinazione è additiva, non un confine di politica rigida.

<Note>
  Il classificatore è una seconda porta che si esegue dopo il [sistema di autorizzazioni](/docs/it/permissions). Per le azioni che non devono mai essere eseguite indipendentemente dall'intento dell'utente o dalla configurazione del classificatore, utilizza `permissions.deny` nelle impostazioni gestite, che blocca l'azione prima che il classificatore venga consultato e non può essere ignorato.
</Note>

<h2 id="define-trusted-infrastructure">
  Definire l'infrastruttura affidabile
</h2>

Per la maggior parte delle organizzazioni, `autoMode.environment` è l'unico campo che devi impostare. Comunica al classificatore quali repository, bucket e domini sono affidabili: il classificatore lo utilizza per decidere cosa significa "esterno", quindi qualsiasi destinazione non elencata è un potenziale bersaglio di esfiltrazione.

A partire da Claude Code v2.1.198, `claude auto-mode defaults` stampa tre tipi di voce di ambiente. Le versioni precedenti a v2.1.195 stampano solo i primi cinque slot di fiducia.

* **Context slots**: descrivono la tua organizzazione, stack e postura di sicurezza in modo che il classificatore legga le altre regole nel tuo contesto. Ognuno predefinito è `None configured` o all'assunzione conservativa denominata accanto ad esso:
  * **Organization**
  * **Primary use of Claude Code**: predefinito per lo sviluppo software
  * **Cloud provider(s)**
  * **Repository visibility**: un repository è assunto privato a meno che il suo host remoto e il nome non indichino diversamente, o il classificatore legga un controllo di visibilità precedente nella conversazione che mostra che è pubblico.

    Nelle richieste del classificatore inviate da Claude Code stesso, il classificatore legge i tuoi messaggi e i comandi che Claude esegue, non il loro output. L'evidenza deve essere qualcosa che il classificatore può leggere, come il tuo stesso messaggio che nomina il repository come pubblico; l'output di un `gh repo view` da solo non lo raggiunge. Il controllo delle prove della trascrizione richiede Claude Code v2.1.200 o successivo
  * **Internal sharing / snippet hosting**: i servizi di paste e gist pubblici sono trattati come esterni al confine di fiducia finché non ne nomini uno
  * **Org-specific CLIs**
  * **Secrets management**
  * **CI/CD deploy targets**
  * **Network posture**
  * **Host containment**: predefinito a una macchina sviluppatore ordinaria o runner CI con internet aperto. Se Claude Code viene eseguito in un container, VM o pod con un allow-list di uscita o vicini che non deve toccare, nomina gli host consentiti, se l'endpoint dei metadati cloud dovrebbe essere raggiungibile e quale progetto cloud, cluster o registro il compito utilizza e sotto quale identità. Finché questa voce non nomina quell'identità, il classificatore [blocca](/docs/it/permission-modes#what-the-classifier-blocks-by-default) le richieste per le credenziali dell'host stesso. Richiede Claude Code v2.1.257 o successivo
  * **Protected deployment namespaces / environments**: ricade all'euristica Sensitive remote targets finché non li nomini
  * **Data retention / declassification**
* **Trust slots**: denominano ciò che il classificatore tratta come interno al tuo confine. Gli slot sono Trusted repo, Source control, Trusted internal domains, Trusted cloud buckets, Key internal services e Internal package registry. Le voci di repository e controllo del codice sorgente predefinite sono il repository di lavoro e i suoi remote configurati. Ogni altro slot di fiducia predefinito è `None configured`, quindi nient'altro è affidabile finché non lo aggiungi. La visibilità di un repository limita solo il materiale confidenziale: un repository privato è una destinazione accettabile per il materiale confidenziale, ma rendere un repository privato non cancella mai i segreti o i dati personali o affidati in esso, e il classificatore tratta il contenuto trasportato, reindirizzato o letto per la prima volta dall'esterno del repository di lavoro come non il lavoro proprio di quel repository. Questo ambito richiede Claude Code v2.1.203 o successivo.
* **Sensitivity slots**: denominano ciò che le regole protettive trattano come ad alto rischio. Gli slot sono Sensitive data locations & audiences, Sensitive remote targets e Protected IaC scopes. Ognuno predefinito è un'euristica ampia, come trattare qualsiasi host o namespace il cui nome contiene `prod` o `production` come bersaglio remoto sensibile, quindi le regole protettive sono attive prima di configurare qualsiasi cosa. Denominare bersagli concreti in uno slot di sensibilità fa sì che quelle regole si applichino ai bersagli denominati invece dell'euristica.

<Info>Prima di v2.1.211, gli slot di contesto includevano anche una voce Default / protected branches che trattava `main` e `master` come protetti finché non ne nominavi altri. v2.1.211 l'ha rimossa: i [push a qualsiasi ramo del repository in cui stai lavorando](#common-boundaries) sono consentiti per impostazione predefinita, quindi non c'è un valore predefinito di ramo protetto da configurare.</Info>

Per aggiungere le tue voci insieme ai valori predefiniti, includi la stringa letterale `"$defaults"` nell'array. Le voci predefinite vengono inserite in quella posizione, quindi le tue voci personalizzate possono andare prima o dopo di esse.

L'esempio seguente mantiene le voci predefinite e aggiunge i repository, i bucket, i domini e i servizi di un'organizzazione.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

Dopo aver salvato le tue impostazioni, esegui `claude auto-mode config` per [confermare che le regole effettive](#inspect-the-defaults-and-your-effective-config) includono le tue voci.

Le voci sono prosa, non regex o pattern di strumenti. Il classificatore le legge come regole in linguaggio naturale. Scrivile come descriveresti la tua infrastruttura a un nuovo ingegnere. Una sezione di ambiente completa copre:

* **Organization**: il nome della tua azienda e per cosa Claude Code viene utilizzato principalmente, come sviluppo software, automazione dell'infrastruttura o ingegneria dei dati
* **Source control**: ogni organizzazione GitHub, GitLab o Bitbucket verso cui i tuoi sviluppatori eseguono il push
* **Cloud providers and trusted buckets**: nomi di bucket o prefissi che Claude dovrebbe essere in grado di leggere e scrivere
* **Trusted internal domains**: nomi host per API, dashboard e servizi all'interno della tua rete, come `*.internal.example.com`
* **Key internal services**: CI, registri di artefatti, indici di pacchetti interni, strumenti di gestione degli incidenti
* **Internal package registry**: il registro npm, PyPI o altro privato attraverso il quale gli install dovrebbero instradare, in modo che gli install che lo bypassano per un registro pubblico vengano bloccati
* **Sensitive data locations & audiences**: i bucket, i database o i percorsi che contengono dati personali, dati aziendali confidenziali, credenziali, dati regolamentati o materiale simile sensibile, e i destinatari con cui i dati in ogni posizione possono essere condivisi, in modo che il classificatore protegga quelle posizioni invece di indovinare dal contenuto. Claude Code v2.1.195 attraverso v2.1.197 denominano questa voce PII / regulated-data locations e coprono solo le posizioni che contengono dati personali o regolamentati, senza la dimensione del destinatario
* **Sensitive remote targets**: gli spazi dei nomi, gli host o i container che contano come produzione, in modo che i shell remoti e i port-forward in essi richiedano la tua approvazione esplicita
* **Protected IaC scopes**: le risorse di infrastruttura il cui apply o destroy dovrebbe sempre richiedere di denominare il cambiamento
* **Additional context**: vincoli del settore regolamentato, infrastruttura multi-tenant o requisiti di conformità che influiscono su ciò che il classificatore dovrebbe trattare come rischioso

Le voci Internal package registry, Sensitive data locations & audiences, Sensitive remote targets e Protected IaC scopes richiedono Claude Code v2.1.195 o successivo. Le versioni precedenti le leggono ancora come contesto semplice ma non hanno le regole incorporate che le prendono di mira.

Un modello di partenza utile: compila i campi tra parentesi e rimuovi le righe che non si applicano.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

Più contesto specifico fornisci, meglio il classificatore può distinguere le operazioni interne di routine dai tentativi di esfiltrazione.

Non è necessario compilare tutto in una volta. Un rollout ragionevole: inizia con i valori predefiniti e aggiungi l'organizzazione di controllo del codice sorgente e i servizi interni chiave, che risolvono i falsi positivi più comuni come il push verso i tuoi repository. Aggiungi successivamente i domini affidabili e i bucket cloud. Compila il resto man mano che emergono i blocchi.

<h2 id="generate-environment-entries">
  Generare voci di ambiente con `/auto-mode-setup`
</h2>

Esegui `/auto-mode-setup` per fare in modo che Claude Code rediga voci `autoMode.environment` e talvolta anche voci di [regole](#override-the-block-and-allow-rules) dal tuo progetto e dalle tue sessioni recenti in esso. Se accetti la bozza, Claude Code la scrive in `~/.claude/settings.json`.

<Note>
  `/auto-mode-setup` richiede un piano Pro, Max o Team e Claude Code v2.1.228 o successivo. Su Windows nativo richiede v2.1.233 o successivo. Non puoi eseguirlo in una [sessione cloud](/docs/it/claude-code-on-the-web). Ha anche bisogno del [recupero dei flag di funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching), quindi non puoi eseguirlo in una sessione in cui hai disattivato il recupero dei flag.
</Note>

<h3 id="what-auto-mode-setup-reads">
  Cosa legge `/auto-mode-setup`
</h3>

Se `~/.claude/settings.json` contiene già voci `autoMode`, Claude Code inizia chiedendo se aggiungere alla tua lista di ambiente o sostituirla e mantiene le regole che hai scritto comunque. Claude Code quindi chiede come utilizzi questo progetto e offre due scansioni opzionali prima di scansionare qualsiasi cosa. Nella scansione, Claude Code legge sempre queste fonti:

* Il `CLAUDE.md`, `README.md`, i file di configurazione e i remote git di questo progetto
* Le tue impostazioni `autoMode` e `permissions.allow`
* Gli host, i bucket e i nomi dei comandi dai comandi che Claude ha eseguito nelle tue sessioni recenti in questo progetto, mai i tuoi messaggi

Le due scansioni opzionali aggiungono una fonte ciascuna:

* La prima parola di ogni comando nella tua cronologia della shell
* Gli host remoti e i nomi dei repository sotto la tua home directory

<h3 id="review-and-save-the-draft">
  Esaminare e salvare la bozza
</h3>

Claude Code esegue la scansione in background, quindi ti mostra la bozza. La accetti o la scarta nel complesso, quindi modifica `~/.claude/settings.json` in seguito per regolare le singole voci. Quando accetti, Claude Code scrive la bozza e la riconcilia con le impostazioni che hai già:

* Claude Code scrive la lista `environment` senza `"$defaults"`, perché la bozza esplicita le voci incorporate che ha lasciato invariate
* Claude Code include `"$defaults"` in ognuno degli elenchi `allow`, `soft_deny` e `hard_deny` a cui la bozza aggiunge voci, a meno che tu non abbia già scritto un elenco `allow` senza di esso, quindi le [regole incorporate](#override-the-block-and-allow-rules) che non hai sostituito rimangono in vigore
* Dopo il salvataggio, Claude Code offre di rimuovere le regole `permissions.allow` in `~/.claude/settings.json` che la modalità auto ignora, come `Bash(*)`, o che approvano automaticamente i comandi distruttivi

Quindi esegui `claude auto-mode config` per [vedere il risultato effettivo](#inspect-the-defaults-and-your-effective-config).

<h3 id="turn-off-auto-mode-setup">
  Disattivare `/auto-mode-setup`
</h3>

Una volta che la modalità auto ha bloccato diverse azioni e non hai ancora voci `autoMode.environment`, Claude Code mostra una finestra di dialogo intitolata "Teach auto mode about your environment?" alla fine di un turno e offre di eseguire `/auto-mode-setup` per te. Per interrompere l'offerta ma mantenere il comando, seleziona **Don't show again** in quella finestra di dialogo.

Per disattivare sia il comando che l'offerta, aggiungi questa voce [`skillOverrides`](/docs/it/skills#override-skill-visibility-from-settings) a `~/.claude/settings.json`:

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup` è un comando incorporato piuttosto che una [skill in bundle](/docs/it/skills#bundled-skills), quindi questa voce `skillOverrides` si applica comunque ad esso, ma [`disableBundledSkills`](/docs/it/settings-reference#disablebundledskills) non lo disattiva.

<h2 id="override-the-block-and-allow-rules">
  Sostituire le regole di blocco e autorizzazione
</h2>

Tre campi aggiuntivi ti permettono di sostituire gli elenchi di regole incorporate del classificatore:

* `autoMode.hard_deny`: confini di sicurezza incondizionati
* `autoMode.soft_deny`: azioni distruttive che l'intento dell'utente può annullare
* `autoMode.allow`: eccezioni alle regole di blocco soft

Ognuno è un array di descrizioni in prosa, lette come regole in linguaggio naturale. Per i blocchi duri basati su pattern di strumenti che vengono eseguiti prima del classificatore, utilizza [`permissions.deny`](/docs/it/permissions).

All'interno del classificatore, la precedenza funziona in quattro livelli:

* Le regole `hard_deny` bloccano incondizionatamente. L'intento dell'utente e le eccezioni `allow` non si applicano.
* Le regole `soft_deny` bloccano successivamente. L'intento dell'utente e le eccezioni `allow` possono ignorare questi blocchi.
* Le regole `allow` quindi sostituiscono i blocchi `soft_deny` corrispondenti come eccezioni.
* L'intento esplicito dell'utente ignora i blocchi soft rimanenti: se il messaggio dell'utente descrive direttamente e specificamente l'azione esatta che Claude sta per intraprendere, il classificatore la consente anche quando una regola `soft_deny` corrisponde.

Le richieste generali non contano come intento esplicito. Chiedere a Claude di "pulire il repository" non autorizza il force-push, ma chiedere a Claude di "force-push questo ramo" sì.

Per allentare, aggiungi a `allow` quando il classificatore contrassegna ripetutamente un pattern di routine che le eccezioni predefinite non coprono. Per stringere, aggiungi a `soft_deny` per i rischi distruttivi specifici del tuo ambiente che i valori predefiniti non coprono, o a `hard_deny` per i confini di sicurezza che non devono mai essere superati.

Per mantenere le regole incorporate mentre aggiungi le tue, includi la stringa letterale `"$defaults"` nell'array. Le regole predefinite vengono inserite in quella posizione, quindi le tue regole personalizzate possono andare prima o dopo di esse, e continui a ereditare gli aggiornamenti mentre l'elenco incorporato cambia tra le versioni.

L'esempio seguente mantiene i valori predefiniti in tutti e quattro gli elenchi e aggiunge regole specifiche dell'organizzazione a ognuno.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  L'impostazione di uno qualsiasi di `environment`, `allow`, `soft_deny` o `hard_deny` senza `"$defaults"` sostituisce l'intero elenco predefinito per quella sezione. Se imposti un array senza `"$defaults"`, scarta le regole incorporate per quella sezione:

  * `soft_deny`: ogni regola di blocco soft incorporata, inclusi force push, `curl | bash`, distribuzioni di produzione e bypass della modalità auto
  * `hard_deny`: la regola incorporata di esfiltrazione dei dati
</Danger>

Ogni sezione viene valutata indipendentemente, quindi l'impostazione di `environment` da sola lascia intatti gli elenchi predefiniti `allow`, `soft_deny` e `hard_deny`. Ometti `"$defaults"` solo quando intendi assumere la piena proprietà dell'elenco. Per farlo in modo sicuro, esegui `claude auto-mode defaults` per stampare le regole incorporate, copiale nel tuo file di impostazioni, quindi esamina ogni regola rispetto alla tua pipeline e tolleranza al rischio.

<h2 id="edit-rules-from-permissions">
  Modificare le regole da `/permissions`
</h2>

Per visualizzare e modificare le regole del classificatore senza aprire un file di impostazioni, esegui [`/permissions`](/docs/it/permissions#manage-permissions) e seleziona la scheda **Auto mode**. La scheda richiede Claude Code v2.1.246 o successivo e appare solo quando [la modalità auto è disponibile](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) per la tua sessione.

La scheda elenca le voci `allow`, `soft_deny`, `hard_deny` e `environment` da ognuno degli [ambiti che il classificatore legge](#where-the-classifier-reads-configuration) e mostra se le regole incorporate sono in vigore per ogni sezione. Claude Code mostra le voci dalle [impostazioni gestite](/docs/it/server-managed-settings) o dal flag `--settings` come di sola lettura e salva ogni modifica che fai sulla scheda in `~/.claude/settings.json`. Dalla scheda puoi:

* Aggiungere, modificare o eliminare regole nelle sezioni `allow`, `soft_deny` e `hard_deny`. Quando aggiungi la prima regola a una sezione, Claude Code inserisce anche `"$defaults"` in modo che le [regole incorporate](#override-the-block-and-allow-rules) rimangono in vigore.
* Attivare o disattivare le regole incorporate per `allow`, `soft_deny` o `hard_deny`. Claude Code registra la scelta aggiungendo o rimuovendo `"$defaults"` nel tuo elenco per quella sezione, quindi una sezione ha bisogno di almeno una regola propria prima di poter disattivare le sue regole incorporate.
* Modificare le voci `environment` come un documento nel tuo editor. Se non hai ancora configurato voci `environment`, Claude Code prima chiede se sostituire l'ambiente incorporato, quindi apre l'editor sul testo incorporato completo. Quando salvi, Claude Code sostituisce il tuo array `autoMode.environment` con il documento. Includi la riga `"$defaults"` per [mantenere le voci incorporate](#define-trusted-infrastructure).

<h2 id="route-all-shell-commands-through-the-classifier">
  Instradare tutti i comandi shell attraverso il classificatore
</h2>

Per impostazione predefinita, le regole di autorizzazione Bash e PowerShell strette come `Bash(npm test)` rimangono in vigore in modalità auto, e Claude Code le risolve prima che il classificatore venga eseguito, a meno che il comando non contenga [domini consentiti per comando](/docs/it/sandboxing#per-command-allowed-domains-in-auto-mode). Claude Code sospende solo le regole ampie che concedono l'esecuzione di codice arbitrario, come `Bash(*)` o interpreti con caratteri jolly, insieme a ogni regola che nomina [`Monitor`](/docs/it/tools-reference#monitor-tool), perché i comandi Monitor vengono eseguiti attraverso la shell. Ciò significa che una regola stretta può comunque far passare un argomento distruttivo senza che il classificatore lo veda, ad esempio un percorso di script o un flag che il prefisso della regola non ha anticipato.

Imposta `autoMode.classifyAllShell` su `true` per sospendere ogni regola di autorizzazione Bash e PowerShell mentre la modalità auto è attiva, in modo che il classificatore valuti ogni comando shell indipendentemente dal tuo elenco di autorizzazioni.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Questo scambia la latenza per la copertura: un comando che una regola di autorizzazione avrebbe approvato istantaneamente ora attende una decisione del classificatore, e ogni comando shell conta come una chiamata del classificatore.

L'impostazione si applica solo mentre la modalità auto è attiva, e le tue regole di autorizzazione si comportano normalmente in altre modalità di autorizzazione.

<Note>
  `autoMode.classifyAllShell` richiede Claude Code v2.1.193 o successivo. Le versioni precedenti ignorano la chiave e continuano a trasportare le regole di autorizzazione shell strette nella modalità auto.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Ispezionare i valori predefiniti e la tua configurazione effettiva
</h2>

I sottocomandi `claude auto-mode` ti aiutano a ispezionare, convalidare e ripristinare la tua configurazione.

Stampa le regole `environment`, `allow`, `soft_deny` e `hard_deny` incorporate come JSON:

```bash theme={null}
claude auto-mode defaults
```

Per leggere la formulazione completa di una regola senza eseguire il piping attraverso `jq`, passa `--label` con l'inizio dell'etichetta della regola, come `claude auto-mode defaults --label 'Git Destructive'`. La corrispondenza è un prefisso senza distinzione tra maiuscole e minuscole sull'etichetta di ogni regola e le sezioni senza corrispondenza vengono stampate come elenchi vuoti. Richiede Claude Code v2.1.208 o successivo.

Stampa ciò che il classificatore effettivamente utilizza come JSON, con le tue impostazioni applicate dove impostate e valori predefiniti altrimenti:

```bash theme={null}
claude auto-mode config
```

Sia `defaults` che `config` stampano i quattro elenchi di regole come un singolo oggetto JSON, con ogni regola come una stringa in prosa. Questo è un esempio troncato:

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, or hardcoding test cases. Placeholder means authored as a placeholder — a file or value copied from a real secret or sensitive path is never a test artifact (see Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), deleting remote branches, tags, or releases, or rewriting remote history. Also `git commit --amend` when the commit being rewritten is not the agent's own unpushed work: either no prior `git commit` is visible (HEAD pre-dates the session), or a `git push` of the current branch is visible after the most recent commit (it has been pushed). Clears when the user asked to amend/reword/fixup, or when it is a message-only reword (`--amend -m …`, nothing newly staged) of a commit the agent visibly created this session.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: The git repository the agent started in (its working directory) and its configured remote(s). When the repo's public/private visibility is given — by the Repository visibility entry or the user's own message — use it to scope what is OK to commit or push there: confidential material is fine in a private repo; in a public one, only that repo's own work is — and content ported, repointed, or first read from outside this session's repo is not its own work, whoever directed the port. Visibility scopes confidential material only: secrets and sensitive data (personal & entrusted) are never cleared into any repo by its visibility (see Definitions).",
    ...
  ]
}
```

Ottieni feedback AI sulle tue regole `allow`, `soft_deny` e `hard_deny` personalizzate:

```bash theme={null}
claude auto-mode critique
```

Esegui `claude auto-mode config` dopo aver salvato le tue impostazioni per confermare che le regole effettive sono quelle che ti aspetti, con `"$defaults"` espanso al suo posto. Se hai scritto regole personalizzate, `claude auto-mode critique` le esamina e contrassegna le voci che sono ambigue, ridondanti o probabilmente causeranno falsi positivi.

Per scartare le tue personalizzazioni e tornare ai valori predefiniti incorporati, esegui il sottocomando reset. Richiede Claude Code v2.1.212 o successivo e rimuove la sezione `autoMode` dal tuo file di impostazioni utente:

```bash theme={null}
claude auto-mode reset
```

Il comando riassume ciò che rimuoverà e chiede `Reset auto mode configuration to defaults?` prima di scrivere; passa `--yes` per saltare la conferma. Reset modifica solo `~/.claude/settings.json`: le regole `autoMode` dalle [impostazioni gestite](/docs/it/server-managed-settings) o dal flag `--settings` si applicano comunque.

<h2 id="review-denials">
  Esaminare i rifiuti
</h2>

Per esaminare e riprovare le azioni che il classificatore della modalità auto ha negato, apri `/permissions` e seleziona la scheda **Recently denied**, dove Claude Code registra ogni rifiuto. Premi `r` su un'azione negata per contrassegnarla per il retry: quando esci dalla finestra di dialogo, Claude Code invia un messaggio dicendo al modello che può riprovare quella chiamata di strumento e riprende la conversazione.

Quando il classificatore produce [nessun verdetto sull'azione](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action), perché un controllo di sicurezza separato dalla modalità auto ha rifiutato la richiesta del classificatore stesso o la sua risposta non è stata analizzata, Claude Code nega l'azione senza registrarla sotto **Recently denied**. La voce di errore collegata copre ciò che Claude viene detto e come eseguire l'azione se ne hai bisogno.

<h3 id="fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry">
  Correggere un rifiuto con una regola di autorizzazione, una voce di ambiente o un retry
</h3>

Per vedere cosa ha bloccato il classificatore, trova la chiamata di strumento nella conversazione. Se la chiamata appare accorciata o ripiegata in una riga di riepilogo come `Ran 3 shell commands`, premi `Ctrl+O` per aprire il [transcript viewer](/docs/it/interactive-mode#transcript-viewer), che l'espande.

Due altri posti sullo schermo che segnalano i rifiuti omettono il comando o l'URL: l'avviso vicino alla casella di input, come `bash denied by auto mode · [Data Exfiltration] · /permissions`, fornisce lo strumento e il motivo, e la scheda **Recently denied** elenca un comando shell dalla descrizione che Claude ha scritto per esso. Per acquisire l'input esatto di questi rifiuti a livello di programmazione, aggiungi un [hook `PermissionDenied`](/docs/it/hooks#permissiondenied), che lo riceve come `tool_input`.

Il testo sotto la chiamata ti dice se c'è qualcosa da correggere. Il testo che segnala un problema con il classificatore stesso, come un modello che `is temporarily unavailable` o un errore del classificatore, significa che Claude Code ha bloccato la chiamata senza un verdetto finale dal classificatore; vedi [Auto mode cannot determine the safety of an action](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action) per cosa fare. Altrimenti, una riga che legge `Denied by auto mode classifier` con un motivo come `[Production Deploy]` o `Blocked by classifier` significa che il classificatore ha giudicato la chiamata non sicura, quindi scegli la correzione da ciò che la chiamata stava cercando di raggiungere o fare:

* Una destinazione che Claude ha bisogno durante l'intero compito, come un registro di pacchetti, un dominio interno o un host di repository: aggiungilo a `autoMode.environment`.
* Un comando che desideri eseguire senza revisione da ora in poi: aggiungi una regola `allow`.
* Un'azione una tantum che intendevi: dichiara quell'intento nel tuo prossimo messaggio e lascia che Claude riprovi.

Puoi aggiungere la voce di ambiente o la regola `allow` dalla scheda [**Auto mode**](#edit-rules-from-permissions) della finestra di dialogo `/permissions`.

Nella maggior parte delle sessioni il nome del motivo nomina la regola che il classificatore ha abbinato, tra parentesi quadre, come `[Data Exfiltration]` o `[Production Deploy]`, e alcune sessioni eseguono un modello di classificatore che aggiunge una breve spiegazione. Claude Code seleziona il modello di classificatore, quindi quale forma vedi non è qualcosa che configuri.

<h3 id="fix-repeated-denials">
  Correggere i rifiuti ripetuti
</h3>

I rifiuti ripetuti per la stessa destinazione di solito significano che il classificatore manca di contesto. Aggiungi quella destinazione a `autoMode.environment`, o [esegui `/auto-mode-setup`](#generate-environment-entries) per fare in modo che Claude Code rediga le voci, quindi esegui `claude auto-mode config` per confermare che il cambiamento ha avuto effetto.

Per reagire ai rifiuti a livello di programmazione, utilizza l'[hook `PermissionDenied`](/docs/it/hooks#permissiondenied).

<h2 id="see-also">
  Vedi anche
</h2>

* [Permission modes](/docs/it/permission-modes#eliminate-prompts-with-auto-mode): cos'è la modalità auto, cosa blocca per impostazione predefinita e quali sessioni iniziano in essa
* [Managed settings](/docs/it/server-managed-settings): distribuisci la configurazione `autoMode` in tutta la tua organizzazione
* [Permissions](/docs/it/permissions): regole di autorizzazione, richiesta e negazione che si applicano prima dell'esecuzione del classificatore
* [All settings](/docs/it/settings-reference#automode): ogni chiave di impostazioni, inclusa `autoMode`
