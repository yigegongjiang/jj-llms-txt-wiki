> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Détecter les problèmes de sécurité au fur et à mesure que Claude écrit du code

> Installez le plugin security-guidance pour que Claude examine ses propres modifications de code à la recherche de vulnérabilités et les corrige dans la même session.

Le plugin de conseils en sécurité fait en sorte que Claude examine ses propres modifications de code à la recherche de vulnérabilités courantes pendant qu'il travaille et corrige ce qu'il trouve dans la même session. Le plugin détecte des problèmes tels que l'injection, la désérialisation non sécurisée et les API DOM non sécurisées avant que le code n'atteigne une demande de tirage, réduisant ainsi la charge de révision de sécurité qui incombe aux examinateurs humains en aval.

Une fois installé, le plugin s'exécute automatiquement. Il n'y a rien à invoquer et aucune commande séparée à retenir.

Le plugin est le compagnon en session de [Code Review](/docs/fr/code-review), qui s'exécute sur les demandes de tirage. Ce plugin réduit ce qui atteint la PR. Code Review attrape ce qui le fait. Pour savoir comment le plugin s'articule avec l'examen à la demande et l'analyse CI, consultez [Comment cela s'intègre avec d'autres outils de sécurité](#how-this-fits-with-other-security-tools).

<h2 id="prerequisites">
  Conditions préalables
</h2>

* Claude Code CLI version 2.1.144 ou ultérieure
* Python 3.8 ou ultérieur sur votre `PATH`. Le plugin essaie `python3`, `python` et `py -3` dans cet ordre
* Un référentiel git pour le répertoire dans lequel vous travaillez. Les examens de fin de tour et de commit effectuent un diff par rapport à l'état git et s'ignorent silencieusement en dehors d'un référentiel. La vérification de motif par modification fonctionne n'importe où

À la première exécution, le plugin crée un environnement virtuel sous `~/.claude/security/` et installe le Claude Agent SDK dedans, ce qui nécessite `pip` et un accès réseau. Si cette installation échoue, l'examen de commit revient à un examen unique au lieu d'un examen agentique. Sur Windows, l'étape d'environnement virtuel est ignorée, de sorte que l'examen de commit agentique ne s'exécute que si `claude-agent-sdk` est déjà importable et revient sinon de la même manière.

<h2 id="install-the-plugin">
  Installer le plugin
</h2>

Dans une session Claude Code, installez à partir du [marketplace officiel Anthropic](/docs/fr/discover-plugins#official-anthropic-marketplace) :

```text theme={null}
/plugin install security-guidance@claude-plugins-official
```

L'installation demande une portée. Choisissez la portée utilisateur pour écrire le plugin dans vos paramètres utilisateur, de sorte qu'il se charge dans chaque nouvelle session locale que vous démarrez sur cette machine. Si Claude Code signale que le marketplace n'est pas trouvé, exécutez d'abord `/plugin marketplace add anthropics/claude-plugins-official`, puis réessayez l'installation.

Ensuite, activez-le dans la session actuelle avec `/reload-plugins`, qui applique les modifications de plugin en attente sans redémarrage :

```text theme={null}
/reload-plugins
```

<h3 id="enable-in-cloud-sessions-and-shared-repositories">
  Activer dans les sessions cloud et les référentiels partagés
</h3>

Les plugins à portée utilisateur ne sont pas transférés vers [Claude Code sur le web](/docs/fr/claude-code-on-the-web), car ces sessions s'exécutent sur l'infrastructure Anthropic plutôt que sur votre machine. Pour activer le plugin là-bas, ou pour l'activer pour tous ceux qui clonent un référentiel, déclarez-le dans les paramètres vérifiés du projet :

```json .claude/settings.json theme={null}
{
  "enabledPlugins": {
    "security-guidance@claude-plugins-official": true
  }
}
```

Les administrateurs peuvent activer le plugin à l'échelle de l'organisation en définissant [`enabledPlugins`](/docs/fr/settings#plugin-settings) dans [les paramètres gérés](/docs/fr/admin-setup).

<h2 id="what-the-plugin-checks">
  Ce que le plugin vérifie
</h2>

Le plugin examine le travail de Claude à trois points, chacun à une profondeur différente :

* [À chaque modification de fichier](#on-each-file-edit) : une correspondance de motif rapide pour les appels risqués, sans appel de modèle
* [À la fin de chaque tour](#at-the-end-of-each-turn) : un examen du modèle en arrière-plan de tout ce que ce tour a modifié
* [À chaque commit ou push que Claude effectue](#on-each-commit-or-push-claude-makes) : un examen agentique plus approfondi qui lit le code environnant

Vous pouvez étendre chaque couche en [ajoutant vos propres règles](#add-your-own-rules). Les vérifications intégrées ne peuvent pas être supprimées individuellement, mais vous pouvez [désactiver chaque couche](#disable-or-uninstall) indépendamment.

<h3 id="on-each-file-edit">
  À chaque modification de fichier
</h3>

Lorsque Claude écrit dans un fichier, le plugin analyse le nouveau contenu à la recherche de motifs risqués connus. Il s'agit d'une correspondance de motif sans appel de modèle, elle n'ajoute donc aucun coût d'utilisation.

Exemples de catégories de motifs :

* Exécution de code dynamique : `eval(`, `new Function`, `os.system`, `child_process.exec`
* Désérialisation non sécurisée : `pickle`
* Injection DOM : `dangerouslySetInnerHTML`, `.innerHTML =`, `document.write`
* Fichiers de flux de travail : modifications sous `.github/workflows/`, qui peuvent accorder des autorisations au niveau du référentiel

La vérification s'exécute après que la modification soit appliquée et ajoute l'avertissement au contexte de Claude pour l'étape suivante. Chaque avertissement se déclenche une fois par motif par fichier par session, de sorte que les correspondances répétées dans le même fichier ne saturent pas la conversation.

Vous pouvez [ajouter vos propres motifs](#add-custom-per-edit-patterns) à cette couche avec un fichier `security-patterns.yaml`.

<h3 id="at-the-end-of-each-turn">
  À la fin de chaque tour
</h3>

Un tour est un cycle de réponse de Claude : vous envoyez un message, Claude travaille et répond, et le tour se termine. Après chaque tour, le plugin calcule un diff git de tout ce qui a changé dans l'arborescence de travail pendant le tour, y compris les modifications des outils d'édition de Claude, des commandes Bash et des sous-agents, et l'envoie à un examen Claude séparé axé sur la sécurité. L'examen s'exécute en arrière-plan, de sorte que la réponse de Claude n'est pas retardée. Si l'examen trouve des problèmes, Claude est relancé avec les résultats et les résout en tant que suivi.

Cela détecte les problèmes qu'une correspondance de chaîne ne peut pas, tels que :

* Contournement d'autorisation
* Références d'objet direct non sécurisées
* Injection
* Falsification de requête côté serveur
* Cryptographie faible

Vous voyez à la fois la conclusion et la résolution de Claude directement dans votre session. L'examen couvre jusqu'à 30 fichiers modifiés par tour et se déclenche au maximum trois fois de suite avant de vous rendre la main.

<h3 id="on-each-commit-or-push-claude-makes">
  À chaque commit ou push que Claude effectue
</h3>

Lorsque Claude exécute `git commit` ou `git push` via son outil Bash, le plugin exécute un examen agentique plus approfondi de la modification en arrière-plan. Cet examen lit le code environnant, y compris les appelants, les désinfectants et les fichiers connexes, pour décider si une conclusion est réelle avant de la signaler. Le contexte supplémentaire maintient les faux positifs bas sur les motifs qui semblent dangereux isolément mais qui sont sûrs dans votre base de code.

Cette couche ne se déclenche que sur les commits et les pushes que Claude effectue via son outil Bash. Les commits que vous exécutez à partir de votre propre shell, y compris l'échappement shell `!` à l'intérieur d'une session, ne sont pas examinés. Les examens de commit et de push sont limités à 20 par heure glissante. Si les conclusions de l'examen de commit dupliquent ce que l'examen de fin de tour a déjà signalé, Claude n'est pas relancé, de sorte qu'un commit propre ne produit aucune sortie visible de cette couche.

<h3 id="review-independence-and-limits">
  Indépendance et limites de l'examen
</h3>

Le plugin ne demande pas à la même instance Claude qui a écrit le code de se noter elle-même. La vérification par modification est une correspondance de chaîne déterministe sans modèle impliqué. Les examens de fin de tour et de commit s'exécutent en tant qu'appel Claude séparé avec un contexte frais et une invite axée sur la sécurité : l'examinateur commence par le diff, n'a aucun intérêt dans l'approche originale et est invité uniquement à trouver des problèmes.

Aucune des couches ne bloque les écritures ou les commits. Les conclusions parviennent au Claude qui écrit sous forme d'instructions, Claude les résout dans la conversation, et le modèle d'examen peut manquer des problèmes. Traitez le plugin comme une couche de défense en profondeur, pas une solution de sécurité complète. Consultez [Comment cela s'intègre avec d'autres outils de sécurité](#how-this-fits-with-other-security-tools).

<h2 id="add-your-own-rules">
  Ajouter vos propres règles
</h2>

Le plugin a deux points d'extension : un fichier de conseils Markdown pour les examens soutenus par le modèle, et un fichier de motifs YAML ou JSON pour la correspondance de chaîne par modification. Les deux sont additifs. Vous pouvez ajouter des vérifications mais ne pouvez pas désactiver les vérifications intégrées à partir de ces fichiers.

<h3 id="add-guidance-for-the-model-backed-reviews">
  Ajouter des conseils pour les examens soutenus par le modèle
</h3>

Créez `.claude/claude-security-guidance.md` dans votre projet et décrivez votre modèle de menace et votre liste de contrôle d'examen en langage clair. Les examens soutenus par le modèle le chargent comme contexte supplémentaire aux côtés de la liste de contrôle des vulnérabilités intégrée.

L'exemple suivant concerne un service web avec des routes d'administration à accès contrôlé par rôle et une politique de journalisation des données client :

```markdown .claude/claude-security-guidance.md theme={null}
# Conseils en sécurité pour ce référentiel

- Ne pas enregistrer `customer_id` ou `account_number` au niveau INFO ou supérieur.
- Toutes les routes sous `/admin` doivent appeler `require_role("admin")` avant toute lecture de base de données.
- Utilisez `crypto.timingSafeEqual` pour la comparaison de jetons au lieu de `===`.
```

Ces règles sont des conseils pour l'examinateur, pas des garde-fous déterministes. Le plugin signale les violations comme des conclusions pour que Claude les corrige, mais il ne bloque pas les écritures et ne garantit pas que chaque violation est détectée. Les conseils sont additifs uniquement : une règle qui dit d'ignorer une classe de vulnérabilité ne supprime pas ces conclusions. Pour une application stricte, associez le plugin à un [hook qui bloque la modification](/docs/fr/hooks-guide#block-edits-to-protected-files) ou une vérification CI.

<h3 id="add-custom-per-edit-patterns">
  Ajouter des motifs personnalisés par modification
</h3>

Créez `.claude/security-patterns.yaml` pour ajouter des règles regex ou de sous-chaîne à la [vérification de motif par modification](#on-each-file-edit). Celles-ci s'exécutent en tant que correspondances de chaîne déterministes aux côtés des motifs intégrés :

```yaml .claude/security-patterns.yaml theme={null}
patterns:
  - rule_name: internal_api_key
    substrings: ["sk_live_", "AKIA"]
    reminder: "Clé API codée en dur. Chargez les identifiants à partir du gestionnaire de secrets."
  - rule_name: tenant_unfiltered_query
    regex: "\\.objects\\.all\\(\\)"
    paths: ["**/src/tenants/**"]
    reminder: "Le code multi-locataire doit filtrer par org_id."
```

| Champ           | Type   | Description                                                                                                                                                                                       |
| :-------------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `rule_name`     | string | Identifiant affiché dans l'avertissement                                                                                                                                                          |
| `reminder`      | string | Texte d'avertissement ajouté au contexte de Claude, limité à 1 KB                                                                                                                                 |
| `regex`         | string | Expression régulière Python mise en correspondance avec le contenu modifié                                                                                                                        |
| `substrings`    | list   | Sous-chaînes littérales ; fournissez ceci ou `regex`                                                                                                                                              |
| `paths`         | list   | Motifs glob optionnels ; la règle s'applique uniquement aux fichiers correspondants. Les globs correspondent au chemin complet du fichier, donc préfixez les motifs relatifs au projet avec `**/` |
| `exclude_paths` | list   | Motifs glob optionnels à ignorer ; même correspondance que `paths`                                                                                                                                |

Le plugin lit également `.claude/security-patterns.yml` et `.claude/security-patterns.json` avec le même schéma. JSON fonctionne sur n'importe quelle installation Python. Les formes YAML nécessitent que PyYAML soit importable, ce que le plugin n'installe pas pour vous. Le plugin charge jusqu'à 50 règles personnalisées et ignore les expressions régulières qui semblent sujettes au retour en arrière catastrophique.

<h3 id="rule-file-lookup-locations">
  Emplacements de recherche des fichiers de règles
</h3>

Le plugin recherche `claude-security-guidance.md` et `security-patterns.yaml` aux mêmes emplacements, indépendamment de la façon dont le plugin a été activé :

| Portée       | Chemin                                      | Notes                                             |
| :----------- | :------------------------------------------ | :------------------------------------------------ |
| Utilisateur  | `~/.claude/claude-security-guidance.md`     | S'applique à chaque projet sur votre machine      |
| Projet       | `.claude/claude-security-guidance.md`       | Vérifié avec le référentiel                       |
| Projet local | `.claude/claude-security-guidance.local.md` | Ignoré par Git, pour les remplacements personnels |

Le plugin charge tous les emplacements qui existent et les concatène, avec un plafond combiné de 8 KB pour le fichier de conseils. Les administrateurs peuvent distribuer des règles à l'échelle de l'organisation en poussant le fichier à portée utilisateur vers `~/.claude/` via la gestion des appareils. Les mêmes chemins s'appliquent à `security-patterns.yaml`.

<h2 id="usage-cost">
  Coût d'utilisation
</h2>

La [vérification de motif par modification](#on-each-file-edit) n'effectue aucun appel de modèle et n'ajoute aucun coût. Les examens de [fin de tour](#at-the-end-of-each-turn) et de [commit](#on-each-commit-or-push-claude-makes) dépensent chacun une utilisation de modèle supplémentaire qui compte vers votre [utilisation](/docs/fr/costs) comme toute autre demande Claude. L'examen de commit est agentique et peut prendre plusieurs tours de modèle par commit, limité à 20 examens par heure glissante. Attendez-vous à environ un appel d'examen par tour qui modifie les fichiers et un examen plus approfondi par commit, tous deux soumis aux plafonds ci-dessus.

Les deux examens soutenus par le modèle utilisent Claude Opus 4.7 par défaut. Définissez `SECURITY_REVIEW_MODEL` pour choisir un modèle différent pour l'examen de fin de tour et `SG_AGENTIC_MODEL` pour l'examen de commit.

Le plugin est disponible sur tous les plans.

<h2 id="disable-or-uninstall">
  Désactiver ou désinstaller
</h2>

Pour désactiver les couches individuelles tout en gardant le reste, définissez la variable d'environnement correspondante :

| Variable                        | Effet                                                                            |
| :------------------------------ | :------------------------------------------------------------------------------- |
| `ENABLE_PATTERN_RULES=0`        | Désactiver la [vérification de motif par modification](#on-each-file-edit)       |
| `ENABLE_STOP_REVIEW=0`          | Désactiver l'[examen diff de fin de tour](#at-the-end-of-each-turn)              |
| `ENABLE_COMMIT_REVIEW=0`        | Désactiver l'[examen de commit et de push](#on-each-commit-or-push-claude-makes) |
| `ENABLE_CODE_SECURITY_REVIEW=0` | Désactiver tous les examens soutenus par le modèle à la fois                     |
| `SECURITY_GUIDANCE_DISABLE=1`   | Désactiver le plugin entièrement sans le désinstaller                            |

Pour mettre en pause le plugin dans votre portée utilisateur :

```text theme={null}
/plugin disable security-guidance@claude-plugins-official
```

Pour le supprimer de votre portée utilisateur :

```text theme={null}
/plugin uninstall security-guidance@claude-plugins-official
```

Si le plugin a été activé via le `.claude/settings.json` d'un projet, le désactiver à partir de `/plugin` écrit un remplacement dans votre `.claude/settings.local.json` plutôt que de modifier le fichier vérifié, de sorte que le plugin reste désactivé pour vous tandis que vos coéquipiers ne sont pas affectés. Le même dialogue propose également de désinstaller le plugin pour tout le monde en le supprimant du `.claude/settings.json` partagé ; cette option nécessite Claude Code v2.1.203 ou version ultérieure. S'il a été activé via [les paramètres gérés](/docs/fr/admin-setup), seul un administrateur peut le désactiver.

<h2 id="how-the-plugin-integrates-with-claude-code">
  Comment le plugin s'intègre avec Claude Code
</h2>

Le plugin est entièrement construit sur [hooks](/docs/fr/hooks), le mécanisme pour exécuter votre propre code à des points spécifiques dans la boucle de Claude. Il enregistre :

| Événement Hook                                                  | Objectif                                                                                                     |
| :-------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `SessionStart`                                                  | Amorcer l'environnement Python du plugin                                                                     |
| `UserPromptSubmit`                                              | Capturer la ligne de base de l'arborescence de travail sur laquelle l'examen de fin de tour effectue un diff |
| `PostToolUse` sur `Edit`, `Write` et `NotebookEdit`             | Correspondance de motif par modification                                                                     |
| `Stop`                                                          | Examen diff de fin de tour, exécuté en arrière-plan                                                          |
| `PostToolUse` sur `Bash`, filtré sur `git commit` et `git push` | Examen de commit et de push, exécuté en arrière-plan                                                         |

Si vous construisez vos propres hooks, la [source du plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/security-guidance) est un exemple fonctionnel d'exécution d'un appel de modèle séparé à partir d'un hook et de renvoi du résultat à la session.

<h2 id="how-this-fits-with-other-security-tools">
  Comment cela s'intègre avec d'autres outils de sécurité
</h2>

Le plugin est une couche dans une approche de défense en profondeur. Il détecte les problèmes au plus tôt, pendant que le code est encore dans l'éditeur, mais ce n'est pas une garantie et ne remplace pas les vérifications ultérieures. Une pile typique :

| Étape                 | Outil                                                             | Ce qu'il couvre                                                                                                                      |
| :-------------------- | :---------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| En session            | Plugin de conseils en sécurité                                    | Vulnérabilités courantes dans le code que Claude écrit, corrigées dans la même session                                               |
| À la demande          | [`/security-review`](/docs/fr/commands#all-commands)                   | Passage de sécurité unique sur la branche actuelle, exécuté quand vous le demandez                                                   |
| Sur demande de tirage | [Code Review](/docs/fr/code-review), plans Team et Enterprise          | Examen multi-agent de la correction et de la sécurité avec contexte complet de la base de code                                       |
| En CI                 | Vos analyseurs statiques existants et vos scanners de dépendances | Règles spécifiques au langage, vérifications de la chaîne d'approvisionnement et application de politique que le plugin ne tente pas |

Chaque étape ultérieure détecte ce que les étapes antérieures manquent. La valeur du plugin est de réduire le volume qui les atteint, pas d'éliminer le besoin de les utiliser.

<h2 id="troubleshooting">
  Dépannage
</h2>

Le plugin écrit les diagnostics d'exécution dans `~/.claude/security/log.txt`. Vérifiez d'abord là-bas si les examens n'apparaissent pas.

Raisons courantes pour lesquelles une couche d'examen s'ignore sans message dans la conversation :

* Le répertoire n'est pas un référentiel git : les examens de fin de tour et de commit nécessitent l'état git et s'ignorent en dehors d'un référentiel
* La session n'a pas d'authentification Anthropic : les examens soutenus par le modèle s'ignorent et seule la vérification de motif par modification s'exécute
* Un fichier `security-patterns.yaml` est présent mais PyYAML n'est pas importable : le fichier est ignoré. Utilisez `security-patterns.json` à la place

<h2 id="related-resources">
  Ressources connexes
</h2>

Pour approfondir les éléments que cette page aborde :

* [Code Review](/docs/fr/code-review) : configurer l'examen multi-agent au moment de la PR
* [Automatiser les flux de travail avec des hooks](/docs/fr/hooks-guide) : créer vos propres vérifications aux mêmes points du cycle de vie
* [Découvrir et installer des plugins](/docs/fr/discover-plugins#official-anthropic-marketplace) : parcourir d'autres plugins officiels
