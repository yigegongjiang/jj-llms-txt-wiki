> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code con GitHub Enterprise Server

> Connetti Claude Code alla tua istanza GitHub Enterprise Server auto-ospitata per sessioni cloud, revisione del codice e marketplace di plugin.

<Note>
  Il supporto per GitHub Enterprise Server è disponibile per i piani Team ed Enterprise.
</Note>

Il supporto per GitHub Enterprise Server (GHES) consente alla tua organizzazione di utilizzare Claude Code con repository ospitati sulla tua istanza GitHub auto-gestita invece di github.com. Una volta che un Owner connette la tua istanza GHES, gli sviluppatori possono eseguire sessioni cloud e ottenere revisioni automatiche del codice senza alcuna configurazione per repository. I marketplace di plugin ospitati sulla tua istanza sono supportati; i requisiti delle credenziali variano in base alla superficie, come descritto in [Plugin marketplaces on GHES](#plugin-marketplaces-on-ghes).

Per i repository su github.com, vedi [Use Claude Code in the cloud](/docs/it/claude-code-on-the-web) e [Code Review](/docs/it/code-review). Per eseguire Claude nella tua infrastruttura CI, vedi [GitHub Actions](/docs/it/github-actions).

<h2 id="what-works-with-github-enterprise-server">
  Cosa funziona con GitHub Enterprise Server
</h2>

La tabella seguente mostra quali funzionalità di Claude Code supportano GHES e eventuali differenze dal comportamento di github.com.

| Funzionalità              | Supporto GHES    | Note                                                                                                                                                      |
| :------------------------ | :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sessioni cloud            | ✅ Supportato     | Un proprietario connette l'istanza GHES una volta; gli sviluppatori utilizzano `claude --cloud` o [claude.ai/code](https://claude.ai/code) come al solito |
| Code Review               | ✅ Supportato     | Stesse revisioni automatiche dei PR di github.com                                                                                                         |
| Claude Security           | ✅ Supportato     | Disponibile in beta pubblica per i piani Enterprise su [claude.ai/security](https://claude.ai/security)                                                   |
| Sessioni Teleport         | ✅ Supportato     | Sposta le sessioni tra cloud e terminale con `--teleport`                                                                                                 |
| Marketplace di plugin     | ✅ Supportato     | I requisiti delle credenziali differiscono per superficie. Vedere [Plugin marketplaces on GHES](#plugin-marketplaces-on-ghes)                             |
| Metriche di contribuzione | ✅ Supportato     | Consegnate tramite webhook al [dashboard di analisi](/docs/it/analytics)                                                                                       |
| GitHub Actions            | ✅ Supportato     | Richiede configurazione manuale del workflow; `/install-github-app` è solo per github.com                                                                 |
| Server GitHub MCP         | ❌ Non supportato | Il server GitHub MCP non funziona con istanze GHES                                                                                                        |

<h2 id="admin-setup">
  Configurazione amministratore
</h2>

Un proprietario connette la tua istanza GHES a Claude Code una volta. Dopo di che, gli sviluppatori della tua organizzazione possono utilizzare i repository GHES senza alcuna configurazione aggiuntiva. Hai bisogno del ruolo Proprietario o Proprietario principale nella tua organizzazione Claude e del permesso di creare GitHub App sulla tua istanza GHES.

La configurazione guidata genera un manifesto GitHub App e ti reindirizza alla tua istanza GHES per creare l'app in un clic. Se il tuo ambiente blocca il flusso di reindirizzamento, è disponibile una [configurazione manuale alternativa](#manual-setup).

<Steps>
  <Step title="Apri le impostazioni amministratore di Claude Code">
    Vai a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) e trova la sezione GitHub Enterprise Server.
  </Step>

  <Step title="Avvia la configurazione guidata">
    Fai clic su **Connetti**. Inserisci un nome visualizzato di massimo 20 caratteri per la connessione e il nome host GHES, ad esempio `github.example.com`. Se la tua istanza GHES utilizza un certificato autofirmato o un'autorità di certificazione privata, incolla il certificato CA nel campo facoltativo.
  </Step>

  <Step title="Crea l'app GitHub">
    Fai clic su **Continua verso GitHub Enterprise**. Il tuo browser si reindirizza alla tua istanza GHES con un manifesto app pre-compilato. Rivedi la configurazione e fai clic su **Crea GitHub App**. GHES ti reindirizza di nuovo a Claude con le credenziali dell'app archiviate automaticamente.
  </Step>

  <Step title="Installa l'app sui tuoi repository">
    Dalla pagina GitHub App sulla tua istanza GHES, installa l'app sui repository o sulle organizzazioni a cui desideri che Claude acceda. Puoi iniziare con un sottoinsieme e aggiungerne altri in seguito.
  </Step>

  <Step title="Abilita le funzionalità">
    Torna a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) e abilita [Code Review](/docs/it/code-review#set-up-code-review), Claude Security e [metriche di contribuzione](/docs/it/analytics#enable-contribution-metrics) per i tuoi repository GHES utilizzando la stessa configurazione di github.com.
  </Step>
</Steps>

<h3 id="github-app-permissions">
  Permessi GitHub App
</h3>

Il manifesto configura l'app GitHub con i permessi e gli eventi webhook di seguito, che insieme coprono sessioni web, Code Review, Claude Security, marketplace di plugin e metriche di contribuzione:

| Permesso             | Accesso             | Utilizzato per                                                                                                                                                                                                                                       |
| :------------------- | :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Contents             | Lettura e scrittura | Clonazione di repository e push di branch                                                                                                                                                                                                            |
| Pull requests        | Lettura e scrittura | Creazione di PR e pubblicazione di commenti di revisione                                                                                                                                                                                             |
| Issues               | Lettura e scrittura | Risposta a menzioni di problemi                                                                                                                                                                                                                      |
| Checks               | Lettura e scrittura | Pubblicazione di esecuzioni di controllo Code Review                                                                                                                                                                                                 |
| Actions              | Lettura             | Lettura dello stato CI per auto-fix                                                                                                                                                                                                                  |
| Commit statuses      | Lettura             | Lettura dello stato CI da provider che segnalano stati di commit invece di esecuzioni di controllo                                                                                                                                                   |
| Repository hooks     | Lettura e scrittura | Creazione di un webhook su un repository marketplace di plugin quando **Sincronizza automaticamente** è attivato per un marketplace in [**Impostazioni organizzazione > Plugin e skills**](https://claude.ai/admin-settings/skills?tab=marketplaces) |
| Metadata             | Lettura             | Richiesto da GitHub per tutte le app                                                                                                                                                                                                                 |
| Organization members | Lettura             | Corrispondenza dell'app GitHub di Claude su github.com, che la utilizza per verificare il ruolo dell'organizzazione di un utente che si connette quando si collega un'installazione                                                                  |

L'app si iscrive agli eventi `pull_request`, `issue_comment`, `pull_request_review_comment`, `pull_request_review`, `check_run` e `status`.

GitHub applica un manifesto solo quando l'app viene creata, quindi un'app creata da una versione precedente del manifesto mantiene i permessi e gli eventi con cui è stata creata. Se la tua app non dispone di alcuno dei permessi o degli eventi di cui sopra, aggiungili nelle impostazioni dell'app sulla tua istanza GHES. GitHub chiede quindi a un proprietario di ogni installazione di approvare i nuovi permessi e l'installazione mantiene i suoi vecchi permessi fino a quando non lo fanno.

<h3 id="manual-setup">
  Configurazione manuale
</h3>

Se il flusso di reindirizzamento guidato è bloccato dalla configurazione di rete, fai clic su **Aggiungi manualmente** invece di Connetti. Crea un'app GitHub sulla tua istanza GHES con i [permessi e gli eventi di cui sopra](#github-app-permissions), quindi inserisci i dettagli della connessione nel modulo: un nome visualizzato, il nome host GHES e la porta facoltativa, e l'ID dell'app, l'ID client, il segreto client, il segreto webhook e la chiave privata. Il modulo accetta anche un certificato CA personalizzato facoltativo e nomi host di replica in lettura.

Claude genera l'URL webhook dell'app quando salvi la connessione. Dopo aver fatto clic su **Aggiungi configurazione**, apri il menu **Altre opzioni** della connessione, seleziona **Copia URL webhook** e incolla l'URL nelle impostazioni webhook dell'app sulla tua istanza GHES. Utilizza lo stesso segreto webhook che hai inserito nel modulo.

<h3 id="network-requirements">
  Requisiti di rete
</h3>

Per le sessioni ospitate da Anthropic, la tua istanza GHES deve essere raggiungibile dall'infrastruttura Anthropic in modo che Claude possa clonare repository e pubblicare commenti di revisione. Se la tua istanza GHES è dietro un firewall, inserisci nella whitelist gli [indirizzi IP in uscita](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) di Anthropic. Le sessioni in un [ambiente self-hosted](/docs/it/self-hosted-environments-deploy#configure-git) clonano dall'interno della tua rete, a meno che il runner non opti per il [proxy git Anthropic](/docs/it/self-hosted-environments-deploy#use-the-anthropic-git-proxy), che recupera dal lato di Anthropic e necessita della stessa raggiungibilità; il [connettore SCM](/docs/it/self-hosted-environments-reference#scm-connector-flags) copre i flussi pre-sessione ospitati, come il selettore di repository, per un host GHES che è instradabile solo internamente.

<h2 id="developer-workflow">
  Flusso di lavoro dello sviluppatore
</h2>

Una volta che il tuo amministratore ha connesso l'istanza GHES, non è necessaria alcuna configurazione lato sviluppatore. Claude Code rileva automaticamente il nome host GHES dal git remote nella tua directory di lavoro.

Clona un repository dalla tua istanza GHES come faresti normalmente, sostituendo `github.example.com` e il percorso del repository con il tuo nome host GHES e il repository:

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

Quindi avvia una sessione cloud. Claude rileva l'host GHES dal tuo git remote e instrada la sessione attraverso l'istanza configurata della tua organizzazione:

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

La sessione clona il tuo repository da GHES e spinge i cambiamenti di nuovo a un branch. Monitora l'avanzamento su [claude.ai/code](https://claude.ai/code). Vedi [Usa Claude Code nel cloud](/docs/it/claude-code-on-the-web) per il flusso di lavoro completo della sessione cloud inclusa la revisione diff, auto-fix e routine.

<h3 id="teleport-sessions-to-your-terminal">
  Teleport delle sessioni al tuo terminale
</h3>

Estrai una sessione cloud nel tuo terminale locale con `claude --teleport`. Teleport verifica che tu sia in un checkout dello stesso repository GHES prima di recuperare il branch e caricare la cronologia della sessione. Vedi [requisiti di teleport](/docs/it/claude-code-on-the-web#teleport-requirements) per i dettagli.

<h2 id="plugin-marketplaces-on-ghes">
  Marketplace di plugin su GHES
</h2>

Ospita marketplace di plugin sulla tua istanza GHES per distribuire strumenti interni in tutta la tua organizzazione. La struttura del marketplace è identica ai marketplace ospitati su github.com, ma l'installazione funziona diversamente a seconda di dove aggiungi il marketplace, e le credenziali differiscono tra le superfici:

| Superficie                                           | Come funziona l'installazione                                                                                                                                                                                                                                                    | Cosa serve a ogni utente                                                                                                                                                                                                                    |
| :--------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Claude Code CLI e desktop                            | Claude Code clona il repository del marketplace utilizzando le credenziali git esistenti della macchina                                                                                                                                                                          | Accesso Git al tuo host GHES dalla loro macchina                                                                                                                                                                                            |
| Impostazioni gestite (`extraKnownMarketplaces`)      | Claude Code registra la voce e clona il repository utilizzando le credenziali git esistenti della macchina                                                                                                                                                                       | Accesso Git al tuo host GHES dalla loro macchina                                                                                                                                                                                            |
| Impostazioni plugin dell'organizzazione su claude.ai | Un Owner seleziona l'istanza GHES come fonte; il backend di Anthropic recupera e sincronizza il repository utilizzando la GitHub App da [admin setup](#admin-setup)                                                                                                              | Nulla per utente una volta aggiunto. L'Owner che lo aggiunge ha bisogno del proprio account GitHub Enterprise connesso come controllo di accesso, e la GitHub App deve essere installata sul repository del marketplace                     |
| Impostazioni utente su claude.ai                     | Il backend di Anthropic recupera il repository utilizzando la connessione GitHub Enterprise dell'utente che lo invia                                                                                                                                                             | Il proprio account GitHub Enterprise connesso a Claude                                                                                                                                                                                      |
| Sessioni cloud                                       | Le sessioni cloud clonano i marketplace all'interno della sandbox della sessione. La sandbox può raggiungere la tua istanza GHES solo quando il repository della sessione si trova su quella stessa istanza, e le sue credenziali git sono limitate ai repository della sessione | Non affidabile per i marketplace ospitati su GHES: un host diverso dal repository della sessione non è raggiungibile, e anche gli install sulla stessa istanza possono fallire. Utilizza invece la CLI, le impostazioni gestite o claude.ai |

<Warning>
  Le connessioni GitHub Enterprise su claude.ai sono per utente quando un marketplace viene aggiunto dalle impostazioni utente. La [admin setup](#admin-setup) connette la tua istanza GHES alla tua organizzazione, ma non connette i singoli account utente: ogni utente che aggiunge un marketplace GHES dalle proprie impostazioni deve prima connettere il proprio account GitHub Enterprise, e la connessione di un utente, inclusa quella dell'Owner, non copre nessun altro. I marketplace aggiunti da un Owner nelle impostazioni plugin dell'organizzazione non pongono questo requisito agli utenti, perché i recuperi continui utilizzano la GitHub App dell'organizzazione. L'Owner che aggiunge il marketplace ha comunque bisogno del proprio account GitHub Enterprise connesso al momento dell'aggiunta.
</Warning>

<h3 id="add-a-ghes-marketplace">
  Aggiungi un marketplace GHES
</h3>

La scorciatoia `owner/repo` si risolve sempre in github.com. Per i marketplace ospitati su GHES, utilizza l'URL git completo, sostituendo `github.example.com` e il percorso del repository con i tuoi. Gli URL HTTPS sono consigliati:

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

Gli URL SSH funzionano se la macchina già si fida del tuo host GHES:

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code esegue git in modo non interattivo e rifiuta le connessioni SSH agli host che non si trovano nel file `known_hosts` della macchina. Un URL HTTPS con un git credential helper evita il requisito `known_hosts`.

Vedi [Crea e distribuisci un marketplace di plugin](/docs/it/plugins/create-marketplace) per la guida completa alla creazione di marketplace.

<h3 id="pre-register-ghes-marketplaces-with-managed-settings">
  Pre-registra i marketplace GHES con le impostazioni gestite
</h3>

L'impostazione `extraKnownMarketplaces` pre-registra un marketplace in modo che gli sviluppatori lo ottengano senza configurazione manuale. Funziona da [qualsiasi file di impostazioni](/docs/it/settings-reference#extraknownmarketplaces), incluso il `.claude/settings.json` di un repository; le impostazioni gestite lo distribuiscono a livello di organizzazione:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code installa questi marketplace localmente: registra ogni voce e clona il repository con le credenziali git esistenti della macchina. Questo percorso non passa attraverso claude.ai, quindi la connessione GitHub Enterprise per utente non è richiesta. Per un rollout di successo:

* **Utilizza un URL git completo.** La scorciatoia `owner/repo` si risolve sempre in github.com e non può fare riferimento a un host GHES.
* **Preferisci gli URL HTTPS.** I cloni SSH falliscono su macchine che non si fidano già della chiave host GHES. Un URL HTTPS con il git credential helper standard della tua organizzazione funziona su qualsiasi macchina con credenziali configurate.
* **Conferma che ogni macchina possa clonare dal tuo host GHES.** Se una macchina manca di credenziali, il marketplace viene registrato ma mai installato, e i suoi plugin vengono segnalati come non trovati invece di richiedere credenziali.
* **Conferma che l'impostazione raggiunga ogni macchina.** Un file di impostazioni gestite ha effetto solo sulle macchine su cui viene distribuito, ad esempio attraverso il tuo sistema di gestione dei dispositivi. Vedi [Distribuisci impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms) per i percorsi dei file.

<h3 id="allowlist-ghes-marketplaces-in-managed-settings">
  Inserisci nella whitelist i marketplace GHES nelle impostazioni gestite
</h3>

Se la tua organizzazione utilizza [impostazioni gestite](/docs/it/settings) per limitare quali marketplace gli sviluppatori possono aggiungere, utilizza il tipo di origine `hostPattern` per consentire tutti i marketplace dalla tua istanza GHES senza enumerare ogni repository. Vedi [Meccanismi di distribuzione](/docs/it/managed-settings#delivery-mechanisms) per i percorsi dei file su ogni piattaforma. Aggiungi il JSON al tuo file `managed-settings.json` o alla policy MDM equivalente:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Vedi il riferimento alle impostazioni [strictKnownMarketplaces](/docs/it/settings-reference#strictknownmarketplaces) e [extraKnownMarketplaces](/docs/it/settings-reference#extraknownmarketplaces) per lo schema completo.

<h2 id="limitations">
  Limitazioni
</h2>

Alcune funzionalità si comportano diversamente su GHES rispetto a github.com. La [tabella delle funzionalità](#what-works-with-github-enterprise-server) riassume il supporto; questa sezione copre le soluzioni alternative.

* **Comando `/install-github-app`**: segui il flusso di [configurazione amministratore](#admin-setup) su claude.ai. Se desideri anche workflow GitHub Actions su GHES, adatta manualmente il [workflow di esempio](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml).
* **Server GitHub MCP**: utilizza invece la CLI `gh` configurata per il tuo host GHES. Esegui `gh auth login --hostname github.example.com` per autenticarti, quindi Claude può utilizzare i comandi `gh` nelle sessioni.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="cloud-session-fails-to-clone-repository">
  La sessione cloud non riesce a clonare il repository
</h3>

Se `claude --cloud` fallisce con un errore di clone, verifica che un Owner abbia completato la configurazione per la tua istanza GHES e che l'app GitHub sia installata sul repository su cui stai lavorando. Chiedi all'Owner che ha connesso l'istanza di confermare che il nome host registrato nelle impostazioni di Claude corrisponda al nome host nel tuo git remote.

<h3 id="marketplace-add-fails-with-a-policy-error">
  L'aggiunta del marketplace fallisce con un errore di policy
</h3>

Se `/plugin marketplace add` è bloccato per il tuo URL GHES, la tua organizzazione ha limitato le origini del marketplace. Chiedi al tuo amministratore di aggiungere una voce `hostPattern` per il nome host GHES nelle [impostazioni gestite](#allowlist-ghes-marketplaces-in-managed-settings).

<h3 id="marketplace-add-on-claude-ai-fails-with-a-github-access-error">
  L'aggiunta del marketplace su claude.ai fallisce con un errore di accesso a GitHub
</h3>

Se l'aggiunta di un marketplace GHES dalle impostazioni utente fallisce con un errore generico come "Marketplace couldn't be added", controlla prima la tua connessione GitHub Enterprise. Questo è quello che appare quando il tuo account GitHub Enterprise non è connesso a Claude, anche se l'istanza GHES della tua organizzazione è configurata e altri utenti sono connessi. La finestra di dialogo non punta al flusso di connessione di GitHub Enterprise, e l'opzione "Connect to GitHub" nella scheda Browse accede a github.com, che non concede accesso ai repository GHES.

Per connettere il tuo account GitHub Enterprise: il selettore di repository su [claude.ai/code](https://claude.ai/code) offre un'opzione di connessione per ogni istanza GHES configurata, e gli Owner possono anche connettersi dalla sezione GitHub Enterprise delle [impostazioni amministratore di Claude Code](https://claude.ai/admin-settings/claude-code). Quindi aggiungi di nuovo il marketplace. In alternativa, chiedi a un Owner di aggiungere il marketplace nelle impostazioni del plugin dell'organizzazione, il che rimuove il requisito di connessione per utente.

Su altre superfici di claude.ai, un errore "Repository not found. If it's private, GitHub access is required" su un marketplace GHES di solito indica la stessa connessione mancante. Connetti il tuo account GitHub Enterprise attraverso uno dei percorsi sopra, quindi riprova.

<h3 id="ghes-instance-not-reachable">
  Istanza GHES non raggiungibile
</h3>

Se le revisioni o le sessioni cloud ospitate da Anthropic si esauriscono, la tua istanza GHES potrebbe non essere raggiungibile dall'infrastruttura Anthropic. Conferma che il tuo firewall consente connessioni in entrata dagli [indirizzi IP in uscita di Anthropic](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses). Le sessioni in un [ambiente self-hosted](/docs/it/self-hosted-environments) raggiungono GHES dall'interno della tua rete, quindi per loro controlla il percorso di rete del runner stesso e il [connettore SCM](/docs/it/self-hosted-environments-reference#scm-connector-flags) invece.

<h3 id="session-start-fails-with-unable-to-get-organization-uuid">
  L'avvio della sessione fallisce con `Unable to get organization UUID`
</h3>

Le sessioni cloud richiedono un'organizzazione Team o Enterprise. Accedi con `/login` utilizzando il tuo account dell'organizzazione. Se ti autentichi con una chiave API, le sessioni cloud falliscono prima con un messaggio che ti chiede di eseguire `/login`.

<h2 id="related-resources">
  Risorse correlate
</h2>

Queste pagine coprono le funzionalità referenziate in questa guida in maggior dettaglio:

* [Claude Code sul web](/docs/it/claude-code-on-the-web): esegui sessioni Claude Code sull'infrastruttura cloud
* [Code Review](/docs/it/code-review): revisioni automatiche dei PR
* [Marketplace di plugin](/docs/it/plugins/host-marketplace): crea e distribuisci cataloghi di plugin
* [Analytics](/docs/it/analytics): traccia l'utilizzo e le metriche di contribuzione
* [Impostazioni gestite](/docs/it/settings): configurazione della policy a livello di organizzazione
* [Configurazione di rete](/docs/it/network-config): requisiti di firewall e whitelist IP
