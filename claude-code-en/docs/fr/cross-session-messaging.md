> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Messagerie entre vos autres sessions Claude Code

> Laissez Claude lister et envoyer des messages à vos autres sessions Claude Code sur cette machine, et atteindre vos sessions sur d'autres machines ou sur le web.

<Note>
  La messagerie entre sessions nécessite Claude Code v2.1.224 ou version ultérieure sur macOS et Linux, y compris Linux à l'intérieur de WSL 2. Sur Windows natif, elle nécessite Claude Code v2.1.234 ou version ultérieure. Lorsqu'une session répond aux exigences, la messagerie est activée sans rien à configurer. Consultez [Disponibilité](#availability) pour les exigences des fournisseurs et comment confirmer qu'une session la possède.
</Note>

La messagerie entre sessions permet à Claude de livrer un message d'une de vos sessions Claude Code à une autre. Lorsqu'une modification dans une session casse ce qu'une autre construit, Claude peut avertir cette session avant que vous ne le remarquiez. Lorsqu'une session résout une question sur laquelle une autre est bloquée, Claude peut envoyer la réponse à travers.

Un message est un morceau de texte qu'un Claude écrit à un autre, jamais l'historique de conversation ou les fichiers de l'expéditeur. Pour déplacer une conversation entière ou son contexte, [reprenez la session](/docs/fr/sessions#resume-a-session) à la place.

Claude utilise deux outils pour cela : `ListAgents` pour découvrir quels agents il peut atteindre, et `SendMessage` pour livrer un message à l'un d'eux par nom. Avec le même outil `SendMessage`, Claude peut également envoyer des messages à des [sous-agents](/docs/fr/sub-agents#resume-subagents) et des coéquipiers [d'équipe d'agents](/docs/fr/agent-teams) au sein d'une seule session ou équipe. Cette page couvre les messages entre vos sessions indépendantes.

<h2 id="when-to-use-cross-session-messaging">
  Quand utiliser la messagerie entre sessions
</h2>

Utilisez la messagerie quand l'une de vos sessions a quelque chose qu'une autre session a besoin en cours de tâche. Claude peut envoyer un message de lui-même quand il voit le besoin, par exemple après avoir fait un changement qui affecte le travail qu'une autre session fait, ou vous pouvez lui demander d'en envoyer un. Les cas courants :

* **Transmettre une découverte** : quand une session découvre un changement cassant ou prend une décision, Claude la résume pour la session travaillant sur la zone affectée, au lieu que vous la réexpliquiez là.
* **Coordonner les worktrees parallèles** : quand les sessions travaillent le même dépôt dans des [worktrees](/docs/fr/worktrees) séparés, Claude peut dire aux autres sessions ce qui a atterri.
* **Obtenir le statut du travail de longue durée** : faire rapporter une migration ou une exécution de test à la session que vous regardez, ou demandez-le vous-même de là. Si cette session est sur cette machine, Claude peut aussi [lui demander un avis quand elle devient inactive ou se termine](#get-a-notice-when-another-session-goes-idle).
* **Envoyer des messages entre machines** : atteindre l'une de vos sessions sur une autre machine ou sur le web.

Utilisez la messagerie entre sessions indépendantes que vous démarrez et dirigez vous-même. Claude Code a une fonctionnalité dédiée pour chacune des autres façons d'exécuter ou d'atteindre plusieurs sessions, donc utilisez celle construite pour ce que vous faites à la place :

* Pour continuer une conversation dans un autre terminal, ou partager son contexte avec une nouvelle session, [reprenez la session](/docs/fr/sessions#resume-a-session)
* Pour une équipe coordonnée de sessions que Claude crée et supervise, utilisez les [équipes d'agents](/docs/fr/agent-teams)
* Pour regarder et diriger de nombreuses sessions d'un seul endroit, utilisez la [vue agent](/docs/fr/agent-view)
* Pour diriger une session vous-même depuis votre téléphone ou un autre appareil, plutôt que d'avoir les sessions se envoyer des messages, utilisez la [Télécommande](/docs/fr/remote-control)
* Pour pousser des événements externes, comme les résultats CI ou les messages de chat, dans une session, utilisez les [canaux](/docs/fr/channels)

<h2 id="message-another-session">
  Envoyer un message à une autre session
</h2>

Quand l'une de vos sessions apprend quelque chose qu'une autre session a besoin, comme une découverte, un statut ou une décision, Claude la transmet au lieu que vous copiiez-colliez entre les terminaux. Claude découvre la cible avec `ListAgents` et envoie avec `SendMessage`, donc vous n'appelez jamais l'un ou l'autre outil vous-même. Claude peut décider d'envoyer un message sans être demandé, et vous pouvez aussi demander un.

Pour en demander un vous-même, dites à Claude ce que vous voulez que l'autre session sache ou fasse. Cet exemple est une invite que vous tapez, pas un message que Claude envoie :

```text wrap theme={null}
Demandez à la session exécutée dans mon autre terminal si la migration est terminée
```

Claude écrit le message réel lui-même, donc votre invite peut laisser le contenu à Claude. Cette invite demande un résumé sans dicter sa formulation, et ce que Claude envoie varie :

```text wrap theme={null}
Expliquez ce que nous venons de faire à la session travaillant sur l'API des paiements
```

Pour nommer la cible vous-même, mentionnez la session dans votre invite : tapez `@` suivi des premières lettres du nom de la session et choisissez la session dans la saisie semi-automatique, de la même manière que vous [@-mentionnez un sous-agent](/docs/fr/sub-agents#invoke-subagents-explicitly). Nécessite Claude Code v2.1.232 ou ultérieure. Claude Code insère la mention, comme `@api-worker`, et dit à Claude quelle session elle nomme, donc Claude peut envoyer un message à cette session sans lister vos sessions en premier. Cette invite nomme la cible avec une mention :

```text wrap theme={null}
Faites savoir à @api-worker que la migration de schéma est terminée
```

La saisie semi-automatique liste vos autres sessions actives sur cette machine. Deux cas nécessitent plus que les premières lettres d'un nom :

* **Une session au-delà de cette machine** : une session cloud ou Télécommande n'apparaît dans la saisie semi-automatique qu'après que Claude ait listé ou envoyé des messages à vos sessions au-delà de cette machine, donc demandez à Claude de les lister en premier.
* **Un nom avec un espace ou d'autres caractères en dehors des lettres, chiffres, tirets et traits de soulignement** : tapez-le entre guillemets doubles, comme `@"release notes"`. Quand vous choisissez la session dans la saisie semi-automatique, Claude Code insère les guillemets pour vous.

Vous pouvez aussi taper la mention sans le sélecteur. Quand plus d'une session active répond au nom mentionné, Claude vous demande laquelle vous voulez dire avant d'envoyer.

Pour ce à quoi ressemble le message que Claude écrit quand il arrive, y compris un exemple d'un, consultez [à quoi ressemble un message](#what-a-message-looks-like).

<h3 id="message-delivery">
  Livraison des messages
</h3>

Le Claude récepteur lit le message entre les appels d'outils lors d'un tour actif, donc un outil en cours d'exécution n'est jamais interrompu. Quand la session réceptrice est inactive, Claude Code démarre un nouveau tour avec le message.

Un message d'une autre session arrive sous forme de texte brut. S'il mentionne un fichier ou une [ressource MCP](/docs/fr/mcp#use-mcp-resources) avec `@`, Claude voit la mention telle qu'écrite et Claude Code n'attache rien, que le message démarre un nouveau tour ou arrive pendant un. Claude peut toujours ouvrir un chemin mentionné sur la machine réceptrice avec ses propres outils, sous réserve des permissions de cette session. Avant v2.1.251, une mention `@` dans un message qui a démarré un nouveau tour attachait le fichier ou la ressource MCP du côté récepteur.

Claude Code refuse un message dans les cas suivants :

* Le message est [au-delà du plafond de taille](#limitations). Claude Code le refuse dans la session d'envoi, avant qu'il ne parte.
* Une rafale rapide vers une session sur cette machine a atteint [ce que la boîte de réception de cette session accepte](#limitations). Claude Code refuse d'autres messages à cette session.
* La cible de réponse sur cette machine échoue une vérification de sécurité, comme une cible symlink ou un point de terminaison qui n'est pas le processus attendu. [Refuser d'envoyer un message entre sessions](/docs/fr/errors#refusing-to-send-a-cross-session-message) liste ces vérifications.
* Claude adresse le message au nom de sa propre session, comme décrit sous [Voir quelles sessions Claude peut atteindre](#see-which-sessions-claude-can-reach).

La session réceptrice vérifie chaque message arrivant par rapport à ses propres [contrôles entrants](#control-inbound-messages), et la vérification aboutit à l'un de trois résultats :

* **Livré** : Claude Code transmet le message au Claude récepteur.
* **Retenu** : Claude Code met le message de côté non livré. Un message retenu atteint Claude seulement quand vous l'approuvez ou qu'un mode ou changement de paramètres ultérieur le permet.
* **Refusé** : Claude Code supprime le message sans le livrer.

Une fois livré, le message compte vers l'[utilisation](/docs/fr/costs) comme une invite que vous tapez, et le Claude récepteur peut répondre à l'expéditeur de la même manière, sauf dans le [cas unidirectionnel entre machines](#message-sessions-on-other-machines).

Les limites de permission restent par session. Claude est instruit de ne jamais demander à une autre session une action qui a été refusée ou bloquée dans sa propre session, ou que ses propres paramètres de permission bloqueraient, et de router ce travail vers vous à la place. Du côté récepteur, les [invites de permission propres de la session réceptrice et les règles s'appliquent toujours](#how-a-session-treats-an-incoming-message) à tout ce que le message demande.

<h3 id="get-a-notice-when-another-session-goes-idle">
  Obtenir un avis quand une autre session devient inactive
</h3>

Claude peut demander à l'une de vos sessions sur cette machine d'envoyer un avis quand cette session devient inactive ou se termine. Inactif ici signifie que la session a terminé un tour sans rien en attente. Utilisez-le quand vous attendez une tâche longue dans une autre session et voulez entendre quand elle est terminée au lieu de vérifier. Nécessite Claude Code v2.1.236 ou ultérieure dans les deux sessions.

<h4 id="ask-for-a-notice">
  Demander un avis
</h4>

Dites à Claude ce que vous attendez. Cette invite demande un avis de la session de migration :

```text wrap theme={null}
Dites-moi quand la session de migration termine ce sur quoi elle travaille
```

Claude s'abonne avec l'entrée `notify_when_idle` de l'outil `SendMessage`, soit attachée à un message qu'il envoie de toute façon, soit seule. Seule, Claude Code s'abonne sans démarrer un tour ou dépenser des jetons dans la session regardée, et envoie l'avis immédiatement si cette session est déjà inactive. Attaché à un message, Claude Code livre d'abord le message et envoie l'avis plus tard.

<h4 id="what-each-session-shows">
  Ce que chaque session affiche
</h4>

La session regardée affiche une ligne disant qu'un autre processus a demandé à être averti quand la session est inactive. La session demandante affiche l'avis comme une ligne nommant la session regardée. La ligne peut inclure l'heure à laquelle le tour de cette session s'est terminé et un statut d'une ligne de ce tour. Si la session demandante est inactive, Claude Code démarre un nouveau tour avec l'avis.

<h4 id="limits">
  Limites
</h4>

L'avis est unique : Claude Code l'envoie une fois de la session regardée, et aucune session n'interroge l'autre. Si aucun avis n'arrive dans les 12 heures, Claude Code supprime l'abonnement et le dit à Claude, donc il n'attend pas indéfiniment.

Les [contrôles entrants](#control-inbound-messages) de chaque côté s'appliquent à un avis comme un message :

* **`refuse` de chaque côté** : rien n'arrive. La session regardée supprime la demande sans l'enregistrer ou y répondre, donc l'abonnement expire sans réponse après 12 heures, et une session demandante avec `refuse` ne s'abonne jamais.
* **`hold` de chaque côté** : l'avis arrive avec moins. La session regardée laisse le statut d'une ligne de côté, et la session demandante affiche l'avis dans votre transcription sans le livrer à Claude.

Seul le Claude dans votre conversation principale peut s'abonner, et seulement à vos sessions sur cette machine. Quand un sous-agent ou un coéquipier d'équipe d'agents définit `notify_when_idle`, Claude Code ne fait aucun abonnement et le lui dit. Quand Claude demande un avis à un autre agent, comme un coéquipier, un sous-agent ou une session au-delà de cette machine, Claude Code refuse l'appel entier, y compris tout message attaché, et rapporte le refus à Claude pour qu'il puisse renvoyer le message sans la demande.

<h3 id="see-which-sessions-claude-can-reach">
  Voir quelles sessions Claude peut atteindre
</h3>

Claude trouve la cible d'un message de lui-même, donc vous n'avez pas besoin d'exécuter quoi que ce soit avant de lui demander d'envoyer. Pour voir vous-même quelles sessions Claude peut atteindre, exécutez la commande `/list-agents`. La première ligne, quand présente, est le nom de cette session, celui que vos autres sessions utilisent pour lui envoyer des messages. Les lignes ci-dessous sont les sessions que Claude peut atteindre :

* **Sous-agents** : agents exécutés à l'intérieur de la session actuelle.
* **Coéquipiers** : les coéquipiers de la propre [équipe d'agents](/docs/fr/agent-teams) de cette session. Avant v2.1.239, les coéquipiers n'apparaissaient pas dans la liste, bien que Claude puisse déjà les envoyer des messages par nom.
* **Vos autres sessions locales** : sessions Claude Code exécutées sur la même machine, y compris les [sessions en arrière-plan](/docs/fr/agent-view). Une session n'apparaît que quand elle lie une [socket de boîte de réception](#the-sessions-inbox-socket).
* **Vos sessions cloud** : vos sessions [Claude Code sur le web](/docs/fr/claude-code-on-the-web), affichées pendant que cette session est connectée à la [Télécommande](/docs/fr/remote-control). Claude Code les étiquette `cloud` dans la liste.
* **Vos sessions Télécommande sur d'autres machines** : affichées pendant que cette session est connectée à la [Télécommande](/docs/fr/remote-control), et étiquetées `Remote Control`. Claude Code affiche `offline` comme le statut d'une session dont la connexion Télécommande a chuté.

Cette session n'est pas l'une des lignes. Si Claude adresse un message au nom de sa propre session, Claude Code le refuse et dit à Claude que la cible est la session actuelle. Avant v2.1.239, la liste n'affichait pas le nom de cette session, et Claude Code rapportait un message envoyé à lui comme un agent qu'il ne pouvait pas trouver.

Pendant que cette session est connectée à la [Télécommande](/docs/fr/remote-control), Claude Code retient certains détails de vos sessions locales de la sortie `/list-agents`, sans changer ce que Claude lui-même voit quand il cherche une session à envoyer des messages :

* **Répertoires de travail** : il laisse de côté le répertoire de travail de chaque session locale.
* **Noms de session** : il laisse de côté tout nom de session qu'il ne peut pas attribuer à une personne, donc une ligne laissée sans nom lit `(unnamed session)`.
* **La première ligne** : il laisse de côté la ligne avec le nom de cette session à moins que vous ayez tapé ce nom à ce terminal, avec `--name` ou avec `/rename` et le nom, depuis que vous avez lancé ou repris la session.

Quand la sortie liste quelque chose, elle se termine par une note disant que les détails ont été retenus. Exécuter `/rename` suivi d'un nom inutilisé à un clavier de sa propre session donne à cette session un nom qui apparaît dans la sortie.

Claude Code lit vos listes de sessions cloud et Télécommande les plus récentes en premier et s'arrête après un nombre limité de pages pour chacune. Si votre compte a plus de ces sessions que ce qui rentre, Claude Code ne liste pas les plus anciennes, et Claude ne peut pas les envoyer des messages par nom. Quand cela se produit, Claude Code le dit dans la liste, et Claude voit la même note quand il envoie un message.

Claude adresse une session au-delà de cette machine par nom, de la même manière qu'une session locale. Consultez [Envoyer des messages aux sessions sur d'autres machines](#message-sessions-on-other-machines) pour comment ces messages voyagent.

Une session répond au nom que vous définissez avec la commande [`/rename`](/docs/fr/commands) ou l'indicateur [`--name`](/docs/fr/cli-reference#cli-flags). Quand vous n'en définissez pas un, Claude Code nomme la session lui-même. Pour une session interactive, c'est le nom affiché dans les [listes de sessions exécutées](/docs/fr/sessions#name-your-sessions).

Quand vous renommez une session, Claude Code met aussi à jour l'enregistrement partagé que vos autres sessions utilisent pour chercher le nom de la session. S'il ne peut pas mettre à jour cet enregistrement, il vous avertit dans la sortie `/rename` que d'autres sessions peuvent toujours afficher l'ancien nom. Exécutez la session avec [`--debug`](/docs/fr/cli-reference#cli-flags), et Claude Code enregistre la cause de la mise à jour échouée.

Quand vous renommez une session, ou démarrez ou reprenez une interactive, avec un nom qu'une autre session active sur cette machine utilise déjà, Claude Code laisse le nom avec la session qui l'a déjà et [renomme le vôtre en une variante](/docs/fr/sessions#name-your-sessions). Les sessions peuvent toujours partager un nom, par exemple quand l'une d'elles exécute une version antérieure de Claude Code ou le nom partagé en est un que Claude Code a généré. À moins que cette session ne soit connectée à la Télécommande, Claude Code affiche le répertoire de travail de chaque session locale dans la sortie `/list-agents`, donc vous pouvez distinguer les sessions de même nom quand elles s'exécutent dans des répertoires différents. Claude adresse le message de l'une de deux façons, selon le nombre de sessions actives qui répondent au nom :

* **Une session répond au nom** : Claude Code livre le message sur le nom seul.
* **Plusieurs sessions partagent le nom, ou Claude Code ne pouvait pas vérifier partout où vos sessions s'exécutent** : Claude ajoute un court identifiant à chaque ligne de sa liste et utilise l'identifiant dans l'adresse.

<h3 id="message-sessions-on-other-machines">
  Envoyer des messages aux sessions sur d'autres machines
</h3>

Comment un message voyage, et s'il passe par les serveurs Anthropic, dépend de l'endroit où la session cible s'exécute :

| Où la session cible s'exécute                            | Comment le message voyage                                                                                                         |
| :------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| Sur cette machine                                        | Sur une socket par session sur macOS et Linux, ou un tuyau nommé par session sur Windows natif, jamais par les serveurs Anthropic |
| Sur une autre de vos machines                            | Par les serveurs Anthropic, arrivant sur la connexion [Télécommande](/docs/fr/remote-control) de cette machine                         |
| Sur [Claude Code sur le web](/docs/fr/claude-code-on-the-web) | Par les serveurs Anthropic, directement à la session cloud                                                                        |

Démarrer une conversation avec une session sur une autre de vos machines nécessite Claude Code v2.1.225 ou ultérieure et une cible qui [apparaît dans la liste](#see-which-sessions-claude-can-reach). Avant v2.1.225, Claude ne pouvait que répondre à un message qui arrivait d'une.

Vous pouvez envoyer un message à une session affichée comme `offline` dans [la liste](#see-which-sessions-claude-can-reach), une dont la connexion Télécommande a chuté. L'envoi passe, mais le message n'arrive qu'après que la machine de cette session se reconnecte. Claude est informé de cela quand il envoie.

La livraison sur la même machine fonctionne partout où la fonctionnalité est activée. Chaque session s'enregistre dans des fichiers sur disque. Quand Claude liste ou envoie des messages à vos sessions locales, Claude Code lit ces fichiers pour trouver les sessions, donc deux sessions ne peuvent se atteindre que quand elles peuvent voir les mêmes fichiers.

Un conteneur a son propre système de fichiers, donc une session à l'intérieur et une session sur l'hôte ne peuvent pas se atteindre. Deux sessions à l'intérieur du même conteneur peuvent toujours s'envoyer des messages, y compris sur un [exécuteur auto-hébergé](/docs/fr/self-hosted-environments). Une session à l'intérieur de WSL 2 et une session Windows native sur le même ordinateur ne peuvent pas non plus se atteindre, car elles s'enregistrent sous des répertoires personnels différents et écoutent sur des types de socket différents.

Pendant que cette session est connectée à la Télécommande, quand vous envoyez un message à une session sur une autre de vos machines, Claude Code affiche le message dans la conversation de cette session sous le nom Télécommande de cette session. Le Claude sur cette machine peut répondre à ce nom. Par exemple, quand cette session est connectée à la Télécommande comme `laptop-graceful-unicorn` et que vous envoyez un message à votre bureau, vous voyez le message dans la session du bureau sous `laptop-graceful-unicorn`.

Si cette session n'est pas connectée à la Télécommande quand Claude envoie à une session au-delà de cette machine, le message passe toujours, mais sans une [adresse de réponse](#what-a-message-looks-like), donc le Claude récepteur ne peut pas y répondre. Claude est informé de cela quand il envoie.

Pour exiger votre approbation avant que tout message ne dépasse cette machine, définissez [`isolatePeerMachines`](#require-approval-for-cross-machine-messages).

<h2 id="how-a-session-treats-an-incoming-message">
  Comment une session traite un message entrant
</h2>

Quand la session A envoie un message à la session B, Claude Code dit au Claude de B que le message venait d'une autre session, pas de vous, et limite ce que le message peut faire :

* **Il ne peut rien approuver** : un message d'une autre session ne compte jamais comme votre consentement, donc il ne peut pas répondre à une invite de permission en attente en votre nom.
* **Il ne peut pas changer la configuration** : Claude Code instruit le Claude récepteur de ne jamais changer les paramètres de permission, `CLAUDE.md` ou d'autres configurations parce qu'une autre session l'a demandé.
* **Les commandes ne s'exécutent pas** : une commande dans le texte du message, comme `/compact`, arrive sous forme de texte brut. Claude Code ne l'exécute jamais.
* **Les invites de permission se déclenchent toujours** : si agir sur le message nécessite une permission que la session réceptrice n'a pas, vous voyez la même invite que vous verriez pour tout autre travail.

<h3 id="what-a-message-looks-like">
  À quoi ressemble un message
</h3>

Quand un message arrive, Claude Code l'affiche dans la conversation comme un aperçu d'une ligne atténué, et la ligne d'aperçu reste dans la conversation après. L'aperçu porte le nom de l'expéditeur et la première ligne du message, coupée avec `…` quand elle est longue, comme `› Message from @api-worker: Schema migration finished (ctrl+o to expand)`. Avant v2.1.247, Claude Code affichait le message arrivant en entier au lieu d'un aperçu.

L'un de ceux-ci affiche le texte complet :

* Appuyez sur `Ctrl+O` pour ouvrir la [visionneuse de transcription](/docs/fr/interactive-mode#transcript-viewer) et lire le texte complet sous le nom de session de l'expéditeur.
* Dans une session démarrée avec [`--verbose`](/docs/fr/cli-reference#cli-flags), Claude Code affiche le texte complet au lieu de l'aperçu.

L'aperçu raccourcit seulement ce que vous voyez. Que vous l'agrandissiez ou non, Claude lit le message complet.

Claude reçoit le message avec le nom de l'expéditeur et une adresse de réponse, sauf pour un [message unidirectionnel entre machines](#message-sessions-on-other-machines), qui ne porte pas d'adresse de réponse. Au-delà du nom et de l'adresse de réponse, le Claude récepteur obtient le texte du message, jamais l'historique de conversation ou les fichiers de l'expéditeur. [Livraison des messages](#message-delivery) couvre les mentions `@` dans le texte.

Un message qu'un [sous-agent](/docs/fr/sub-agents) a écrit arrive sous le nom de la session d'envoi, avec le sous-agent identifié dans le texte du message. Une réponse à celui-ci atteint la conversation principale de cette session, pas le sous-agent.

Cet exemple est un message qu'un Claude a écrit à un autre, tel qu'il se lit en entier quand vous l'agrandissez :

```text wrap theme={null}
Schema migration finished
The new column is tenant_id, and rebasing on main is safe now.
```

<h3 id="control-inbound-messages">
  Contrôler les messages entrants
</h3>

Définissez [`crossSessionInbound`](/docs/fr/settings-reference#crosssessioninbound) pour choisir ce qu'une session fait avec les messages arrivant de vos autres sessions :

| Valeur   | Comportement                                                                                                                                                                                                                   |
| :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `accept` | Claude Code livre chaque message à Claude                                                                                                                                                                                      |
| `hold`   | Claude Code affiche un avis pour chaque message et ne le livre pas. Si un `accept` s'applique plus tard, selon les [règles de précédence](/docs/fr/settings-reference#crosssessioninbound), Claude Code libère les messages retenus |
| `refuse` | Claude Code supprime chaque message sans le livrer                                                                                                                                                                             |

Au-delà de l'édition d'un fichier de paramètres, vous pouvez sélectionner la valeur dans la ligne `/config` **Messages from your other sessions**. Claude Code écrit la valeur que vous sélectionnez dans vos paramètres utilisateur. La ligne nécessite Claude Code v2.1.232 ou ultérieure et n'apparaît pas pendant que les paramètres gérés ou l'indicateur `--settings` définit la clé, car une valeur de paramètres utilisateur ne s'appliquerait pas alors. Claude Code rejette le raccourci `/config crossSessionInbound=value` pour cette clé.

Pour voir quelle valeur s'applique, suivez les règles de précédence `crossSessionInbound` dans la [référence des paramètres](/docs/fr/settings-reference#crosssessioninbound).

Quand aucune valeur ne s'applique, Claude Code décide par message à partir des modes de permission des deux sessions. Il groupe les sessions qui [contournent les invites de permission](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode) dans une classe, et chaque autre session dans l'autre. Le mode Plan compte comme contournement dans les sessions de terminal interactif avec les permissions de contournement disponibles, et [auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), `acceptEdits` et `dontAsk` comptent comme invitant :

* **La session réceptrice invite pour les permissions** : Claude Code livre chaque message. Il retient un pour votre approbation seulement quand la session d'envoi s'identifie comme contournant les invites de permission.
* **La session réceptrice contourne les invites de permission** : Claude Code retient chaque message pour votre approbation. Il livre un seulement quand la session d'envoi s'identifie aussi comme contournant.

Quand le défaut retient un message, Claude Code ouvre une boîte de dialogue d'approbation dans la session réceptrice. La boîte de dialogue affiche l'expéditeur et un aperçu :

* **Approuver** livre ce message à Claude.
* **Refuser**, ou fermer la boîte de dialogue, le supprime.
* Quand la boîte de dialogue reste sans réponse au-delà de la date limite [`dialogExpiry`](/docs/fr/settings-reference#dialogexpiry), Claude Code la ferme et supprime le message. La date limite par défaut est de cinq minutes.
* Pendant qu'aucun terminal n'est attaché à une [session en arrière-plan](/docs/fr/agent-view), Claude Code laisse la boîte de dialogue ouverte au-delà de la date limite. Après que vous attachiez, si la boîte de dialogue reste sans réponse pendant une période de date limite complète, Claude Code la ferme et supprime le message.
* Si la classe de mode de permission de cette session change pendant que les messages sont retenus, Claude Code réapplique les règles entrantes, livre les messages qu'elles acceptent maintenant, et affiche un avis.
* Si un changement de paramètres rend `refuse` applicable pendant que les messages sont retenus, Claude Code supprime chaque message retenu et rapporte un refus à chaque expéditeur qu'il peut atteindre.

Quand l'expéditeur est une session sur la même machine, Claude Code envoie un avis vers elle quand le récepteur retient le message, et un suivi quand le récepteur livre, refuse ou l'expire plus tard. L'avis atteint le Claude d'envoi, donc il sait de ne pas continuer à attendre un message que l'autre session n'a pas lu.

Dans une session d'envoi interactive, l'avis apparaît dans la transcription. Un expéditeur [`claude -p`](/docs/fr/headless) le reçoit dans la [sortie en flux](/docs/fr/headless#stream-responses) comme un [message `system` informatif](/docs/fr/agent-sdk/typescript#sdkinformationalmessage). Les avis aux expéditeurs `claude -p` nécessitent Claude Code v2.1.271 ou ultérieur.

Si le récepteur refuse le message, l'avis de l'expéditeur dit que le récepteur n'accepte pas les messages entre sessions et dit au Claude de l'expéditeur de ne pas attendre ou renvoyer.

Claude Code retient au maximum 100 messages, séparément de la file d'attente de livraison, et au-delà de cela supprime les plus anciens.

<h3 id="non-interactive-sessions">
  Sessions non-interactives
</h3>

Claude Code lie une socket de boîte de réception pour une session [`claude -p`](/docs/fr/headless) comme une interactive, donc un worker `-p` de longue durée peut recevoir des messages et apparaît dans la liste. Quand vous démarrez une session en [mode nu](/docs/fr/headless#start-faster-with-bare-mode), Claude Code ne lie pas la socket, donc cette session ne peut pas recevoir de messages et n'apparaît pas dans la liste d'agents.

Une session `-p` ne peut pas afficher la boîte de dialogue d'approbation. Quand le [défaut entrant](#control-inbound-messages) retient un message là, Claude Code le garde pour la même date limite [`dialogExpiry`](/docs/fr/settings-reference#dialogexpiry) que la boîte de dialogue utilise, cinq minutes par défaut :

* **Avant la date limite** : si un mode ou changement de paramètres permet le message, Claude Code le livre.
* **Au-delà de la date limite** : Claude Code supprime le message et le rapporte comme expiré à un expéditeur qu'il peut atteindre.

Définissez `dialogExpiry` à `"never"` pour garder les messages retenus par défaut jusqu'à ce que la session se termine. Un message retenu par un paramètre `hold` explicite n'expire pas ; Claude Code le livre seulement quand un `accept` s'applique plus tard.

Quand la session se termine avec des messages toujours retenus, Claude Code les rapporte comme expirés à chaque expéditeur qu'il peut atteindre. Avant v2.1.225, aucune date limite ne s'appliquait dans une session `-p` : un message retenu restait retenu à moins qu'un changement de mode de permission pendant l'exécution le livre, et une session qui se terminait avec des messages retenus ne rapportait rien à leurs expéditeurs.

Pour laisser un worker `-p` prendre des messages sans surveillance, démarrez-le avec `crossSessionInbound` défini à `accept` dans sa valeur `--settings`. Un `accept` dans vos paramètres utilisateur fonctionne aussi mais s'applique à chaque session que vous exécutez.

<h3 id="the-sessions-inbox-socket">
  La socket de boîte de réception de la session
</h3>

Lisez cette section quand une session que vous attendez n'est pas dans la liste d'agents, quand vous voulez qu'un script ou hook poste dans une session, ou quand une commande sandboxée ne peut pas atteindre la socket.

Claude Code lie une socket de boîte de réception pour chaque session avec la messagerie entre sessions activée, où d'autres sessions sur la machine livrent des messages. La socket est une socket de domaine Unix sur macOS et Linux, y compris Linux à l'intérieur de WSL 2, et un tuyau nommé sur Windows natif. Pour quels types de session en lient un, consultez [Sessions non-interactives](#non-interactive-sessions).

Vous pouvez trouver le chemin de la socket à deux endroits :

* `/status` l'affiche dans la ligne `Peer address`. Le chemin est préfixé avec `uds:`.
* Claude Code l'exporte vers les [hooks](/docs/fr/hooks) et les commandes Bash comme la variable d'environnement [`CLAUDE_CODE_MESSAGING_SOCKET`](/docs/fr/env-vars#variables) :
  * Dans une session qui démarre avec la messagerie activée, Claude Code exporte la variable avant que tout hook ne s'exécute, y compris `SessionStart`.
  * Chaque session exporte sa propre socket, jamais une héritée d'une session parent.

Sur macOS et Linux, Claude Code restreint la socket à votre utilisateur du système d'exploitation. Sur Windows natif, elle nécessite plutôt que chaque connexion s'authentifie d'abord avec une clé que seul votre utilisateur du système d'exploitation peut lire. De toute façon, sur une machine partagée les sessions d'un autre utilisateur ne peuvent pas la livrer.

Sur macOS et Linux, Claude Code refuse aussi de créer la socket dans un répertoire qu'il ne peut pas accepter, par exemple un qu'un autre utilisateur possède, et utilise un répertoire privé par utilisateur, `/tmp/cc-socks-<uid>`, à la place. Quand il ne peut accepter aucun répertoire, la session s'exécute sans boîte de réception : Claude Code affiche un avis, `/status` affiche `unavailable` et la raison dans sa ligne `Peer address`, et le journal [`--debug`](/docs/fr/cli-reference#cli-flags) enregistre le refus complet.

Aux côtés du chemin de la socket, Claude Code exporte un jeton par session comme [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/fr/env-vars#variables). Un script postant à la socket de sa propre session peut envoyer `{"type":"auth","token":"<token>"}` comme la première ligne de sa connexion, où `<token>` est la valeur de `CLAUDE_CODE_MESSAGING_TOKEN`. Si Claude Code nécessite la ligne dépend de la plateforme :

* **macOS et Linux, y compris WSL 2** : la ligne est optionnelle. Claude Code accepte une connexion avec ou sans elle.
* **Windows natif** : la ligne est requise. Claude Code ferme toute connexion dont la première ligne n'est pas une ligne d'authentification valide et ne livre rien de cette connexion.

Ouvrez la connexion seulement quand le message que vous postez est prêt. Claude Code ferme une connexion qui n'a pas envoyé une ligne complète dans les 30 secondes, donc capturez d'abord la sortie d'une commande lente puis ouvrez la connexion pour l'envoyer.

Les [règles propres-enfant](#own-child-messages) ci-dessous disent quand Claude Code consulte le jeton et comment il traite un message qu'il ne peut pas vérifier.

<span id="own-child-messages" />Claude Code exécute les messages arrivant sur la socket par les mêmes [contrôles entrants](#control-inbound-messages) que tout autre message pair, avec une exception et une condition préalable :

* **Messages propres-enfant** : quand aucune valeur `crossSessionInbound` ne s'applique, Claude Code livre un message qu'il vérifie provenir des processus enfants de la session, comme un hook ou une commande Bash postant vers la socket de sa propre session.
  * Sur Linux, y compris à l'intérieur de WSL 2, Claude Code peut vérifier par preuve de processus même pour un enfant qui a déjà quitté. Sur macOS il ne peut vérifier que de cette manière que pendant que le processus de postage s'exécute toujours, et dans un conteneur où Claude Code s'exécute comme ID de processus 1 il n'a aucune preuve de processus du tout. Sur Windows natif il n'en a pas non plus.
  * Sur macOS après que le processus de postage ait quitté et dans les conteneurs où Claude Code s'exécute comme ID de processus 1, cette preuve de processus manque, et Claude Code vérifie plutôt un enfant qui a envoyé le [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/fr/env-vars#variables) exporté de la session dans la ligne d'authentification qui a ouvert sa connexion. Sur Windows natif, ce jeton est la seule façon que Claude Code vérifie un message propre-enfant.
  * Quand Claude Code ne peut vérifier de l'une ou l'autre manière, il traite le message comme tout autre qui n'affirme aucune classe de permission, donc une session qui contourne les invites de permission le retient pour votre approbation.
* **Sessions sandboxées** : contrôlez si une commande Bash peut atteindre la socket de l'intérieur du [sandbox](/docs/fr/sandboxing) avec les paramètres de socket Unix du sandbox, [`sandbox.network.allowAllUnixSockets` et `sandbox.network.allowUnixSockets`](/docs/fr/settings-reference#sandbox-settings).

<h2 id="restrict-cross-session-messaging">
  Restreindre la messagerie entre sessions
</h2>

Au-delà des défauts par message, vous pouvez restreindre la messagerie de deux façons. Exigez votre approbation avant que tout message ne quitte la machine, ou désactivez la messagerie pour une session ou une organisation.

<h3 id="require-approval-for-cross-machine-messages">
  Exiger l'approbation pour les messages entre machines
</h3>

Définissez [`isolatePeerMachines`](/docs/fr/settings-reference#isolatepeermachines) à `true` pour exiger votre approbation explicite avant que tout `SendMessage` n'atteigne une session au-delà de cette machine :

```json theme={null}
{
  "isolatePeerMachines": true
}
```

Avec cela défini, Claude Code demande votre approbation avant que le message de Claude à une session au-delà de cette machine ne parte, même en mode `bypassPermissions`, qui saute les invites de permission ordinaires. Un `true` de n'importe quelle portée de paramètres s'applique, donc un fichier de projet enregistré peut activer l'exigence mais pas la désactiver. Claude Code ne demande pas pour les messages entre sessions sur la même machine.

<h3 id="turn-off-cross-session-messaging">
  Désactiver la messagerie entre sessions
</h3>

La réception et l'envoi sont des contrôles séparés, donc désactivez la direction dont vous avez besoin, ou les deux. Utilisez `crossSessionInbound` pour les messages qui arrivent, et les règles de permission pour ce que Claude ici peut envoyer ou lister :

* **Arrêter la réception** : définissez `crossSessionInbound` à `refuse`, et Claude Code supprime les messages pair entrants sans les livrer. À partir des paramètres de projet ou locaux, `refuse` s'applique sur chaque autre source, et à partir de vos paramètres utilisateur il s'applique à moins que les paramètres gérés ou l'indicateur `--settings` définissent une valeur.
* **Arrêter l'envoi et la liste** : ajoutez des [règles de permission de refus spécifiques à l'outil](/docs/fr/permissions#tool-specific-permission-rules) nommant `SendMessage` et `ListAgents`. Les deux prennent le nom d'outil nu sans spécificateur.

Les administrateurs peuvent désactiver les deux côtés pour une organisation dans les [paramètres gérés](/docs/fr/managed-settings), combinant les règles de refus avec le `refuse` :

```json theme={null}
{
  "permissions": {
    "deny": ["SendMessage", "ListAgents"]
  },
  "crossSessionInbound": "refuse"
}
```

Avec cela en place, Claude Code lie toujours la socket de boîte de réception de chaque session, mais supprime chaque message qui arrive sur elle sans livrer quoi que ce soit à Claude. Refuser `SendMessage` supprime aussi la messagerie aux sous-agents et aux coéquipiers d'équipe d'agents, car le même outil sert les deux. Une session refusante n'affiche aucun changement visible, dans son propre `/status` ou dans les listes d'autres sessions sur la même machine, donc pour le confirmer, vérifiez les fichiers de paramètres qui s'appliquent à cette session plutôt que son statut.

<h2 id="availability">
  Disponibilité
</h2>

La messagerie entre sessions nécessite Claude Code v2.1.224 ou ultérieure sur macOS, Linux et WSL 2, et v2.1.234 ou ultérieure sur Windows natif. La disponibilité, et quelles sessions Claude peut envoyer des messages, dépendent aussi de votre système d'exploitation, fournisseur et configuration :

* **Système d'exploitation** : disponible sur macOS, Windows et Linux, y compris Linux à l'intérieur de WSL 2.

* **Sessions sur cette machine** : disponible sur chaque fournisseur, y compris Amazon Bedrock, Claude Platform sur AWS, Agent Platform de Google Cloud et Microsoft Foundry, et dans les sessions qui s'exécutent avec la [récupération de drapeau de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching) désactivée. Sur ces fournisseurs, et avec la récupération de drapeau désactivée, la messagerie sur la même machine nécessite Claude Code v2.1.248 ou ultérieure. Claude Code livre ces messages sur une [socket par session sur votre machine](#the-sessions-inbox-socket), jamais par les serveurs Anthropic.

  Pour arrêter une session de les recevoir, définissez [`crossSessionInbound`](#turn-off-cross-session-messaging) à `refuse`.

* **Sessions au-delà de cette machine** : Claude trouve vos [sessions Claude Code sur le web](/docs/fr/claude-code-on-the-web) et vos sessions sur d'autres machines à partir d'une session connectée à la Télécommande, qui nécessite une connexion claude.ai comme authentification active de cette session et les autres [exigences de Télécommande](/docs/fr/remote-control#requirements). Claude ne peut pas trouver ces sessions avec une clé API ou sur Amazon Bedrock, Claude Platform sur AWS, Agent Platform de Google Cloud et Microsoft Foundry.

Pour vérifier une session, tapez `/list-agents`, aussi disponible comme `/peers`. Le résultat sépare une session qui n'a pas la fonctionnalité d'une session où quelque chose de plus étroit a bloqué un message, comme un outil `SendMessage` manquant ou un envoi refusé :

* **`/list-agents` n'est pas reconnu** : la session n'a pas la messagerie entre sessions. Travaillez à travers les exigences ci-dessus, en commençant par `claude --version` pour l'exigence de version.
* **`/list-agents` fonctionne mais un envoi n'est pas arrivé** : la messagerie est activée, et quelque chose de plus étroit s'applique :
  * **Règles de refus** : une [règle de permission de refus](#turn-off-cross-session-messaging) supprime les outils `SendMessage` et `ListAgents`.
  * **Contrôles entrants** : les [contrôles entrants de la session réceptrice](#control-inbound-messages) peuvent retenir ou supprimer ce que vous lui envoyez.
  * **Session cloud manquante** : une session cloud n'apparaît que pendant que cette session est connectée à la [Télécommande](/docs/fr/remote-control).
  * **Session sur une autre machine manquante** : une session sur une autre de vos machines n'apparaît que quand elle s'exécute avec la [Télécommande](/docs/fr/remote-control) et que cette session est aussi connectée.
  * **Session sur une autre machine `offline`** : un message à une session listée comme `offline` passe, mais [n'arrive que après que la machine de cette session se reconnecte](#message-sessions-on-other-machines).
  * **Session cloud ou sur une autre machine plus ancienne manquante** : Claude Code [lit ces listes de sessions les plus récentes en premier et s'arrête après un nombre limité de pages](#see-which-sessions-claude-can-reach), donc Claude ne peut pas envoyer un message à une session qui a dépassé par nom.
  * **Démarrer une conversation** : [Envoyer des messages aux sessions sur d'autres machines](#message-sessions-on-other-machines) couvre le démarrage d'une conversation avec une session au-delà de cette machine.

Dans une session avec messagerie, `/status` affiche aussi une ligne `Peer address` avec l'adresse de boîte de réception propre de la session, ou `unavailable` et la raison quand Claude Code [ne pouvait pas configurer une boîte de réception](#the-sessions-inbox-socket).

<h2 id="limitations">
  Limitations
</h2>

Les limites ici sont des propriétés du canal de messagerie lui-même et s'appliquent partout où la fonctionnalité s'exécute. Pour les lacunes de plateforme et de fournisseur, consultez [Disponibilité](#availability) à la place.

* **Texte brut seulement** : Claude envoie seulement du texte brut entre les sessions. Les messages de protocole structuré d'[équipe d'agents](/docs/fr/agent-teams) restent au sein d'une équipe.
* **La taille du message sur la même machine est plafonnée** : Claude Code refuse un message à une session sur cette machine une fois que sa forme sérialisée dépasse environ un million de caractères. Le refus [nomme les tailles exactes](/docs/fr/errors#message-too-large-for-cross-session-delivery). Rien n'atteint la session réceptrice.
* **Les rafales rapides vers une session sont refusées à l'expéditeur** : une fois qu'une rafale rapide de messages vers une session sur cette machine atteint ce que la boîte de réception de cette session accepte, Claude Code refuse d'autres envois dans la session d'envoi. Le [refus nomme la rafale](/docs/fr/errors#too-many-messages-to-this-session-just-now) et dit à Claude de regrouper le reste en un message ou d'attendre. Avant v2.1.236, Claude Code rapportait ces envois comme envoyés pendant que la session réceptrice les supprimait.
* **Les boucles de message sont limitées** : dans la session réceptrice, Claude Code limite le débit des messages répétés par expéditeur, supprime les répétitions identiques arrivant dans une courte fenêtre, et met en file d'attente au maximum 50 messages acceptés pour que Claude les lise. Une boucle de message entre deux sessions s'arrête donc d'elle-même. Quand la limite de débit, la vérification de répétition ou le plafond de file d'attente supprime un message d'une session interactive sur cette machine, Claude Code dit à cette session lequel l'a supprimé et dit à son Claude de ne pas renvoyer immédiatement.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Sous-agents](/docs/fr/sub-agents#resume-subagents) et [équipes d'agents](/docs/fr/agent-teams#messages-between-agents) : messagerie au sein d'une seule session ou équipe
* [Agents en arrière-plan](/docs/fr/agent-view) : dispatcher et surveiller les sessions parallèles que vous pourriez envoyer des messages
* [Télécommande](/docs/fr/remote-control) : connectez cette session pour atteindre vos sessions sur d'autres machines
* [Paramètres](/docs/fr/settings-reference#all-settings) : `crossSessionInbound`, `isolatePeerMachines` et `dialogExpiry`
* [Modes de permission](/docs/fr/permission-modes) : les modes derrière les deux classes du défaut entrant
* [Référence des outils](/docs/fr/tools-reference) : les lignes `ListAgents` et `SendMessage` dans le tableau des outils
* [Exécuter les agents en parallèle](/docs/fr/agents) : comparez les façons que Claude Code exécute plusieurs agents
