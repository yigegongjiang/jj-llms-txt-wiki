> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recommander votre plugin depuis votre CLI

> Invitez les utilisateurs de Claude Code à installer votre plugin de la marketplace officielle en émettant une balise claude-code-hint depuis votre CLI ou SDK.

Si vous maintenez une CLI ou un SDK, votre outil peut inviter les utilisateurs de Claude Code à installer votre plugin. Lorsque votre CLI détecte qu'elle s'exécute dans Claude Code, faites-la écrire une balise `<claude-code-hint />` sur une seule ligne vers stderr. Claude Code supprime la ligne de la sortie des outils Bash et PowerShell avant que le modèle ne voie la sortie, puis affiche à l'utilisateur une invite d'installation unique.

Cette page s'applique uniquement si votre plugin est répertorié dans `claude-plugins-official` ou une autre marketplace avec l'un des [noms de marketplace officiels d'Anthropic](/docs/fr/plugins/security#official-marketplace-names). La marketplace communautaire, `claude-community`, n'en fait pas partie.

<Note>
  Pour publier un plugin, consultez [Publier et distribuer un plugin](/docs/fr/plugins/publish).
</Note>

<h2 id="emit-the-hint">
  Émettre l'indice
</h2>

Émettez la balise uniquement lorsque `CLAUDECODE` ou `CLAUDE_CODE_CHILD_SESSION` est défini, afin qu'elle n'apparaisse pas lorsqu'une personne exécute votre CLI directement.

Claude Code définit `CLAUDECODE=1` dans les commandes qu'il exécute via les outils Bash et PowerShell et dans les commandes hook. À partir de la v2.1.172, il définit également `CLAUDE_CODE_CHILD_SESSION=1` là-bas. Les variables diffèrent dans les processus qui les portent :

* **`CLAUDECODE`** : défini par chaque version de Claude Code. Les extensions IDE le définissent également dans leurs terminaux intégrés, donc une vérification sur `CLAUDECODE` seul émet également la balise lorsqu'une personne exécute votre CLI elle-même dans l'un de ces terminaux
* **`CLAUDE_CODE_CHILD_SESSION`** : défini uniquement dans les sous-processus que Claude Code lui-même démarre. Utilisez-le lorsque vous pouvez exiger la v2.1.172 ou une version ultérieure

La [référence des variables d'environnement](/docs/fr/env-vars) contient les détails.

Les exemples suivants vérifient `CLAUDECODE` pour la portée la plus large et émettent un indice pour un plugin nommé `example-cli` dans la marketplace officielle :

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
  if [ -n "$CLAUDECODE" ]; then
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  fi
  ```
</CodeGroup>

Remplacez `example-cli` par le nom de votre plugin dans la marketplace officielle.

Vous pouvez émettre l'indice à chaque invocation, car Claude Code demande une fois pour chaque plugin.

Pour vérifier l'émetteur, exécutez `CLAUDECODE=1 example-cli` dans un terminal et confirmez que la ligne de balise apparaît sur stderr, puis exécutez `example-cli` sans la variable et confirmez que rien d'extra ne s'affiche.

<h2 id="hint-format">
  Format de l'indice
</h2>

La balise doit occuper sa propre ligne ; Claude Code ignore une balise intégrée au milieu d'une ligne.

La balise prend trois attributs, tous obligatoires :

| Attribut | Description                                                   |
| :------- | :------------------------------------------------------------ |
| `v`      | Version du protocole. `1` est la seule valeur prise en charge |
| `type`   | Type d'indice. `plugin` est la seule valeur prise en charge   |
| `value`  | Identifiant du plugin sous la forme `name@marketplace`        |

Les valeurs peuvent être entre guillemets doubles ou sans guillemets ; une valeur sans guillemets ne peut pas contenir d'espaces.

Claude Code supprime la ligne de la sortie même lorsque `v` ou `type` n'est pas reconnu.

<h2 id="check-when-the-prompt-appears">
  Vérifier quand l'invite apparaît
</h2>

L'invite n'apparaît que dans les sessions de terminal interactives. Dans les exécutions `claude -p`, dans les exécutions de sous-agent et dans la sortie des commandes hook, la balise est supprimée et aucune invite n'est affichée. Toutes ces vérifications doivent également réussir :

* **Officiel et installable** : `value` nomme un plugin que Claude Code trouve dans sa copie locale d'une marketplace officielle, qui n'est pas déjà installé, et qu'aucune politique ne bloque
* **Analytics activé** : une session où les analytics de Claude Code sont désactivés ne demande jamais, par exemple une avec `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` défini, ou une sur un fournisseur tiers tel qu'Amazon Bedrock, où la [désactivation automatique de la télémétrie](/docs/fr/data-usage#default-behaviors-by-api-provider) s'applique
* **Limites de fréquence** : une invite par session, une invite au total par plugin indépendamment de la réponse de l'utilisateur, et aucune une fois que 100 plugins ont été demandés sur cette machine
* **Non désactivé** : l'utilisateur n'a pas choisi **Non, et ne plus afficher les conseils d'installation de plugins**
* **Session locale et assistée** : l'espace de travail de la session est local plutôt que sur une machine cloud ou distante, et la session n'est pas exécutée sans surveillance. Par exemple, une session démarrée avec `--cloud`, une servant Remote Control, ou un coéquipier d'une équipe d'agents ne demande jamais

<h2 id="preview-what-the-user-sees">
  Aperçu de ce que l'utilisateur voit
</h2>

Lorsque les vérifications dans [Vérifier quand l'invite apparaît](#check-when-the-prompt-appears) réussissent, Claude Code affiche une boîte de dialogue **Recommandation de plugin** comme suit :

```text theme={null}
─────────────────────────────────────────────────────────────
  Recommandation de plugin

    La commande example-cli suggère d'installer un plugin.

    Plugin : example-cli
    Marketplace : claude-plugins-official
    Description : Intégration officielle pour les déploiements example-cli

    Voulez-vous l'installer ?
    ❯ 1. Oui, installer
      2. Non
      3. Non, et ne plus afficher les conseils d'installation de plugins

─────────────────────────────────────────────────────────────
```

La boîte de dialogue nomme le premier mot de la commande shell que Claude a exécutée, afin que les utilisateurs puissent détecter une discordance. Chaque réponse a un effet :

* **Oui, installer** : installe le plugin au [niveau utilisateur](/docs/fr/plugins/install)
* **Non, et ne plus afficher les conseils d'installation de plugins** : désactive les futures invites d'indice pour cet utilisateur
* **Pas de réponse pendant 30 secondes** : compte comme **Non**

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Publier et distribuer un plugin](/docs/fr/plugins/publish) : les routes dans chaque marketplace, y compris la marketplace officielle, que l'indice nécessite
* [Référence des commandes de plugin](/docs/fr/plugins/cli-reference#plugin-install) : la commande shell qui installe le même plugin en dehors d'une session
