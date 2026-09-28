> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Aspetti legali e conformità

> Accordi legali, certificazioni di conformità e informazioni sulla sicurezza per Claude Code.

<h2 id="legal-agreements">
  Accordi legali
</h2>

<h3 id="license">
  Licenza
</h3>

L'utilizzo di Claude Code è soggetto a:

* [Termini commerciali](https://www.anthropic.com/legal/commercial-terms) - per gli utenti di Team, Enterprise e Claude API
* [Termini di servizio per i consumatori](https://www.anthropic.com/legal/consumer-terms) - per gli utenti di Free, Pro e Max

<h3 id="commercial-agreements">
  Accordi commerciali
</h3>

Che stiate utilizzando l'API Claude direttamente (1P) o accedendovi tramite Amazon Bedrock o Google Cloud's Agent Platform (3P), il vostro accordo commerciale esistente si applicherà all'utilizzo di Claude Code, a meno che non abbiate concordato diversamente.

<h3 id="can-customers-offer-claude-code-in-their-products">
  I clienti possono offrire Claude Code nei loro prodotti?
</h3>

A meno che non abbiate concordato diversamente, la preinstallazione o l'esecuzione di Claude Code nei vostri prodotti o servizi (ad esempio in sandbox ospitati o altre infrastrutture di agenti) richiede l'accettazione dei nostri [Termini commerciali](https://www.anthropic.com/legal/commercial-terms) e il rispetto delle condizioni seguenti:

* **Il binario di Claude Code non deve essere modificato.** Claude Code deve essere installato ed eseguito come pubblicato da Anthropic, e i clienti non possono rimuovere, disabilitare o limitare alcun metodo di autenticazione integrato (inclusi i metodi che consentono l'accesso con un account Claude o la chiave API dell'utente).
* **I clienti non possono pagare, rivendere o intermediare l'utilizzo di Claude per conto dei loro utenti finali.** Ogni utente finale deve autenticarsi con la propria chiave API Anthropic, le credenziali del piano di abbonamento Claude o le credenziali del provider di inferenza di terze parti (Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry). Tale utilizzo viene fatturato direttamente all'utente finale secondo il suo accordo con Anthropic o, per i provider di inferenza di terze parti, con il provider applicabile.

**Utilizzo del nome e del logo di Claude Code.** Potete affermare accuratamente, in testo semplice, che il vostro prodotto ha Claude Code preinstallato o che esegue Claude Code. Ma non potete utilizzare i nomi o i logo di Claude Code o Anthropic come parte del vostro nome di prodotto, funzionalità o azienda, nel vostro logo, o in un modo che suggerisca che Anthropic ha costruito, approva o è in partnership con il vostro prodotto. Qualsiasi altro utilizzo dei nomi o dei logo di Anthropic è disciplinato dalle nostre [Linee guida sui marchi](https://www.anthropic.com/legal/trademark-guidelines) e richiede la nostra autorizzazione scritta.

Claude Code rimane disciplinato dai termini standard di Anthropic (vedere le sezioni Licenza e Accordi commerciali sopra) indipendentemente dalla piattaforma attraverso la quale vi si accede.

<h2 id="compliance">
  Conformità
</h2>

<h3 id="healthcare-compliance-baa">
  Conformità sanitaria (BAA)
</h3>

Se un cliente ha stipulato un Business Associate Agreement (BAA) con Anthropic e ha [Zero Data Retention (ZDR)](/docs/it/zero-data-retention) abilitato per l'organizzazione pertinente, tale BAA si estende al traffico API del cliente attraverso Claude Code.

<h2 id="usage-policy">
  Politica di utilizzo
</h2>

<h3 id="acceptable-use">
  Utilizzo accettabile
</h3>

L'utilizzo di Claude Code è soggetto alla [Politica di utilizzo di Anthropic](https://www.anthropic.com/legal/aup). I limiti di utilizzo pubblicizzati per i piani Pro e Max presuppongono un utilizzo ordinario e individuale di Claude Code e dell'Agent SDK.

<h3 id="authentication-and-credential-use">
  Autenticazione e utilizzo delle credenziali
</h3>

Claude Code si autentica con i server di Anthropic utilizzando token OAuth o chiavi API. Questi metodi di autenticazione servono a scopi diversi:

* **L'autenticazione OAuth** è destinata esclusivamente agli acquirenti dei piani di abbonamento Claude Free, Pro, Max, Team ed Enterprise ed è progettata per supportare l'utilizzo ordinario di Claude Code e di altre applicazioni native di Anthropic. Per i passaggi di accesso, consultare [Accesso al vostro account Claude](https://support.claude.com/en/articles/13189465-logging-in-to-your-claude-account); per informazioni su come Claude Code esegue l'autenticazione OAuth, consultare [Autenticazione](/docs/it/authentication).
* **Gli sviluppatori** che creano prodotti o servizi che interagiscono con le capacità di Claude, inclusi quelli che utilizzano l'[Agent SDK](/docs/it/agent-sdk/overview), devono utilizzare l'autenticazione tramite chiave API tramite [Claude Console](https://platform.claude.com/) o un provider cloud supportato. Anthropic non consente ai sviluppatori di terze parti di offrire l'accesso a Claude.ai nelle loro applicazioni, o di instradare le richieste tramite credenziali dei piani Free, Pro o Max per conto dei loro utenti. Inoltre, gli sviluppatori non possono raccogliere, archiviare o intermediare credenziali di Claude.ai o token di sessione — l'accesso a un account Claude deve completarsi attraverso il flusso di Anthropic.

Questo non limita il modo in cui i clienti forniscono e gestiscono le proprie chiavi API o credenziali di provider di inferenza di terze parti — ad esempio, configurando una chiave API in un ambiente di sviluppo, un gestore di segreti, o un'immagine di macchina per l'uso da parte degli utenti autorizzati del cliente — a condizione che l'utilizzo risultante sia fatturato al proprietario della chiave secondo il suo accordo con Anthropic (o il provider applicabile) e non sia rivenduto o intermediato come descritto sopra. Né impedisce a un utente finale di accedere al binario Claude Code non modificato con il proprio abbonamento Claude, incluso il caso in cui una piattaforma ospiti Claude Code come descritto sotto *I clienti possono offrire Claude Code nei loro prodotti?* sopra.

Anthropic si riserva il diritto di adottare misure per far rispettare queste restrizioni e può farlo senza preavviso.

Per domande sui metodi di autenticazione consentiti per il vostro caso d'uso, vi preghiamo di [contattare il team di vendita](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=legal_compliance_contact_sales).

<h2 id="security-and-trust">
  Sicurezza e fiducia
</h2>

<h3 id="trust-and-safety">
  Fiducia e sicurezza
</h3>

Potete trovare ulteriori informazioni nel [Centro fiducia di Anthropic](https://trust.anthropic.com) e nell'[Hub di trasparenza](https://www.anthropic.com/transparency).

<h3 id="security-vulnerability-reporting">
  Segnalazione di vulnerabilità di sicurezza
</h3>

Anthropic gestisce il nostro programma di sicurezza tramite HackerOne. [Utilizzate questo modulo per segnalare vulnerabilità](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new).

***

© Anthropic PBC. Tutti i diritti riservati. L'utilizzo è soggetto ai Termini di servizio di Anthropic applicabili.
