> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recommander votre plugin depuis votre CLI

> Émettez un marqueur d'une ligne depuis votre CLI pour que Claude Code invite les utilisateurs à installer votre plugin officiel.

Si vous maintenez une CLI ou un SDK et disposez d'un plugin dans la marketplace officielle d'Anthropic, votre outil peut inviter les utilisateurs de Claude Code à installer ce plugin. Votre CLI écrit un marqueur d'une ligne sur stderr lorsqu'elle détecte qu'elle s'exécute dans Claude Code. Claude Code lit le marqueur, le supprime de la sortie et affiche à l'utilisateur une invite d'installation unique.

Claude Code supprime la ligne d'indice de la sortie de la commande avant de l'envoyer au modèle, de sorte que le marqueur n'apparaît jamais dans la conversation et n'est pas comptabilisé dans l'utilisation des tokens. Le protocole ne nécessite aucune commande supplémentaire et ne change pas ce que votre CLI affiche pour les utilisateurs en dehors de Claude Code.

Cette page est destinée aux mainteneurs de CLI et de SDK. Si vous cherchez à installer des plugins, consultez [Découvrir et installer des plugins](/docs/fr/discover-plugins).

<h2 id="how-it-works">
  Fonctionnement
</h2>

Claude Code définit la variable d'environnement [`CLAUDECODE`](/docs/fr/env-vars) à `1` pour chaque commande qu'elle exécute via les outils Bash et PowerShell, et pour les commandes [hook](/docs/fr/hooks). À partir de la v2.1.172, elle définit également [`CLAUDE_CODE_CHILD_SESSION`](/docs/fr/env-vars) à `1` dans ces mêmes sous-processus. Lorsque votre CLI voit l'une de ces variables, elle écrit une balise auto-fermante `<claude-code-hint />` sur stderr. Dans les commandes hook, la balise d'indice est supprimée et ignorée. Seule la sortie des outils Bash et PowerShell déclenche l'invite d'installation.

Lorsque Claude Code reçoit la sortie de la commande, elle :

1. Analyse les lignes d'indice et les supprime avant que la sortie n'atteigne le modèle
2. Vérifie que l'indice cible un plugin dans une marketplace officielle d'Anthropic
3. Vérifie que le plugin n'est pas déjà installé et n'a pas été proposé auparavant
4. Affiche à l'utilisateur une invite d'installation qui nomme la commande qui a émis l'indice

Claude Code n'installe jamais un plugin automatiquement. L'utilisateur confirme toujours.

<h2 id="emit-the-hint">
  Émettre l'indice
</h2>

Les invites d'indice ne s'activent que pour les plugins listés dans la marketplace officielle d'Anthropic. Consultez [Faire entrer votre plugin dans la marketplace officielle](#get-your-plugin-into-the-official-marketplace) avant de déployer l'intégration.

Conditionnez l'émission sur une variable d'environnement afin que le marqueur n'apparaisse pas lorsqu'un utilisateur humain exécute directement votre CLI, puis écrivez la balise sur stderr sur sa propre ligne. Choisissez la variable à vérifier :

* `CLAUDECODE` : définie sur chaque version de Claude Code, elle atteint donc le plus de sessions. Elle est également définie dans les sessions tmux et les sous-processus du serveur MCP stdio que Claude Code démarre. Les extensions IDE la définissent également dans leurs terminaux intégrés, où un utilisateur humain peut exécuter directement votre CLI.
* `CLAUDE_CODE_CHILD_SESSION` : définie uniquement dans les sous-processus que Claude Code lui-même génère, tels que les appels d'outils, les commandes hook et les commandes de [ligne d'état](/docs/fr/statusline), de sorte que la balise n'atteint normalement pas un terminal humain. Un processus de longue durée qui a été démarré à l'intérieur d'une session, tel qu'un serveur tmux, capture la variable, de sorte que les shells lancés ultérieurement à partir de ce processus affichent toujours la balise brute. Nécessite Claude Code v2.1.172 ou version ultérieure, de sorte que les sessions sur les versions antérieures manquent l'indice.

Les exemples suivants conditionnent sur `CLAUDECODE` pour une portée maximale et émettent un indice pour un plugin nommé `example-cli` dans la marketplace officielle :

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  [ -n "$CLAUDECODE" ] &&
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  ```
</CodeGroup>

Remplacez `example-cli` par le nom de votre plugin dans la marketplace officielle.

<h2 id="choose-where-to-emit">
  Choisir où émettre
</h2>

Vous contrôlez les chemins de code qui émettent l'indice. Claude Code déduplique par plugin, donc émettre à chaque invocation n'a aucun inconvénient. Les points de contact qui fonctionnent bien incluent :

| Placement                                  | Pourquoi cela fonctionne                                               |
| :----------------------------------------- | :--------------------------------------------------------------------- |
| Sortie `--help`                            | Claude exécute souvent l'aide lors de l'exploration d'une CLI inconnue |
| Erreurs de sous-commande inconnue          | Atteint le moment où Claude est confus par votre interface             |
| Succès de connexion ou d'authentification  | L'utilisateur est déjà dans un état d'esprit de configuration          |
| Message de bienvenue de première exécution | Un moment d'intégration naturel                                        |

<h2 id="what-the-user-sees">
  Ce que voit l'utilisateur
</h2>

Lorsque l'indice passe tous les contrôles, Claude Code affiche une invite comme la suivante :

```text theme={null}
─────────────────────────────────────────────────────────────
  Recommandation de plugin

    La commande example-cli suggère d'installer un plugin.

    Plugin : example-cli
    Marketplace : claude-plugins-official
    Intégration officielle pour les déploiements example-cli

    Voulez-vous l'installer ?
    ❯ 1. Oui, installer example-cli
      2. Non
      3. Non, et ne plus afficher les suggestions d'installation de plugins

─────────────────────────────────────────────────────────────
```

L'invite nomme la commande qui a produit l'indice afin que les utilisateurs puissent détecter une discordance entre l'outil et le plugin qu'il recommande. Si l'utilisateur ne répond pas dans les 30 secondes, l'invite se ferme en tant que **Non**.

La fréquence des invites est limitée :

* **Une fois par plugin** : après l'affichage de l'invite, Claude Code enregistre le plugin et ne le propose jamais plus, quel que soit la réponse de l'utilisateur.
* **Une fois par session** : sur tous les CLIs de la machine, au maximum une invite d'indice apparaît par session Claude Code.

Sélectionner **Oui** installe le plugin à la portée utilisateur. Sélectionner **Non, et ne plus afficher les suggestions d'installation de plugins** désactive toutes les futures invites d'indice pour l'utilisateur.

<h2 id="hint-format">
  Format de l'indice
</h2>

L'indice est une balise auto-fermante avec trois attributs requis.

```text theme={null}
<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />
```

| Attribut | Requis | Description                                                   |
| :------- | :----- | :------------------------------------------------------------ |
| `v`      | Oui    | Version du protocole. `1` est la seule valeur prise en charge |
| `type`   | Oui    | Type d'indice. `plugin` est la seule valeur prise en charge   |
| `value`  | Oui    | Identifiant du plugin sous la forme `name@marketplace`        |

Les valeurs d'attribut peuvent être entre guillemets doubles ou sans guillemets. Les valeurs sans guillemets ne peuvent pas contenir d'espaces. Les séquences d'échappement ne sont pas prises en charge.

<h2 id="requirements">
  Exigences
</h2>

Claude Code applique deux conditions avant d'agir sur un indice. Les indices qui échouent à l'un des contrôles sont supprimés :

* **Ligne propre** : la balise doit occuper sa propre ligne. Une balise intégrée au milieu d'une ligne, par exemple à l'intérieur d'une instruction de journal, est ignorée. Les espaces blancs de début et de fin sur la ligne sont autorisés.
* **Marketplace officielle** : la `value` doit référencer un plugin dans une marketplace contrôlée par Anthropic, telle que `claude-plugins-official`. Les indices pointant vers d'autres marketplaces sont silencieusement supprimés.

La ligne d'indice est toujours supprimée de la sortie avant qu'elle n'atteigne le modèle, même lorsque la version ou le type n'est pas reconnu, de sorte que le marqueur n'est jamais comptabilisé dans l'utilisation des tokens.

Les conseils restants sont recommandés mais non appliqués. Claude Code ne peut pas observer si votre CLI les suit :

* **Écrire sur stderr** : stderr garde la balise hors des pipelines shell tels que `example-cli deploy | jq`. Claude Code analyse les deux flux, donc stdout fonctionne aussi.
* **Conditionner sur une variable d'environnement** : n'émettez que lorsque `CLAUDECODE` ou `CLAUDE_CODE_CHILD_SESSION` est défini. Consultez [Émettre l'indice](#emit-the-hint) pour savoir comment les deux variables diffèrent.

<h2 id="get-your-plugin-into-the-official-marketplace">
  Obtenir votre plugin dans la marketplace officielle
</h2>

Le protocole d'indice ne prend effet que pour les plugins répertoriés dans la marketplace officielle d'Anthropic, `claude-plugins-official`. Anthropic gère cette marketplace à sa discrétion, et les formulaires de soumission intégrés à l'application ajoutent des plugins à la [marketplace communautaire](/docs/fr/plugins#submit-your-plugin-to-the-community-marketplace) à la place, que le protocole d'indice ne vérifie pas. Si vous travaillez avec un contact partenaire d'Anthropic, contactez-le pour coordonner une inscription à la marketplace officielle.

<h2 id="see-also">
  Voir aussi
</h2>

* [Créer des plugins](/docs/fr/plugins) : créez le plugin que votre CLI recommande
* [Créer et distribuer une marketplace de plugins](/docs/fr/plugin-marketplaces) : hébergez des plugins en dehors de la marketplace officielle
* [Variables d'environnement](/docs/fr/env-vars) : référence complète pour `CLAUDECODE` et les variables associées
