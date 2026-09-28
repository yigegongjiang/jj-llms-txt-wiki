> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Démarrage rapide des environnements auto-hébergés

> Configurez votre premier environnement auto-hébergé : installez Claude Code, créez l'environnement, démarrez un runner et routez une session vers celui-ci.

<Note>
  Les environnements auto-hébergés sont en bêta publique sur les plans Team et Enterprise ; [Disponibilité et limitations](/docs/fr/self-hosted-environments#availability-and-limitations) couvre le chemin d'activation. Cette page lance votre première session ; consultez [Environnements auto-hébergés](/docs/fr/self-hosted-environments) pour comprendre ce qu'ils sont et [Déployer en production](/docs/fr/self-hosted-environments-deploy) pour le durcissement et les recettes de flotte.
</Note>

Un [environnement auto-hébergé](/docs/fr/self-hosted-environments) exécute les [sessions cloud](/docs/fr/claude-code-on-the-web) de Claude Code sur l'infrastructure que votre organisation exploite, exécutées par des processus runner que vous déployez. Ce démarrage rapide en configure votre premier, le plus petit qui fonctionne : un runner sur un seul hôte, exécutant une session de test. Il y a deux étapes : [créer l'environnement, démarrer un runner et router une session vers celui-ci](#set-up-an-environment-and-runner), puis [envoyer un message à cette session depuis votre terminal](#send-a-follow-up-message-to-a-running-session). Vous vous déplacerez entre deux surfaces : claude.ai pour créer l'environnement, vérifier son statut et router une session, et un terminal sur l'hôte pour tout ce que le runner fait.

À la fin, vous aurez un environnement sur la [page d'administration **Environnements cloud**](https://claude.ai/admin-settings/cloud-environments), un runner interrogeant le travail, et une session s'exécutant sur votre hôte. Avant de connecter des référentiels réels ou des systèmes internes, travaillez sur [Déployer en production](/docs/fr/self-hosted-environments-deploy), qui couvre la posture de sécurité, le contrôle de sortie, les identifiants git et l'orchestration.

<h2 id="prerequisites">
  Prérequis
</h2>

<h3 id="organization-and-roles">
  Organisation et rôles
</h3>

Le côté claude.ai a besoin de :

* **Autoriser les environnements auto-hébergés** activé par un [Propriétaire](/docs/fr/cloud-environments#organization-shared-environments) sur la [page d'administration **Environnements cloud**](https://claude.ai/admin-settings/cloud-environments) ; le bouton **Nouveau** n'apparaît pas tant qu'il ne l'est pas. Si vous ne tenez pas le rôle, quelqu'un qui le tient peut créer l'environnement et vous remettre son secret ; les étapes du runner et du terminal sur cette page ne nécessitent aucun rôle claude.ai, et où une étape vérifie le statut dans l'interface d'administration, les propres lignes de journal du runner vous donnent le même signal.
* Une [connexion GitHub](/docs/fr/claude-code-on-the-web#github-authentication-options) pour votre organisation, afin que les développeurs puissent sélectionner des référentiels lorsqu'ils démarrent des sessions.

<h3 id="host-and-network">
  Hôte et réseau
</h3>

L'hôte du runner a besoin de :

* Un hôte ou conteneur Linux ou macOS avec HTTPS sortant vers `api.anthropic.com`, vers `claude.ai` et les hôtes de téléchargement vers lesquels il redirige pour l'étape d'installation ci-dessous, et vers votre hôte git pour le clone ; le [tableau des exigences réseau](/docs/fr/self-hosted-environments-deploy#network-requirements) a la liste complète. Windows n'est pas pris en charge en tant qu'hôte runner ; exécutez le runner dans un conteneur Linux à la place. Les postes de travail des développeurs ne sont pas affectés, car les sessions démarrent à partir de claude.ai dans un navigateur.
* Une horloge synchronisée à l'heure réelle, par exemple avec NTP. L'authentification échoue lorsque l'horloge est décalée de plus de cinq minutes ; consultez [Dépannage](/docs/fr/self-hosted-environments-deploy#troubleshooting).

<h3 id="software-on-the-runner-host">
  Logiciel sur l'hôte du runner
</h3>

Installez sur l'hôte avant de commencer :

* **Claude Code v2.1.224 ou ultérieur**, avec l'une des [méthodes d'installation standard](/docs/fr/setup). Le runner fait partie du binaire `claude` standard, et les versions antérieures ne reconnaissent pas la sous-commande `self-hosted-runner`. Le canal `latest` par défaut du programme d'installation natif porte chaque version dès sa publication ; le canal `stable`, le cask Homebrew `claude-code`, et les référentiels apt, dnf et apk stables traînent d'environ une semaine. Pour épingler la version exacte que votre flotte exécute, consultez [Installer une version spécifique](/docs/fr/setup#install-a-specific-version). Pour les images de conteneur, consultez le Dockerfile dans [Déployer en production](/docs/fr/self-hosted-environments-deploy#build-the-runner-image).
* **Git 2.24 ou plus récent**. Certaines options git sur la page de déploiement nécessitent des versions plus récentes ; [Configurer git](/docs/fr/self-hosted-environments-deploy#configure-git) indique chaque plancher.

Confirmez que l'hôte est prêt :

```bash theme={null}
claude self-hosted-runner --help
```

Un hôte prêt imprime le texte d'utilisation du runner, listant les drapeaux tels que `--environment-secret-file`. Sur les versions antérieures à 2.1.224, la commande imprime la sortie générale `claude --help` à la place ; mettez à jour avec `claude update` ou réinstallez à partir du canal `latest`.

<h2 id="set-up-an-environment-and-runner">
  Configurer un environnement et un runner
</h2>

Claude Code inclut une configuration guidée : une session Claude Code interactive qui vous guide à travers la création de l'environnement dans l'interface d'administration, démarre un runner local avec le fichier secret que vous enregistrez, confirme que le runner s'enregistre, et écrit une feuille de triche dans `./runner-setup/CHEAT-SHEET.md`. Exécutez-le sur une machine où vous vous êtes connecté avec `claude auth login` en utilisant un compte qui détient un rôle Propriétaire ; il n'est pas disponible avec les clés API ou les fournisseurs de modèles tiers. Sur les hôtes où une session interactive n'est pas possible, utilisez plutôt les étapes manuelles ci-dessous. Confirmez d'abord que la [vérification de version](#software-on-the-runner-host) a réussi : sur les versions antérieures à 2.1.224, cette commande démarre une session Claude ordinaire avec les mots comme invite au lieu de la configuration guidée. Pour démarrer la configuration guidée, exécutez la sous-commande setup et suivez les invites :

```bash theme={null}
claude self-hosted-runner setup
```

Pour configurer manuellement à la place :

<Steps>
  <Step title="Créer un environnement">
    Allez à la [page **Environnements cloud**](https://claude.ai/admin-settings/cloud-environments) dans les paramètres d'administration. Sous **Environnements auto-hébergés**, sélectionnez **Nouveau**, nommez l'environnement, et sélectionnez **Créer**. À la deuxième étape de l'assistant, sélectionnez **Copier la clé d'environnement** pour copier le secret d'environnement, que l'interface d'administration étiquette comme clé d'environnement. claude.ai affiche le secret une fois, et vous ne pouvez pas le récupérer plus tard ; il expire 365 jours après sa création. L'ID `ccpool_...` de l'environnement reste visible dans sa boîte de dialogue de détail ; vous en aurez besoin pour la vérification `aud` dans [vérification de token](/docs/fr/self-hosted-environments-identity) et pour dispatcher [les sessions de test à partir de CI](/docs/fr/self-hosted-environments-testing#run-the-test-loop).

    Si vous perdez le secret ou avez besoin de le faire tourner, créez un nouveau secret à partir de l'onglet **Configuration** de l'environnement, déployez le nouveau secret sur vos runners, puis révoquez l'ancien. Les runners détenant un secret révoqué échouent leur prochain sondage authentifié et se terminent, en enregistrant `poll auth failed`, et votre orchestrateur les redémarre avec le nouveau secret.
  </Step>

  <Step title="Démarrer un runner">
    Créez le répertoire secret. Cette étape et la suivante nécessitent root pour le chemin `/etc/claude` ; n'importe quel chemin que le processus runner peut lire fonctionne, donc ajustez les deux commandes et la valeur `--environment-secret-file` ensemble si vous en utilisez un différent.

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    Écrivez le secret d'environnement dans un fichier. La commande ci-dessous lit depuis votre terminal afin que le secret reste hors de l'historique du shell : collez la valeur que vous avez copiée, appuyez sur Entrée, puis Ctrl-D, et le `umask` du sous-shell rend le fichier lisible uniquement par son propriétaire.

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    Choisissez un répertoire de base, en remplaçant `<writable-dir>` dans la commande du runner ci-dessous par un chemin absolu que le runner peut écrire ou créer. Le runner crée le répertoire au démarrage, puis extrait les référentiels et crée des répertoires par session sous celui-ci. Sans `--base-dir`, il utilise `/workspace`, qui ne fonctionne que si ce répertoire existe déjà et est accessible en écriture ou si vous démarrez le runner en tant que root.

    Si le runner ne peut pas créer ou écrire dans le chemin, il se termine au démarrage avec une erreur nommant le répertoire au lieu de s'enregistrer. Consultez [Dépannage](/docs/fr/self-hosted-environments-deploy#troubleshooting).

    Ensuite, démarrez le runner avec `--environment-secret-file` et `--base-dir`. Le runner s'enregistre auprès de votre environnement et commence à interroger le travail. Si le runner se termine, redémarrez-le manuellement. Les déploiements en production exécutent le runner sous un orchestrateur qui redémarre les runners terminés, normalement avec un système de fichiers frais par redémarrage ; [Réutiliser un checkout pré-chauffé](/docs/fr/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) couvre la configuration de disque persistant prise en charge.

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="Vérifier que le runner apparaît">
    Retournez à la [page **Environnements cloud**](https://claude.ai/admin-settings/cloud-environments). Le statut de votre environnement passe de **Aucun runner déployé** à **Sain** en quelques secondes après le démarrage du runner ; ouvrez l'environnement et sélectionnez **Activité** pour voir le runner lui-même.
  </Step>

  <Step title="Router une session vers l'environnement">
    Démarrez une session à claude.ai/code et sélectionnez votre environnement dans le sélecteur d'environnement, où les environnements auto-hébergés apparaissent aux côtés des environnements hébergés par Anthropic. Le runner clone avec les identifiants git que l'hôte a déjà, donc choisissez un référentiel que cet hôte peut déjà cloner, ou un public ; les options d'identifiants pour les référentiels privés en production sont sur [Configurer git](/docs/fr/self-hosted-environments-deploy#configure-git). Le prochain runner disponible récupère la session en attente et enregistre `Picked up session <session-id>` ainsi que son nombre actif et sa capacité, afin que vous puissiez confirmer à partir de la propre sortie du runner quel hôte a pris la session. Regardez la session fonctionner et lisez les réponses de Claude à [claude.ai/code](https://claude.ai/code). Si la session reste en attente à la place, consultez [Dépannage](/docs/fr/self-hosted-environments-deploy#troubleshooting).
  </Step>
</Steps>

Le runner se termine par conception une fois que ses sessions actives se terminent ; consultez [Cycle de vie du runner](/docs/fr/self-hosted-environments#runner-lifecycle). Pour la production, déployez-le sous un orchestrateur qui le redémarre à la sortie. Consultez [Déployer en production](/docs/fr/self-hosted-environments-deploy).

<h2 id="send-a-follow-up-message-to-a-running-session">
  Envoyer un message de suivi à une session en cours d'exécution
</h2>

Une fois qu'une session s'exécute sur votre environnement, envoyez-lui un suivi à partir de la CLI `claude` sur n'importe quelle machine où vous êtes connecté avec `claude auth login` ; la commande n'a pas besoin de s'exécuter à partir de la machine qui a démarré la session. La commande publie un message :

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Pour `<session-id>`, passez l'ID nu `session_...` ou `cse_...` ou l'URL claude.ai/code de la session. Un envoi réussi imprime `Sent to cloud session.` avec l'ID de session et un lien de visualisation. Les formes d'ID acceptées, la sortie JSON, les exigences de compte et de politique, et la référence d'erreur sont sur [Envoyer des suivis à partir de la CLI](/docs/fr/claude-code-on-the-web#send-follow-ups-from-the-cli), car la commande fonctionne de la même manière contre les sessions hébergées par Anthropic.

<h2 id="what’s-next">
  Étapes suivantes
</h2>

* [Déployer en production](/docs/fr/self-hosted-environments-deploy) : durcir le déploiement, contrôler la sortie, configurer les identifiants git et exécuter la flotte sous Kubernetes ou Compose
* [Personnaliser les sessions](/docs/fr/self-hosted-environments-configuration) : scripts wrapper, hooks de cycle de vie, runners à la demande, serveurs MCP et permissions
* [Tester de bout en bout](/docs/fr/self-hosted-environments-testing) : un test de fumée CI qui dispatche une session et lit les réponses de Claude
