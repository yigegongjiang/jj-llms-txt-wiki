> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Exécuter Claude Code via un lanceur d'entreprise

> Acheminez les processus que Claude Code démarre à partir de son propre binaire, y compris le service d'arrière-plan et chaque session de vue agent, via un lanceur obligatoire avec CLAUDE_CODE_PROCESS_WRAPPER ou le paramètre processWrapper.

Certaines organisations exigent que chaque processus sur une station de travail démarre via un lanceur obligatoire. Le lanceur applique le bac à sable, les contrôles réseau ou l'injection de credentials dont dépend la posture de sécurité de l'entreprise, et un binaire qui démarre sans cela constitue une violation de politique.

`CLAUDE_CODE_PROCESS_WRAPPER` démarre chaque processus que Claude Code lance à partir de son propre binaire via votre lanceur : le service d'arrière-plan, chaque session qu'il héberge dans la [vue agent](/docs/fr/agent-view), et les relanceurs de Claude Code après une mise à jour. Définissez-le sur le chemin absolu de votre lanceur, et Claude Code exécute le lanceur avec la commande Claude Code comme arguments.

Un lanceur qui enveloppe la commande `claude` sur votre `PATH` ne peut pas atteindre le service d'arrière-plan ou les sessions qu'il héberge, car ils démarrent à partir du chemin direct du binaire sans consulter `claude`.

<Note>
  `CLAUDE_CODE_PROCESS_WRAPPER` nécessite Claude Code v2.1.208 ou ultérieur. Les versions antérieures ignorent la variable et démarrent chaque processus sans enveloppe. Le paramètre [`processWrapper`](/docs/fr/settings-reference#processwrapper) équivalent nécessite v2.1.210 ou ultérieur. Les versions antérieures l'ignorent comme clé inconnue, n'appliquent aucun lanceur et ne signalent aucune erreur.

  Après avoir déployé l'une ou l'autre forme, utilisez l'[étape Vérifier](#set-up-the-launcher) pour confirmer que la version en cours l'applique.
</Note>

<h2 id="what-the-launcher-covers">
  Ce que le lanceur couvre
</h2>

Avec `CLAUDE_CODE_PROCESS_WRAPPER` défini, Claude Code démarre chacun des processus suivants via votre lanceur :

* Le service d'arrière-plan que `claude agents` et les sessions d'arrière-plan démarrent à la demande.
* L'hôte terminal et la session Claude Code à l'intérieur de chaque ligne de vue agent, y compris les sessions de secours à chaud que le service maintient prêtes.
* Les sessions que le service redémarre après une mise à jour ou un plantage.
* Le relancement que Claude Code effectue de lui-même pour terminer l'installation d'une mise à jour, y compris l'action de redémarrage pour mise à jour de la vue agent.
* Les processus de session que [Remote Control](/docs/fr/remote-control) démarre. Nécessite Claude Code v2.1.210 ou version ultérieure.
* Les sessions coéquipier en volet divisé que [agent teams](/docs/fr/agent-teams) démarrent dans tmux ou iTerm2. Les volets coéquipier sont interactifs plutôt que des processus d'arrière-plan, mais Claude Code les démarre à partir de son propre binaire, donc le lanceur les couvre. Nécessite Claude Code v2.1.210 ou version ultérieure.

Sur Windows, la variable est ignorée : le contrat du lanceur dépend de `exec`, que Windows ne supporte pas. Une machine Windows avec la variable définie exécute chaque processus sans enveloppe et continue de fonctionner, et le seul signal est un avertissement dans le [journal de débogage](/docs/fr/troubleshooting). Si votre politique de lanceur couvre Windows, la variable ne la satisfait pas là : comptez les machines Windows comme non enveloppées lorsque vous planifiez le déploiement.

<h3 id="processes-that-start-outside-the-launcher">
  Processus qui démarrent en dehors du lanceur
</h3>

Les processus suivants ne démarrent pas via le lanceur :

* Un [service d'arrière-plan installé](/docs/fr/agent-view#the-supervisor-process) dont l'unité a été écrite avant la configuration du lanceur : `launchd` ou `systemd` démarre ce processus à partir de son fichier d'unité. `/status` et `claude daemon status` avertissent quand le service en cours d'exécution et le lanceur configuré ne correspondent pas, et les sessions que le service génère démarrent toujours via le lanceur une fois que le service redémarre avec la variable dans ses paramètres.
* Une session que vous démarrez vous-même dans un terminal, qui s'exécute comme vous l'avez invoquée. Pour couvrir ces sessions, mettez un script nommé `claude` dans un répertoire plus tôt sur `PATH` qui exécute votre lanceur avec le vrai binaire ; ne remplacez pas le lien symbolique géré. Le service d'arrière-plan et ses sessions démarrent sans une recherche `PATH`, donc les deux lanceurs ne s'empilent pas là.
* Le premier processus d'un lien profond `claude-cli://`, que le gestionnaire de protocole du système d'exploitation démarre directement. Tout ce que cette session démarre en arrière-plan par la suite s'exécute via le lanceur. Pour fermer complètement ce chemin, [empêchez l'enregistrement du gestionnaire](/docs/fr/deep-links#registration-and-supported-platforms) avec le paramètre `disableDeepLinkRegistration`.
* Le relancement que `--worktree` combiné avec `--tmux` effectue : le multiplexeur de terminal démarre ce volet, pas le binaire de Claude Code.
* L'hôte de messagerie native que [Claude in Chrome](/docs/fr/chrome) enregistre : le navigateur le démarre, pas le binaire de Claude Code.

<h3 id="helper-process-names-in-process-monitors">
  Noms des processus d'aide dans les moniteurs de processus
</h3>

Avec un lanceur configuré, `ps` et Activity Monitor n'affichent plus les étiquettes `claude bg-pty-host` et `claude bg-spare` de Claude Code pour les processus d'aide d'arrière-plan, car le `exec` du lanceur reconstruit la liste d'arguments. La perte des étiquettes est un effet secondaire, pas une dissimulation : les processus sont autrement inchangés, et Claude Code identifie ses propres processus par chemin binaire, jamais par nom d'affichage.

<h2 id="set-up-the-launcher">
  Configurer le lanceur
</h2>

<Steps>
  <Step title="Écrire le script du lanceur">
    Créez un script exécutable à un chemin absolu, tel que `/opt/corp/launcher`. Claude Code l'exécute avec la commande Claude Code complète comme arguments, et le script doit se terminer en appelant `exec "$@"` pour qu'il se remplace par Claude Code :

    ```bash theme={null}
    #!/bin/sh
    # Configuration de votre organisation : entrez dans le bac à sable, appliquez
    # les contrôles réseau ou injectez les credentials.
    exec "$@"
    ```

    Rendez-le exécutable avec `chmod +x`. La partie configuration est tout ce que votre lanceur doit faire avant que Claude Code s'exécute ; [le contrat du lanceur](#the-launcher-contract) ci-dessous énumère les règles que le script doit suivre.

    <Note>
      Si vous avez précédemment remplacé le lien symbolique `~/.local/bin/claude` par votre lanceur, restaurez le lien symbolique d'origine dans le même changement. Un lien symbolique remplacé fait démarrer la première session enveloppée du service d'arrière-plan via les deux lanceurs à la fois, et cela met l'installation dans un état géré en externe : `/doctor` le signale, la mise à jour automatique laisse le fichier en place, et le nettoyage des anciennes versions reste désactivé jusqu'à ce que l'installateur gère à nouveau ce chemin.
    </Note>
  </Step>

  <Step title="Définir CLAUDE_CODE_PROCESS_WRAPPER dans les paramètres">
    Définissez la variable dans le bloc `env` d'un fichier de paramètres pour que le service d'arrière-plan détaché l'hérite. Un `export` shell n'est pas suffisant : le service d'arrière-plan démarre à la demande, survit à votre shell et ne relit jamais les profils shell.

    Pour une machine, ajoutez-le à `~/.claude/settings.json`. Pour le déployer sur chaque machine de votre organisation, mettez le même bloc dans les [paramètres gérés](/docs/fr/managed-settings) :

    ```json theme={null}
    {
      "env": {
        "CLAUDE_CODE_PROCESS_WRAPPER": "/opt/corp/launcher"
      }
    }
    ```

    Quand plus d'une source définit la variable, la valeur des paramètres gérés remplace à la fois `~/.claude/settings.json` et une valeur exportée dans le shell, donc les utilisateurs ne peuvent pas pointer les auto-générations vers un lanceur différent.

    Le paramètre [`processWrapper`](/docs/fr/settings-reference#processwrapper) porte la même valeur qu'une clé de paramètres nommée et de haut niveau. Définissez-le quand votre organisation pousse les paramètres comme des clés individuelles plutôt qu'un bloc `env`. Le paramètre `processWrapper` nécessite Claude Code v2.1.210 ou ultérieur. Le fichier de paramètres suivant définit le même lanceur via la clé :

    ```json theme={null}
    {
      "processWrapper": "/opt/corp/launcher"
    }
    ```

    `CLAUDE_CODE_PROCESS_WRAPPER` a la priorité quand les deux sont définis.

    Parce que `processWrapper` est un paramètre nommé, une organisation qui le fournit via les [paramètres gérés à distance](/docs/fr/managed-settings#delivery-mechanisms) le voit listé sur la [boîte de dialogue d'approbation de sécurité](/docs/fr/server-managed-settings#security-approval-dialogs) aux côtés des autres paramètres qui exécutent des exécutables fournis par l'administrateur.

    Les paramètres de projet et locaux ne peuvent pas configurer le lanceur. Un fichier validé dans un référentiel ne doit pas pouvoir mettre un binaire devant chaque processus Claude Code sur la machine, donc Claude Code ignore `CLAUDE_CODE_PROCESS_WRAPPER` dans `.claude/settings.json` ou `.claude/settings.local.json` avec un avertissement dans le [journal de débogage](/docs/fr/troubleshooting), et ne lit jamais la clé `processWrapper` de ces fichiers.
  </Step>

  <Step title="Redémarrer le service d'arrière-plan et vos sessions">
    Un service d'arrière-plan en cours d'exécution et toute session `claude` ouverte lisent la variable une fois au démarrage, donc ils continuent de lancer des processus sans enveloppe jusqu'au redémarrage. Exécutez `claude daemon stop --any` pour arrêter le service à la demande ; la commande suivante qui en a besoin, comme `claude agents`, en démarre un enveloppé. Un [service installé](/docs/fr/agent-view#the-supervisor-process) prend `claude daemon stop` sans `--any`. Ensuite, redémarrez vos sessions `claude` ouvertes.

    Sur les machines que vous ne pouvez pas redémarrer manuellement, la première session démarrée après le push des paramètres retire automatiquement un service à la demande sans enveloppe restant. Une machine où aucune nouvelle session ne démarre garde son service sans enveloppe jusqu'à ce qu'une le fasse, et un service installé a toujours besoin du redémarrage dans cette étape.
  </Step>

  <Step title="Vérifier">
    Exécutez `/status` dans une session : l'entrée Self-exec affiche la commande de lancement résolue et avertit quand le service d'arrière-plan en cours d'exécution ne correspond pas. `claude daemon status` imprime les mêmes informations depuis le shell, y compris après que vous ayez annulé la variable, quand `/status` n'affiche plus l'entrée.
  </Step>
</Steps>

<h2 id="the-launcher-contract">
  Le contrat du lanceur
</h2>

Quand le lanceur ne peut pas s'exécuter, Claude Code refuse de démarrer le processus au lieu de le démarrer sans enveloppe. Sur Windows, [la variable est ignorée](#what-the-launcher-covers) et les processus démarrent sans enveloppe. Claude Code tient le script à ces règles :

* **Terminez par `exec "$@"`.** Un lanceur qui crée un enfant et se termine laisse un processus Claude Code orphelin que le service d'arrière-plan ne peut pas suivre. La vue agent marque une telle session comme échouée avec un message nommant le lanceur, et le service récolte ce que le lanceur a laissé derrière.
* **Ne réordonnez pas, n'absorbez pas et ne préparez pas les arguments.** Le premier argument est le binaire Claude Code et tout ce qui suit est son argv.
* **Passez chaque variable d'environnement héritée via `exec`.** Ajouter des variables, comme les credentials injectées, est correct ; supprimer les héritées ne l'est pas.
  * Les jetons d'authentification par session, la sélection du modèle et du fournisseur, et `CLAUDE_CODE_PROCESS_WRAPPER` lui-même voyagent tous sur l'environnement hérité, donc un lanceur qui le reconstruit à partir d'une liste d'autorisation casse les sessions qu'il démarre, et `/status` signale une non-correspondance du lanceur.
  * Si le lanceur doit entrer dans un espace de noms ou un bac à sable qui réinitialise l'environnement, réexportez l'environnement hérité à l'intérieur verbatim.
* **Atteignez `exec` en environ trois secondes chaque fois que le lanceur s'exécute.** Une expédition d'arrière-plan à froid exécute le lanceur deux fois en série avant le premier octet de sortie, donc faites un travail lent comme un échange d'authentification unique paresseusement ou à partir d'un cache.
* **Tolérez d'être invoqué de l'intérieur de vous-même.** Claude Code applique le lanceur à chaque auto-génération imbriquée, donc un lanceur qui acquiert une ressource exclusive doit détecter qu'il la détient déjà.
* **N'écrivez pas sur le terminal avant que Claude Code ne démarre.** Tout ce qui est imprimé avant le `exec` est signalé comme la cause du plantage si la session meurt avant l'initialisation.

<h3 id="format-of-the-launcher-value">
  Format de la valeur du lanceur
</h3>

`CLAUDE_CODE_PROCESS_WRAPPER` et le paramètre `processWrapper` prennent le même format. Pour la plupart des lanceurs, la valeur est le chemin absolu du script, comme `/opt/corp/launcher`.

Pour passer à votre lanceur ses propres arguments, écrivez-les après le chemin. Claude Code analyse la valeur comme une liste d'arguments, pas une commande shell :

* L'espace blanc sépare les jetons, et les guillemets doubles groupent un jeton qui contient des espaces.
* Une valeur qui commence par `[` est lue comme un tableau de chaînes JSON, comme `["/opt/corp/launcher", "--profile", "cc"]`.
* La syntaxe shell ne fonctionne pas : il n'y a pas d'expansion de variable ou de globbing, et un opérateur non cité comme `;`, `|`, `&` ou `$(` est rejeté comme une erreur de configuration plutôt que réinterprété.

Quand la valeur ne peut pas être utilisée, Claude Code refuse de démarrer le processus affecté et [signale la raison](/docs/fr/errors#claude_code_process_wrapper-launcher-errors).

<h2 id="relationship-to-claude_code_shell_prefix">
  Relation avec `CLAUDE_CODE_SHELL_PREFIX`
</h2>

`CLAUDE_CODE_PROCESS_WRAPPER` enveloppe les propres processus de Claude Code et passe la commande via des jetons argv séparés pour que le lanceur `exec`. [`CLAUDE_CODE_SHELL_PREFIX`](/docs/fr/env-vars) enveloppe les commandes shell que Claude Code exécute en votre nom, comme les appels d'outil Bash, les hooks et les commandes qui démarrent les serveurs MCP stdio, et passe chacun comme une seule chaîne entre guillemets shell dans `$1` pour que le wrapper la réévalue. Un lanceur écrit pour l'un ne fonctionne pas comme l'autre.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Vue agent](/docs/fr/agent-view) : les sessions d'arrière-plan et le processus superviseur que le lanceur couvre
* [Variables d'environnement](/docs/fr/env-vars) : l'entrée de référence `CLAUDE_CODE_PROCESS_WRAPPER`
* [Paramètres gérés](/docs/fr/permissions#managed-settings) : livrez le bloc `env` sur une flotte
* [Référence d'erreur du lanceur](/docs/fr/errors#claude_code_process_wrapper-launcher-errors) : les messages de refus et comment récupérer
