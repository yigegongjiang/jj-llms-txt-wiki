> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Analysez votre base de code pour détecter les vulnérabilités

> Installez le plugin Claude Security pour analyser votre base de code afin de détecter les vulnérabilités dans une session Claude Code et transformez les résultats en correctifs que vous examinez et appliquez.

Le plugin Claude Security exécute une analyse multi-agents des vulnérabilités de votre base de code dans une session Claude Code. Une équipe d'agents Claude cartographie votre architecture, construit un modèle de menace, recherche les vulnérabilités et examine indépendamment chaque résultat avant de rédiger le rapport. Utilisez le plugin pour analyser un référentiel entier ou [uniquement un ensemble de modifications](#scan-only-your-changes), comme le diff d'une branche, le diff d'une demande de tirage ou un seul commit, puis transformez les résultats que vous choisissez en correctifs que vous examinez et appliquez vous-même.

Le plugin s'exécute localement dans votre session, utilise les modèles auxquels vous avez accès dans Claude Code, et chaque analyse compte par rapport aux limites d'utilisation de votre plan. Si vous souhaitez un service géré qui surveille vos référentiels, ou si vous souhaitez exécuter des analyses sur [Claude Mythos 5](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5), consultez le produit [Claude Security](https://claude.com/product/claude-security), disponible sur le plan Enterprise. Le plugin accède au code que le produit géré ne peut pas atteindre, comme les référentiels hébergés sur GitLab ou Bitbucket, ou sur des réseaux qui n'autorisent pas les connexions entrantes.

Le plugin est également distinct des outils d'examen déjà présents dans Claude Code : le [plugin de conseils en sécurité](/docs/fr/security-guidance) examine le code au fur et à mesure que Claude l'écrit, [`/security-review`](/docs/fr/commands#all-commands) exécute une seule passe sur votre branche, et [Code Review](/docs/fr/code-review) examine les demandes de tirage. Pour comprendre comment les couches s'empilent, consultez [Comment le plugin s'intègre avec les autres outils de sécurité](#how-the-plugin-fits-with-other-security-tools).

<h2 id="prerequisites">
  Conditions préalables
</h2>

Pour exécuter le plugin, vous avez besoin de :

* Un plan payant, pour les [flux de travail dynamiques](/docs/fr/workflows) que l'analyse utilise pour orchestrer ses agents. Sur Pro, activez-les à partir de la ligne Dynamic workflows dans `/config`.
* Python 3.9 ou version ultérieure disponible sur votre `PATH` en tant que `python3`. Vérifiez avec `python3 --version`. L'outillage du plugin utilise uniquement la bibliothèque standard Python, donc rien n'est installé.
* Linux, macOS ou Windows.
* Git, pour les analyses de modifications et pour transformer les résultats en correctifs ; ces tâches ne supportent pas les autres systèmes de contrôle de version. Une analyse complète fonctionne dans n'importe quel répertoire, avec ou sans contrôle de version.

<h2 id="install-the-plugin">
  Installez le plugin
</h2>

Dans une session Claude Code, installez à partir de la [place de marché officielle Anthropic](/docs/fr/plugins/anthropic-marketplaces) :

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

La commande ouvre les détails du plugin, où vous choisissez une [portée d'installation](/docs/fr/plugins/install#install-a-plugin) pour démarrer l'installation.

Si l'installation échoue, la correction dépend du message que Claude Code signale :

* S'il signale `Marketplace "claude-plugins-official" not found`, ajoutez la place de marché avec `/plugin marketplace add anthropics/claude-plugins-official`, puis réessayez l'installation.
* S'il signale qu'il [ne peut pas trouver le plugin sur la place de marché](/docs/fr/plugins/install#install-a-plugin), vérifiez le nom du plugin pour une faute de frappe.

Vérifiez le résumé d'installation. S'il signale `Run /reload-plugins to activate.`, consultez [Appliquer les modifications du plugin sans redémarrer](/docs/fr/plugins/cli-reference#reload-plugins) pour activer le plugin dans votre session actuelle.

Une fois le plugin actif, vous êtes prêt à [analyser et corriger votre base de code](#scan-and-fix-your-codebase).

<h3 id="uninstall-the-plugin">
  Désinstallez le plugin
</h3>

Pour supprimer le plugin, désinstallez-le à partir du menu `/plugin`, ou exécutez `claude plugin uninstall claude-security` dans votre terminal.

<h2 id="scan-and-fix-your-codebase">
  Analysez et corrigez votre base de code
</h2>

Le plugin ajoute une commande, `/claude-security`, qui ouvre un menu de ses trois tâches : analyser la base de code, analyser un ensemble de modifications et suggérer des correctifs. Le chemin heureux exécute une analyse complète, puis transforme ses résultats en correctifs :

<Steps>
  <Step title="Ouvrez le menu Claude Security">
    Exécutez `/claude-security` et choisissez **Scan codebase**.
  </Step>

  <Step title="Choisissez ce que vous souhaitez analyser">
    Le plugin lit d'abord votre référentiel, puis propose le référentiel entier ou une zone ciblée, avec le nombre de fichiers et le coût relatif de chaque option indiqués. Choisissez le référentiel entier, ou répondez « I don't know » et le plugin choisit une valeur par défaut sensée pour la taille de votre référentiel.
  </Step>

  <Step title="Confirmez l'exécution">
    Une analyse peut prendre un certain temps, peut utiliser un nombre important de jetons et nécessite que Claude Code reste ouvert jusqu'à son achèvement. Rien ne s'exécute jusqu'à ce que vous confirmiez.
  </Step>

  <Step title="Lisez le rapport">
    Pendant que l'analyse s'exécute, elle signale chaque étape au fur et à mesure qu'elle commence, avec les détails disponibles sous [`/workflows`](/docs/fr/workflows). Les résultats se trouvent dans un répertoire horodaté dans votre référentiel, décrit dans [Lisez les résultats de l'analyse](#read-the-scan-results).
  </Step>

  <Step title="Transformez les résultats en correctifs">
    Exécutez `/claude-security` à nouveau et choisissez **Suggest patches**, puis sélectionnez les résultats à traiter. Les correctifs examinés se trouvent dans le dossier `patches/` du rapport ; [Corrigez les résultats](#fix-findings) explique comment chaque correctif est construit et examiné.
  </Step>

  <Step title="Appliquez les correctifs que vous acceptez">
    Appliquez chaque correctif à partir de votre shell avec `git apply`, dans sa propre demande de fusion. Les correctifs ne sont jamais appliqués automatiquement.
  </Step>
</Steps>

Vous n'avez pas besoin de commencer par le menu : demandez une tâche directement, comme arguments de la commande, tels que `/claude-security scan my branch`, ou en langage naturel, comme « scan commit abc1234 ». Le plugin fonctionne mieux en [mode auto](/docs/fr/permission-modes), qui permet aux agents de l'analyse de procéder sans invite de permission à chaque étape.

<h3 id="scan-only-your-changes">
  Analysez uniquement vos modifications
</h3>

Lorsque votre branche a des commits que sa base n'a pas, le menu `/claude-security` propose d'analyser uniquement ce diff, afin que vous puissiez vérifier une branche avant de la fusionner. Vous pouvez également analyser l'une de vos demandes de fusion ouvertes, ou un seul commit en le demandant, comme « scan commit abc1234 ». Seules les modifications validées sont analysées : validez ou remisez d'abord les modifications en cours, ou exécutez une analyse complète, qui lit l'arborescence de travail.

Les analyses de modifications nécessitent un référentiel git ; les analyses complètes d'un répertoire sans version fonctionnent toujours. Trouver vos demandes de fusion ouvertes est la seule étape qui atteint le réseau, et elle n'est proposée que lorsque votre session a déjà la permission d'exécuter le CLI GitHub et que `gh` est connecté.

<h3 id="scope-large-repositories">
  Délimitez les grands référentiels
</h3>

Sur un grand référentiel, analysez une zone à la fois au lieu de l'arborescence entière. Choisissez l'une des portées ciblées que le plugin propose, comme votre couche API ou votre code d'authentification, et l'exécution se dimensionne en fonction de ce que vous choisissez. La section de couverture du rapport indique ce qui a été et n'a pas été examiné. Exécutez une autre analyse sur une zone différente à tout moment.

<h3 id="read-the-scan-results">
  Lisez les résultats de l'analyse
</h3>

Chaque analyse écrit ses résultats dans un répertoire horodaté `CLAUDE-SECURITY-<timestamp>/` dans votre référentiel :

* **`CLAUDE-SECURITY-RESULTS.md`** : le rapport, avec l'ID de chaque résultat, comme `F1`, plus son impact, son scénario d'exploitation, sa gravité, sa confiance et sa recommandation
* **`CLAUDE-SECURITY-RESULTS.jsonl`** : les mêmes résultats sous forme lisible par machine, un objet JSON par ligne
* **`CLAUDE-SECURITY-RESULTS.sarif`** : les mêmes résultats sous forme de journal [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) pour l'analyse du code GitHub et tout autre outil qui lit la norme. L'analyse classe les résultats selon leurs catégories de faiblesse [CWE](https://cwe.mitre.org/)
* **`CLAUDE-SECURITY-REVISION-<commit>.json`** : le tampon de révision, enregistrant quel commit a été analysé, avec quel effort, si les modifications non validées faisaient partie de l'arborescence analysée, et à quel point l'exécution a été vérifiée, de sorte qu'un rapport soit toujours lié au code qu'il décrit. Une analyse en dehors du contrôle de version tamponne `UNVERSIONED` à la place du commit

Ce répertoire est la seule modification qu'une analyse apporte à votre extraction, et il porte son propre `.gitignore`, de sorte qu'un `git add` égaré ne balaye jamais un rapport dans un commit. Pour conserver un rapport dans l'historique pour une piste d'audit, supprimez ce seul fichier `.gitignore` et validez le répertoire comme n'importe quel autre.

Les résultats n'apparaissent dans le rapport qu'après que les agents vérificateurs indépendants les analysent, ce qui garde les rapports courts et dignes de lecture. Les analyses sont non déterministes : deux analyses du même code peuvent révéler des résultats différents. Exécutez les analyses régulièrement et utilisez les tampons de révision pour attribuer chaque rapport au code exact et aux paramètres qu'il couvrait.

<h2 id="fix-findings">
  Corrigez les résultats
</h2>

Commencez le flux de correction en choisissant **Suggest patches** dans le menu `/claude-security`, ou demandez en langage naturel, comme « fix finding F3 », puis sélectionnez les résultats du rapport à traiter. Les correctifs sont construits par rapport au code validé, et le rapport doit toujours décrire le code que vous avez : les résultats dont le code a changé depuis sont ignorés avec une note, et le plugin propose une analyse fraîche au lieu de corriger à partir d'un rapport obsolète. Chaque correctif est rédigé dans une copie de travail de votre référentiel, de sorte que vos fichiers source restent intacts jusqu'à ce que vous appliquiez vous-même un correctif.

Avant la livraison, chaque correctif est examiné par un agent indépendant de celui qui l'a écrit, qui exécute les tests de votre projet par rapport à la modification lorsque le code les a et lit le diff de ses propres termes pour tout ce qu'il pourrait introduire de nouveau. Un correctif n'est écrit que lorsque cet examen peut attester que la modification traite le seul résultat, n'introduit aucune nouvelle vulnérabilité et laisse le comportement autrement inchangé. Lorsqu'il ne peut pas attester de ces trois points, vous obtenez une courte note expliquant pourquoi au lieu d'un correctif.

<h3 id="patches-are-never-applied-automatically">
  Les correctifs ne sont jamais appliqués automatiquement
</h3>

L'application d'un correctif est toujours votre décision. Les correctifs se trouvent dans le dossier `patches/` du rapport, un `F<n>.patch` par résultat avec une note à côté expliquant la modification. Appliquez-en un à partir de votre shell, ou demandez à Claude de l'appliquer et d'ouvrir une demande de fusion :

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

Lorsque le code corrigé n'a pas de tests, la note du correctif le dit, de sorte que vous sachiez que son examen s'est déroulé sans passage de test. Appliquez chaque correctif dans sa propre demande de fusion afin qu'il puisse être examiné et testé de manière indépendante.

<h2 id="how-the-plugin-fits-with-other-security-tools">
  Comment le plugin s'intègre avec les autres outils de sécurité
</h2>

Le plugin Claude Security est la couche d'analyse approfondie à la demande dans une pile de défense en profondeur, aux côtés du [plugin de conseils en sécurité](/docs/fr/security-guidance), [`/security-review`](/docs/fr/commands#all-commands), [Code Review](/docs/fr/code-review), du produit géré [Claude Security](https://claude.com/product/claude-security) et de vos scanners existants :

| Étape                             | Outil                                                                          | Ce qu'il couvre                                                                                             |
| :-------------------------------- | :----------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| En session                        | [Plugin de conseils en sécurité](/docs/fr/security-guidance)                        | Vulnérabilités courantes dans le code que Claude écrit, corrigées dans la même session                      |
| À la demande, passe unique        | [`/security-review`](/docs/fr/commands#all-commands)                                | Passe de sécurité unique sur la branche actuelle                                                            |
| À la demande, analyse approfondie | Plugin Claude Security                                                         | Analyse multi-agents d'un référentiel ou d'un diff, avec résultats examinés indépendamment et correctifs    |
| Sur demande de fusion             | [Code Review](/docs/fr/code-review), plans Team et Enterprise                       | Examen multi-agents de la correction et de la sécurité avec contexte de base de code complet                |
| Géré                              | [Claude Security](https://claude.com/product/claude-security), plan Enterprise | Analyse hébergée qui surveille les référentiels connectés                                                   |
| En CI                             | Vos scanners d'analyse statique et de dépendances existants                    | Règles spécifiques au langage, vérifications de la chaîne d'approvisionnement et application des politiques |

Le plugin ne remplace pas vos outils de sécurité du code source existants. Exécutez-le aux côtés de l'analyse statique, de l'analyse des dépendances et de l'examen du code : il raisonne sur votre code de la manière qu'un chercheur en sécurité humain le ferait, ce qui complète les vérifications déterministes que ces outils fournissent.

<h2 id="troubleshooting">
  Dépannage
</h2>

**Le menu `/claude-security` s'ouvre avec un avertissement Python.** Le plugin a besoin de `python3` 3.9 ou version ultérieure sur votre `PATH`. Lorsqu'il ne peut pas trouver `python3` du tout, le menu avertit que Claude Security ne fonctionnera pas jusqu'à ce qu'un soit installé ; lorsque le premier `python3` sur votre `PATH` est plus ancien, l'avertissement nomme la version qu'il a trouvée. Installez Python 3, ou mettez un `python3` plus récent en premier sur votre `PATH`, puis démarrez une nouvelle session.

**Vous pouvez voir un avis « safeguards flagged this message » lors de l'analyse sur un modèle Fable.** Le message nomme le modèle, par exemple « Fable 5.1's safeguards flagged this message ». Les classificateurs de sécurité de cybersécurité de Fable signalent certaines demandes, et Claude Code réexécute une demande signalée sur un modèle Opus via [automatic model fallback](/docs/fr/model-config#automatic-model-fallback). C'est attendu, et l'analyse devrait toujours se terminer avec succès.

<h2 id="related-resources">
  Ressources connexes
</h2>

Pour approfondir les éléments que cette page aborde :

* [Plugin de conseils en sécurité](/docs/fr/security-guidance) : détectez les problèmes dans le code au fur et à mesure que Claude l'écrit, dans la même session
* [Code Review](/docs/fr/code-review) : configurez l'examen multi-agents au moment de la demande de fusion
* [Claude Security](https://claude.com/product/claude-security) : le service géré qui surveille les référentiels connectés
* [Sécurité Claude Code](/docs/fr/security) : comment Claude Code aborde la confiance, les permissions et les protections
* [Installer et gérer les plugins](/docs/fr/plugins/install) : trouvez et installez d'autres plugins à partir de la marketplace officielle
