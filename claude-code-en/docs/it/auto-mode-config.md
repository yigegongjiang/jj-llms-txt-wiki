> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurare la modalità auto

> Comunica al classificatore della modalità auto quali repository, bucket e domini la tua organizzazione ritiene affidabili. Imposta il contesto dell'ambiente, sostituisci le regole di blocco e autorizzazione predefinite e ispeziona la tua configurazione effettiva con i sottocomandi CLI della modalità auto.

[Auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) consente a Claude Code di funzionare senza prompt di autorizzazione instradando ogni chiamata di strumento attraverso un classificatore che blocca qualsiasi cosa irreversibile, distruttiva o rivolta al di fuori del tuo ambiente. Le regole di negazione e richiesta esplicita vengono valutate prima del classificatore e continuano comunque a bloccare o richiedere. Utilizza il blocco di impostazioni `autoMode` per comunicare al classificatore quali repository, bucket e domini la tua organizzazione ritiene affidabili, in modo che smetta di bloccare le operazioni interne di routine.

<Note>
  Auto mode è disponibile a tutti gli utenti dell'API Anthropic. Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e sessioni [gateway di app Claude](/docs/it/claude-apps-gateway) con accesso, devi prima [impostare `CLAUDE_CODE_ENABLE_AUTO_MODE`](/docs/it/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry). Se Claude Code segnala che la modalità auto non è disponibile per il tuo account, controlla i [requisiti completi](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), che coprono anche i modelli supportati e l'abilitazione del proprietario sui piani Team ed Enterprise.
</Note>

Per impostazione predefinita, il classificatore si fida solo della directory di lavoro e dei remote configurati del repository corrente. Azioni come il push verso l'organizzazione di controllo del codice sorgente della tua azienda o la scrittura in un bucket cloud del team vengono bloccate finché non le aggiungi a `autoMode.environment`.

Per informazioni su come abilitare la modalità auto e cosa blocca per impostazione predefinita, consulta [Permission modes](/docs/it/permission-modes#eliminate-prompts-with-auto-mode). Questa pagina è il riferimento di configurazione.

Questa pagina spiega come:

* [Scegliere dove impostare le regole](#where-the-classifier-reads-configuration) in CLAUDE.md, impostazioni utente e impostazioni gestite
* [Definire l'infrastruttura affidabile](#define-trusted-infrastructure) con `autoMode.environment`
* [Sostituire le regole di blocco e autorizzazione](#override-the-block-and-allow-rules) quando i valori predefiniti non si adattano alla tua pipeline
* [Instradare tutti i comandi shell attraverso il classificatore](#route-all-shell-commands-through-the-classifier) con `autoMode.classifyAllShell`
* [Ispezionare la tua configurazione effettiva](#inspect-the-defaults-and-your-effective-config) con i sottocomandi `claude auto-mode`
* [Esaminare i rifiuti](#review-denials) in modo da sapere cosa aggiungere successivamente

<h2 id="where-the-classifier-reads-configuration">
  Dove il classificatore legge la configurazione
</h2>

Il classificatore legge lo stesso contenuto [CLAUDE.md](/docs/it/memory) che Claude stesso carica, quindi un'istruzione come "non forzare mai il push" nel CLAUDE.md del tuo progetto guida sia Claude che il classificatore contemporaneamente. Inizia da lì per le convenzioni del progetto e le regole comportamentali.

Per le regole che si applicano tra i progetti, come l'infrastruttura affidabile o le regole di negazione a livello di organizzazione, utilizza il blocco di impostazioni `autoMode`. Il classificatore legge `autoMode` dai seguenti ambiti:

| Ambito                        | File                                            | Utilizzare per                                                 |
| :---------------------------- | :---------------------------------------------- | :------------------------------------------------------------- |
| Un sviluppatore               | `~/.claude/settings.json`                       | Infrastruttura affidabile personale                            |
| Un progetto, uno sviluppatore | `.claude/settings.local.json`                   | Bucket o servizi affidabili per progetto                       |
| A livello di organizzazione   | [Managed settings](/docs/it/server-managed-settings) | Infrastruttura affidabile distribuita a tutti gli sviluppatori |
| Flag `--settings` o Agent SDK | JSON inline                                     | Override per invocazione per l'automazione                     |

Il classificatore non legge `autoMode` dalle impostazioni di progetto condivise in `.claude/settings.json`, quindi un repository archiviato non può iniettare le proprie regole di autorizzazione.

Le voci di ogni ambito vengono combinate. Uno sviluppatore può estendere `environment`, `allow`, `soft_deny` e `hard_deny` con voci personali ma non può rimuovere le voci fornite dalle impostazioni gestite. Poiché le regole di autorizzazione agiscono come eccezioni alle regole di blocco morbido all'interno del classificatore, una voce `allow` aggiunta da uno sviluppatore può sostituire una voce `soft_deny` dell'organizzazione: la combinazione è additiva, non un confine di politica rigida.

<Note>
  Il classificatore è una seconda porta che si esegue dopo il [sistema di autorizzazioni](/docs/it/permissions). Per le azioni che non devono mai essere eseguite indipendentemente dall'intento dell'utente o dalla configurazione del classificatore, utilizza `permissions.deny` nelle impostazioni gestite, che blocca l'azione prima che il classificatore venga consultato e non può essere ignorato.
</Note>

<h2 id="define-trusted-infrastructure">
  Definire l'infrastruttura affidabile
</h2>

Per la maggior parte delle organizzazioni, `autoMode.environment` è l'unico campo che devi impostare. Comunica al classificatore quali repository, bucket e domini sono affidabili: il classificatore lo utilizza per decidere cosa significa "esterno", quindi qualsiasi destinazione non elencata è un potenziale bersaglio di esfiltrazione.

A partire da Claude Code v2.1.198, `claude auto-mode defaults` stampa tre tipi di voce di ambiente. Le versioni precedenti a v2.1.195 stampano solo i primi cinque slot di fiducia.

* **Context slots**: descrivono la tua organizzazione, stack e postura di sicurezza in modo che il classificatore legga le altre regole nel tuo contesto. A differenza degli altri due tipi, gli slot di contesto non hanno regole proprie che li prendono di mira. Ognuno predefinito è `None configured` o all'assunzione conservativa denominata accanto ad esso:
  * **Organization**
  * **Primary use of Claude Code**: predefinito per lo sviluppo software
  * **Cloud provider(s)**
  * **Repository visibility**: un repository è assunto privato a meno che il suo host remoto e il nome non indichino diversamente, o un controllo di visibilità precedente nella conversazione che il classificatore legge mostri che è pubblico. Il classificatore legge i tuoi messaggi e i comandi che Claude esegue, non il loro output, quindi l'evidenza deve essere qualcosa che può leggere, come il tuo stesso messaggio che nomina il repository come pubblico; l'output di un `gh repo view` da solo non lo raggiunge. Il controllo delle prove della trascrizione richiede Claude Code v2.1.200 o successivo
  * **Internal sharing / snippet hosting**: i servizi di paste e gist pubblici sono trattati come esterni al confine di fiducia finché non ne nomini uno
  * **Org-specific CLIs**
  * **Secrets management**
  * **Default / protected branches**: `main` e `master` sono trattati come protetti finché non nomini altri
  * **CI/CD deploy targets**
  * **Network posture**
  * **Protected deployment namespaces / environments**: ricade all'euristica Sensitive remote targets finché non li nomini
  * **Data retention / declassification**
* **Trust slots**: denominano ciò che il classificatore tratta come interno al tuo confine. Gli slot sono Trusted repo, Source control, Trusted internal domains, Trusted cloud buckets, Key internal services e Internal package registry. Le voci di repository e controllo del codice sorgente predefinite sono il repository di lavoro e i suoi remote configurati. Ogni altro slot di fiducia predefinito è `None configured`, quindi nient'altro è affidabile finché non lo aggiungi. La visibilità di un repository limita solo il materiale confidenziale: un repository privato è una destinazione accettabile per il materiale confidenziale, ma rendere un repository privato non cancella mai i segreti o i dati personali o affidati in esso, e il classificatore tratta il contenuto trasportato, reindirizzato o letto per la prima volta dall'esterno del repository di lavoro come non il lavoro proprio di quel repository. Questo ambito richiede Claude Code v2.1.203 o successivo.
* **Sensitivity slots**: denominano ciò che le regole protettive trattano come ad alto rischio. Gli slot sono Sensitive data locations & audiences, Sensitive remote targets e Protected IaC scopes. Ognuno predefinito è un'euristica ampia, come trattare qualsiasi host o namespace il cui nome contiene `prod` o `production` come bersaglio remoto sensibile, quindi le regole protettive sono attive prima di configurare qualsiasi cosa. Denominare bersagli concreti in uno slot di sensibilità fa sì che quelle regole si applichino ai bersagli denominati invece dell'euristica.

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

Le voci sono prosa, non regex o pattern di strumenti. Il classificatore le legge come regole in linguaggio naturale. Scrivile come descriveresti la tua infrastruttura a un nuovo ingegnere. Una sezione di ambiente completa copre:

* **Organizzazione**: il nome della tua azienda e per cosa Claude Code viene utilizzato principalmente, come sviluppo software, automazione dell'infrastruttura o ingegneria dei dati
* **Controllo del codice sorgente**: ogni organizzazione GitHub, GitLab o Bitbucket verso cui i tuoi sviluppatori eseguono il push
* **Provider cloud e bucket affidabili**: nomi di bucket o prefissi che Claude dovrebbe essere in grado di leggere e scrivere
* **Domini interni affidabili**: nomi host per API, dashboard e servizi all'interno della tua rete, come `*.internal.example.com`
* **Servizi interni chiave**: CI, registri di artefatti, indici di pacchetti interni, strumenti di gestione degli incidenti
* **Registro di pacchetti interno**: il registro npm, PyPI o altro privato attraverso il quale gli install dovrebbero instradare, in modo che gli install che lo bypassano per un registro pubblico vengano bloccati
* **Sensitive data locations & audiences**: i bucket, i database o i percorsi che contengono dati personali, dati aziendali confidenziali, credenziali, dati regolamentati o materiale simile sensibile, e i destinatari con cui i dati in ogni posizione possono essere condivisi, in modo che il classificatore protegga quelle posizioni invece di indovinare dal contenuto. Claude Code v2.1.195 attraverso v2.1.197 denominano questa voce PII / regulated-data locations e coprono solo le posizioni che contengono dati personali o regolamentati, senza la dimensione del destinatario
* **Bersagli remoti sensibili**: gli spazi dei nomi, gli host o i container che contano come produzione, in modo che i shell remoti e i port-forward in essi richiedano la tua approvazione esplicita
* **Ambiti IaC protetti**: le risorse di infrastruttura il cui apply o destroy dovrebbe sempre richiedere di denominare il cambiamento
* **Contesto aggiuntivo**: vincoli del settore regolamentato, infrastruttura multi-tenant o requisiti di conformità che influiscono su ciò che il classificatore dovrebbe trattare come rischioso

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
  L'impostazione di uno qualsiasi di `environment`, `allow`, `soft_deny` o `hard_deny` senza `"$defaults"` sostituisce l'intero elenco predefinito per quella sezione. Un array `soft_deny` senza `"$defaults"` scarta ogni regola di blocco soft incorporata, inclusi force push, `curl | bash` e distribuzioni di produzione. Un array `hard_deny` senza `"$defaults"` scarta le regole incorporate di esfiltrazione dei dati e di bypass della modalità auto.
</Danger>

Ogni sezione viene valutata indipendentemente, quindi l'impostazione di `environment` da sola lascia intatti gli elenchi predefiniti `allow`, `soft_deny` e `hard_deny`. Ometti `"$defaults"` solo quando intendi assumere la piena proprietà dell'elenco. Per farlo in modo sicuro, esegui `claude auto-mode defaults` per stampare le regole incorporate, copiale nel tuo file di impostazioni, quindi esamina ogni regola rispetto alla tua pipeline e tolleranza al rischio.

<h2 id="route-all-shell-commands-through-the-classifier">
  Instradare tutti i comandi shell attraverso il classificatore
</h2>

Per impostazione predefinita, le regole di autorizzazione Bash e PowerShell strette come `Bash(npm test)` si trasportano nella modalità auto e si risolvono prima dell'esecuzione del classificatore. La modalità auto sospende solo le regole ampie che concedono l'esecuzione di codice arbitrario, come `Bash(*)` o interpreti con caratteri jolly. Ciò significa che una regola stretta può comunque far passare un argomento distruttivo senza che il classificatore lo veda, ad esempio un percorso di script o un flag che il prefisso della regola non ha anticipato.

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

Tre sottocomandi CLI ti aiutano a ispezionare e convalidare la tua configurazione.

Stampa le regole `environment`, `allow`, `soft_deny` e `hard_deny` incorporate come JSON:

```bash theme={null}
claude auto-mode defaults
```

Stampa ciò che il classificatore effettivamente utilizza come JSON, con le tue impostazioni applicate dove impostate e valori predefiniti altrimenti:

```bash theme={null}
claude auto-mode config
```

Ottieni feedback AI sulle tue regole `allow`, `soft_deny` e `hard_deny` personalizzate:

```bash theme={null}
claude auto-mode critique
```

Esegui `claude auto-mode config` dopo aver salvato le tue impostazioni per confermare che le regole effettive sono quelle che ti aspetti, con `"$defaults"` espanso al suo posto. Se hai scritto regole personalizzate, `claude auto-mode critique` le esamina e contrassegna le voci che sono ambigue, ridondanti o probabilmente causeranno falsi positivi.

Se hai bisogno di rimuovere o riscrivere una regola incorporata piuttosto che aggiungerne una accanto ad essa, salva l'output di `claude auto-mode defaults` in un file, modifica gli elenchi e incolla il risultato nel tuo file di impostazioni al posto di `"$defaults"`.

<h2 id="review-denials">
  Esaminare i rifiuti
</h2>

Quando la modalità auto nega una chiamata di strumento, il rifiuto viene registrato in `/permissions` nella scheda Recently denied. Premi `r` su un'azione negata per contrassegnarla per il retry: quando esci dalla finestra di dialogo, Claude Code invia un messaggio al modello dicendogli che può riprovare quella chiamata di strumento e riprende la conversazione.

In Claude Code v2.1.193 e successivo, il motivo del classificatore per ogni rifiuto appare accanto alla chiamata di strumento bloccata nella trascrizione, nella notifica di rifiuto e sotto ogni voce nella scheda Recently denied. Utilizza il motivo per decidere se la correzione è una voce `environment`, un'eccezione `allow` o un retry con intento esplicito nel tuo prossimo messaggio.

I rifiuti ripetuti per la stessa destinazione di solito significano che il classificatore manca di contesto. Aggiungi quella destinazione a `autoMode.environment`, quindi esegui `claude auto-mode config` per confermare che ha avuto effetto.

Per reagire ai rifiuti a livello di programmazione, utilizza l'[hook `PermissionDenied`](/docs/it/hooks#permissiondenied).

<h2 id="see-also">
  Vedi anche
</h2>

* [Permission modes](/docs/it/permission-modes#eliminate-prompts-with-auto-mode): cos'è la modalità auto, cosa blocca per impostazione predefinita e come abilitarla
* [Managed settings](/docs/it/server-managed-settings): distribuisci la configurazione `autoMode` in tutta la tua organizzazione
* [Permissions](/docs/it/permissions): regole di autorizzazione, richiesta e negazione che si applicano prima dell'esecuzione del classificatore
* [Settings](/docs/it/settings): il riferimento completo delle impostazioni, inclusa la chiave `autoMode`
