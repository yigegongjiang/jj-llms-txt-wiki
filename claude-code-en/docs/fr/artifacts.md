> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Partager la sortie de session en tant qu'artefacts

> Les artefacts transforment le travail de Claude Code en pages interactives en direct sur claude.ai que vous pouvez garder privées, partager avec votre organisation ou publier via un lien public.

<Note>
  Les artefacts sont disponibles sur les plans Pro, Max, Team et Enterprise et nécessitent une session connectée avec [`/login`](/docs/fr/setup#authenticate). Consultez [Disponibilité](#availability) pour l'ensemble complet des exigences.
</Note>

Un artefact est une page web interactive en direct que Claude Code publie à partir de votre session vers une URL privée sur claude.ai. Vous l'ouvrez dans un navigateur, et il se met à jour sur place au fur et à mesure que la session continue. Partagez-le à partir de l'en-tête de la page lorsque vous souhaitez que quelqu'un d'autre le voie également. Par exemple, utilisez un artefact pour guider un relecteur à travers une demande de tirage avec des diffs annotés, créer un tableau de bord à partir des données de session, ou maintenir une chronologie d'investigation qui se remplit au fur et à mesure que Claude travaille.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Un artefact ouvert dans un navigateur à claude.ai/code/artifact. L'en-tête du visualiseur affiche le titre de l'artefact acme-funnel-fix, un bouton Partager et l'avatar de l'auteur. Le menu Partager est ouvert avec le bouton bascule Toujours partager la dernière version, un sélecteur de version indiquant Partage de la version 2, un sélecteur d'audience Tout le monde chez Acme, et un bouton Copier le lien. Sous l'en-tête, la page d'artefact affiche deux maquettes mobiles côte à côte, un graphique en entonnoir et une ligne de cartes de métriques." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Quand utiliser un artefact
</h2>

Utilisez un artefact lorsque le texte du terminal n'est pas le bon médium pour ce que Claude a produit : une sortie qui est plus facile à regarder et avec laquelle interagir qu'à lire ligne par ligne. Claude construit la page à partir de tout ce que votre session peut atteindre, y compris votre base de code et les données qu'elle récupère via vos [outils connectés](/docs/fr/mcp), de sorte que la page peut afficher des choses qui prendraient des paragraphes à décrire. Par exemple, demandez à Claude de :

* Guider un relecteur à travers une demande de tirage avec des diffs annotés
* Afficher un tableau de bord à partir des données que la session a déjà récupérées
* Disposer plusieurs options de conception ou d'implémentation côte à côte
* Maintenir une chronologie d'investigation qui se remplit pendant qu'une tâche longue s'exécute
* Envoyer à un coéquipier un lien au lieu de coller la sortie dans Slack
* Publier un tableau de bord qui [récupère les données actualisées via les connecteurs MCP](#pull-live-data-with-mcp-connectors) chaque fois que quelqu'un l'ouvre

Consultez [Ce que vous pouvez créer](#what-you-can-build) pour les invites qui correspondent à chacune de ces options, et [Récupérer les données en direct avec les connecteurs MCP](#pull-live-data-with-mcp-connectors) pour l'invite du tableau de bord soutenu par des connecteurs.

<h3 id="what-an-artifact-is-not">
  Ce qu'un artefact n'est pas
</h3>

Un artefact est une capture de travail, pas une application. C'est une page autonome unique sans backend, elle ne peut donc pas stocker l'entrée de formulaire ou servir plusieurs routes, et son seul chemin vers les données externes lorsque quelqu'un la visualise est [l'appel des connecteurs MCP](#pull-live-data-with-mcp-connectors). Pour un outil interne hébergé avec un backend, déployez-le plutôt sur votre propre infrastructure. Consultez [Contraintes de page](#page-constraints) pour l'ensemble complet des limites.

<h2 id="create-an-artifact">
  Créer un artefact
</h2>

Claude peut publier un artefact de lui-même lorsque la sortie convient à une page, ou vous pouvez en demander un directement. Pour demander, nommez la fonctionnalité ou décrivez la sortie visuelle que vous souhaitez en langage naturel. Un bon candidat est tout ce qui est plus facile à voir qu'à lire en tant que texte, comme un diff annoté, un graphique ou un ensemble d'options à comparer. Les invites ci-dessous sont deux exemples ; consultez [Ce que vous pouvez créer](#what-you-can-build) pour plus de modèles.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

Claude écrit la page dans un fichier HTML ou Markdown dans votre projet, puis la publie. Avant de publier un nouvel artefact, Claude Code demande la permission ; il pourrait dire quelque chose comme `Claude wants to publish "Deploy failures by service" (deploy-failures.html) to a private page on claude.ai`. Republier un artefact que vous avez déjà approuvé ne demande pas à nouveau.

Sélectionnez **Oui** pour publier. Claude imprime l'URL, et votre navigateur s'ouvre sur la nouvelle page. Appuyez sur `Ctrl+]` à tout moment pour rouvrir l'artefact le plus récent à partir du terminal.

Claude choisit le titre de l'artefact et un emoji pour son icône d'onglet de navigateur. Les deux apparaissent dans votre [galerie d'artefacts](#share-an-artifact) sur claude.ai et dans les liens partagés, donc demandez à Claude d'utiliser un titre ou une icône spécifique si vous en voulez un.

Pour empêcher le navigateur de s'ouvrir automatiquement lorsqu'un nouvel artefact est publié, définissez `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` dans votre environnement.

Si Claude répond qu'il ne peut pas publier, ou écrit un fichier HTML local sans lien, l'outil n'est pas activé pour votre session. Vérifiez les exigences de [Disponibilité](#availability).

<h2 id="update-an-artifact">
  Mettre à jour un artefact
</h2>

Demandez à Claude de réviser la page, ou laissez une tâche longue republier au fur et à mesure qu'elle progresse. Claude modifie le fichier sous-jacent et publie à nouveau vers la même URL.

```text wrap theme={null}
Add a per-region breakdown below the summary chart and republish.
```

Quiconque a la page ouverte voit la mise à jour sur place. Chaque publication devient une version, et à partir du contrôle **Partager** dans l'en-tête de la page, vous pouvez choisir quelle version les spectateurs voient.

Pour mettre à jour un artefact à partir d'une session différente, donnez à Claude l'URL de l'artefact et demandez-lui de le réviser. Sans l'URL, une nouvelle session crée toujours un nouvel artefact plutôt que de mettre à jour un artefact existant.

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="share-an-artifact">
  Partager un artifact
</h2>

Un nouvel artifact n'est visible que pour vous. Pour le partager, ouvrez l'artifact dans votre navigateur et utilisez le contrôle **Partager** dans l'en-tête de la page. L'en-tête vous nomme comme auteur de l'artifact, de sorte que toute personne avec laquelle vous le partagez peut voir qui a publié la page. Il renvoie également à votre galerie sur [claude.ai/code/artifacts](https://claude.ai/code/artifacts), qui répertorie chaque artifact que vous avez créé.

Les personnes avec lesquelles vous pouvez partager dépendent de votre plan :

* **Au sein de votre organisation** : sur les plans Team et Enterprise, accordez l'accès à des personnes spécifiques de votre organisation, ou à tout le monde. Les spectateurs se connectent à claude.ai en tant que membres de votre organisation pour voir la page.
* **Publiquement** : partagez un lien que n'importe qui sur Internet peut ouvrir, sans connexion à claude.ai requise. Sur les plans Pro et Max, un lien public est le seul moyen de partager un artifact. Sur les plans Team et Enterprise, le partage public est désactivé jusqu'à ce qu'un propriétaire [l'active pour l'organisation](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Laisser quelqu'un modifier avec vous
</h3>

Les personnes avec lesquelles vous partagez sont des spectateurs par défaut : elles voient chaque version que vous publiez mais ne peuvent pas modifier la page. Sur les plans Team et Enterprise, vous pouvez également faire de quelqu'un un éditeur. Dans la boîte de dialogue de partage, ajoutez une personne et changez son rôle de **spectateur** à **éditeur**.

Un éditeur publie de nouvelles versions de la même manière que vous [mettez à jour l'artifact à partir d'une autre session](#update-an-artifact) : il donne à Claude l'URL de l'artifact dans sa propre session, et Claude récupère le contenu actuel et le republié avec ses modifications. Tous ceux qui ont la page ouverte voient chaque mise à jour en direct.

<h2 id="pull-live-data-with-mcp-connectors">
  Extraire des données en direct avec les connecteurs MCP
</h2>

Un artifact peut appeler les [connecteurs MCP](/docs/fr/mcp#use-mcp-servers-from-claude-ai) chaque fois que quelqu'un le consulte, de sorte que la page affiche les données actuelles plutôt qu'une capture instantanée de la session qui l'a créé. Les appels de connecteur à partir d'artifacts sont disponibles sur les plans Pro, Max, Team et Enterprise et nécessitent Claude Code v2.1.209 ou une version ultérieure. Sur les versions antérieures, Claude publie la page avec les données que la session a rassemblées lors de sa création.

Pour créer une page sauvegardée par un connecteur, nommez le connecteur et les données que vous souhaitez dans votre prompt :

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude déclare quels connecteurs la page peut appeler lors de la publication, et la page ne peut pas appeler de connecteurs en dehors de cette déclaration. Seuls les connecteurs de votre compte claude.ai sont admissibles : Claude les nomme dans la déclaration, et lorsque quelqu'un consulte la page, chaque appel [s'exécute via la propre connexion du compte qui consulte](#how-connector-calls-work-for-viewers) à ce connecteur. Les serveurs MCP locaux que vous configurez dans Claude Code, tels que les serveurs de `.mcp.json`, peuvent fournir des données pendant que Claude crée la page, mais la page publiée ne peut pas les appeler.

La page récupère les données au chargement et peut s'actualiser à un intervalle ou lorsqu'un visiteur utilise un contrôle d'actualisation sur la page. Les réponses sont mises en cache dans le navigateur du visiteur, de sorte qu'une page rouverte s'affiche à partir des réponses mises en cache immédiatement, puis se met à jour avec les résultats actualisés.

<h3 id="how-connector-calls-work-for-viewers">
  Comment les appels de connecteur fonctionnent pour les visiteurs
</h3>

Lorsqu'une page publiée appelle un connecteur, l'appel utilise le compte de la personne qui consulte la page, et non le compte de la personne qui l'a publiée :

* **Chaque visiteur utilise ses propres connecteurs** : les appels passent par les outils connectés du compte qui consulte, de sorte que deux personnes ouvrant le même tableau de bord peuvent voir des données différentes selon ce que leurs comptes peuvent accéder. La page ne voit jamais les identifiants de personne ; claude.ai effectue les appels au nom de la page.
* **Les visiteurs approuvent d'abord l'accès** : claude.ai demande à chaque visiteur la permission avant le premier appel de connecteur de la page. Un visiteur qui refuse, ou qui n'a pas connecté un connecteur que la page utilise, voit toujours la page sans ses sections en direct.
* **Les actions utilisent également le compte du visiteur** : une page peut offrir des contrôles qui invoquent des outils de connecteur avec des effets secondaires, tels que publier un message ou mettre à jour un problème. L'action s'exécute via le compte de celui qui sélectionne le contrôle.

Lorsque vous prévoyez de partager une page sauvegardée par un connecteur, demandez à Claude d'inclure un message de secours dans chaque section en direct qui nomme le connecteur dont elle a besoin. Un visiteur qui n'a pas la connexion voit alors ce qu'il faut connecter au lieu d'une section vide.

Un artifact qui appelle des connecteurs ne peut pas être partagé via un lien public sur aucun plan. Sur les plans Team et Enterprise, vous pouvez le garder privé ou [le partager au sein de votre organisation](#share-an-artifact). Sur les plans Pro et Max, où un lien public est le seul moyen de partager, un artifact sauvegardé par un connecteur reste privé pour vous.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  La page n'affiche aucune donnée en direct pour un visiteur
</h3>

Lorsqu'une page sauvegardée par un connecteur s'affiche mais que ses sections en direct restent vides pour quelqu'un avec qui vous l'avez partagée, travaillez à travers ces causes :

* **Le visiteur n'a pas connecté le connecteur** : les connecteurs sont par compte, de sorte que chaque visiteur a besoin de sa propre connexion à chaque connecteur que la page appelle. Il peut en ajouter un sous **Paramètres > Connecteurs** sur claude.ai, puis recharger la page.
* **Le visiteur a refusé la demande de permission** : un refus dure pour le reste de ce chargement de page. Recharger la page ramène la demande de permission.
* **Les appels de connecteur sont désactivés pour l'organisation** : un propriétaire contrôle le [bouton bascule **Activer les connecteurs d'artifact**](#control-connector-calls-from-artifacts) dans les paramètres d'administration.

<h2 id="what-you-can-build">
  Ce que vous pouvez créer
</h2>

Un artefact est une seule page HTML, donc tout ce que vous pouvez exprimer en HTML, CSS et JavaScript en ligne est dans le champ d'application. Les modèles ci-dessous reviennent le plus souvent.

<h3 id="walk-through-a-change">
  Parcourir une modification
</h3>

Demandez une page qui affiche un diff ou une modification de conception avec des annotations à côté des lignes pertinentes, afin que les relecteurs puissent lire votre raisonnement à côté du code au lieu de le reconstruire à partir d'une description.

```text wrap theme={null}
Make an artifact that walks through this PR. Render the diff with margin annotations and color-code findings by severity.
```

<h3 id="compare-alternatives">
  Comparer les alternatives
</h3>

Demandez plusieurs variantes sur une page afin de pouvoir les évaluer les unes par rapport aux autres. Cela fonctionne pour les mises en page, le texte, les formes d'API ou les plans d'implémentation.

```text wrap theme={null}
Make an artifact with four distinctly different layouts for the settings panel. Vary density and grouping, and lay them out as a grid with a one-line tradeoff under each.
```

<h3 id="tune-with-interactive-controls">
  Affiner avec des contrôles interactifs
</h3>

Demandez des curseurs, des bascules ou des champs d'entrée liés à ce que vous ajustez, afin de pouvoir explorer les valeurs directement au lieu de les décrire.

```text wrap theme={null}
Build an artifact with sliders for the easing curve, duration, and delay so I can try values on this transition. Show the animation live as I move them.
```

<h3 id="bring-the-result-back-to-your-session">
  Ramener le résultat à votre session
</h3>

Un artefact peut servir d'éditeur léger pour une décision que vous remettez ensuite à Claude. Demandez un contrôle d'exportation qui produit du texte que vous pouvez coller dans le terminal, afin que le résultat de l'interaction avec la page revienne à la session au lieu de rester sur la page.

```text wrap theme={null}
Make a triage board artifact with each open issue as a draggable card across Now, Next, Later, and Cut columns. Add a "Copy as prompt" button that gives me the final ordering to paste back here.
```

<h3 id="track-work-in-progress">
  Suivre le travail en cours
</h3>

Demandez à Claude de maintenir un artefact à jour pendant qu'une tâche longue s'exécute, afin que quiconque dispose du lien puisse suivre sans lire le terminal.

```text wrap theme={null}
Turn this migration plan into a checklist artifact. Check items off as you complete them and add a note for anything you skip.
```

<h2 id="improve-the-visual-design">
  Améliorer la conception visuelle
</h2>

À partir de Claude Code v2.1.183, Claude applique une compétence de conception intégrée lorsqu'il construit un artefact, de sorte que les pages obtiennent une palette, une typographie et une mise en page délibérées sans invite supplémentaire. Cette compétence recherche également un système de conception existant dans votre projet avant de choisir le sien. Pour garder les artefacts cohérents avec la marque de votre produit, enregistrez vos jetons de conception où Claude peut les trouver, comme le [CLAUDE.md](/docs/fr/memory) du projet ou un fichier de thème dans votre référentiel :

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude traite votre système de conception comme ayant une priorité plus élevée que ses propres choix, et votre invite comme ayant une priorité plus élevée que les deux. L'en-tête et le format ci-dessus sont un exemple ; toute liste claire de couleurs, de polices et d'espacement fonctionne.

<h2 id="page-constraints">
  Contraintes de page
</h2>

Chaque artefact est une page autonome unique. Claude Code enveloppe le fichier que vous publiez dans une coque de document HTML et le sert sous une politique de sécurité du contenu (CSP) stricte, qui façonne ce que la page peut faire.

| Contrainte               | Effet                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Pas de demandes externes | La CSP bloque les scripts, les feuilles de style, les polices et les images chargées à partir de tout autre hôte, ainsi que les appels `fetch`, XHR et WebSocket. Claude intègre CSS et JavaScript et intègre les images en tant qu'URI de données afin que la page s'affiche sans aucune demande externe. [Les appels de connecteur](#pull-live-data-with-mcp-connectors) font exception : la page les transmet à claude.ai, qui effectue lui-même l'appel réseau. |
| Pas de backend           | Un artefact est une page statique. Il ne peut pas stocker les données soumises via un formulaire ou authentifier les spectateurs lui-même. Son seul moyen de récupérer des données lorsque quelqu'un le consulte est [d'appeler des connecteurs MCP](#pull-live-data-with-mcp-connectors), et non une API qui lui serait propre.                                                                                                                                    |
| Page unique              | Les liens relatifs ne se résolvent pas, car rien n'est déployé à côté de la page. Pour le contenu multi-sections, Claude utilise des ancres dans la page plutôt que des fichiers séparés.                                                                                                                                                                                                                                                                           |
| Types de fichiers source | Le fichier publié doit être `.html`, `.htm` ou `.md`. Les fichiers Markdown s'affichent en HTML stylisé.                                                                                                                                                                                                                                                                                                                                                            |
| Taille rendue            | La page rendue doit faire 16 Mio ou moins. Les grandes images intégrées sont la cause habituelle lorsqu'une publication échoue pour la taille.                                                                                                                                                                                                                                                                                                                      |

Générer un artefact utilise des jetons de sortie comme toute autre réponse, et une page stylisée est plus gourmande en jetons que le même contenu en tant que texte de terminal. CSS en ligne, JavaScript pour les contrôles interactifs, et surtout les images intégrées en tant qu'URI de données sont les principaux contributeurs. Pour réduire le coût en jetons d'un artefact :

* Préférez SVG, ou HTML et CSS, pour les diagrammes plutôt que les images raster intégrées
* Omettez l'interactivité dont vous n'avez pas besoin
* Faites en sorte que la page résume les grands ensembles de données plutôt que de les intégrer en intégralité

<h2 id="availability">
  Disponibilité
</h2>

Les artefacts nécessitent chaque condition ci-dessous. Lorsque l'une d'elles n'est pas remplie, Claude écrit un fichier HTML local ou dit qu'il ne peut pas publier à la place.

| Exigence                    | Disponible quand                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan                        | Pro, Max, Team ou Enterprise. Sur les plans Pro et Max, les artefacts sont privés pour vous jusqu'à ce que vous les partagiez, et aucune gestion d'administrateur ne s'applique. Sur les plans Team, les artefacts sont activés par défaut. Sur les plans Enterprise, un propriétaire [les active](#manage-artifacts-for-your-organization) dans les paramètres d'administration de claude.ai.                                                                                                     |
| Authentification            | La session est sauvegardée par un compte claude.ai : connectez-vous avec `/login` dans la CLI ou l'application de bureau. Les sessions Claude Tag sont connectées via l'identité de l'agent, donc aucune étape n'est nécessaire. Les sessions utilisant une clé API, un [jeton de passerelle](/docs/fr/llm-gateway) ou une identifiant de fournisseur cloud ne peuvent pas publier.                                                                                                                     |
| Fournisseur de modèle       | API Anthropic. Non disponible sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) ou [Microsoft Foundry](/docs/fr/microsoft-foundry).                                                                                                                                                                                                                                                                                                                       |
| Politique organisationnelle | Les clés de chiffrement gérées par le client (CMEK), HIPAA et [Zéro rétention de données](/docs/fr/zero-data-retention) ne sont pas activées pour l'organisation.                                                                                                                                                                                                                                                                                                                                       |
| Surface                     | CLI Claude Code version 2.1.183 ou ultérieure, ou l'application de bureau Claude version 1.13576.0 ou ultérieure. Les sessions [Claude Tag](https://claude.com/docs/claude-tag/overview) peuvent également publier des artefacts lorsque Claude Tag et les artefacts sont activés pour l'organisation. Désactivé par défaut dans les contextes [Agent SDK](/docs/fr/agent-sdk/overview), GitHub Action et MCP-server, et lorsque [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars) est défini. |

<h2 id="disable-artifacts">
  Désactiver les artefacts
</h2>

Pour désactiver les artefacts pour vos propres sessions indépendamment du paramètre de votre organisation, utilisez l'une des options suivantes :

| Méthode                                  | Paramètre                               |
| :--------------------------------------- | :-------------------------------------- |
| [Fichier de paramètres](/docs/fr/settings)    | `"disableArtifact": true`               |
| [Variable d'environnement](/docs/fr/env-vars) | `CLAUDE_CODE_DISABLE_ARTIFACT=1`        |
| [Règle de permission](/docs/fr/permissions)   | Ajouter `Artifact` à `permissions.deny` |

<h2 id="manage-artifacts-for-your-organization">
  Gérer les artefacts pour votre organisation
</h2>

Les propriétaires sur les plans Team et Enterprise contrôlent les artefacts à partir des [paramètres d'administration de claude.ai](https://claude.ai/admin-settings/claude-code). Le contenu des artefacts est stocké sur l'infrastructure exploitée par Anthropic et n'est visible que pour les membres authentifiés de l'organisation de publication, sauf si l'artefact est [partagé publiquement](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Activer ou désactiver les artefacts
</h3>

Pour activer ou désactiver les artefacts pour l'ensemble de l'organisation, allez à **Paramètres > Claude Code > Capacités** et utilisez le bouton bascule **Artefacts**. Sur les plans Enterprise avec contrôle d'accès basé sur les rôles, vous pouvez également limiter les artefacts à des rôles spécifiques : allez à **Paramètres > Rôles**, modifiez un rôle et définissez la permission **Artefacts** sous le groupe **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Contrôler les appels de connecteur à partir des artefacts
</h3>

Les [appels de connecteur à partir des artefacts](#pull-live-data-with-mcp-connectors) disposent de leur propre bouton bascule, distinct du bouton bascule **Artefacts** qui active ou désactive les artefacts. Allez à [**Paramètres > Capacités**](https://claude.ai/admin-settings/capabilities) et utilisez le bouton bascule **Activer les connecteurs d'artefacts**. Le même bouton bascule régit les appels de connecteur à partir des artefacts créés dans les conversations de claude.ai, c'est pourquoi il se trouve sous **Paramètres > Capacités** plutôt que sous **Paramètres > Claude Code**.

<h3 id="control-public-sharing">
  Contrôler le partage public
</h3>

Le partage public est désactivé par défaut sur les plans Team et Enterprise, de sorte que les membres ne peuvent partager les artefacts que dans l'organisation jusqu'à ce qu'un propriétaire l'active. Pour permettre aux membres de publier des artefacts vers des liens publics que n'importe qui peut consulter sans se connecter, allez à **Paramètres > Claude Code > Capacités** et activez **Partage externe** sous le bouton bascule **Artefacts**. Le désactiver à nouveau bloque l'accès via les liens publics existants sans modifier l'audience de chaque artefact ; l'accès reprend si vous le réactivez.

<h3 id="set-a-retention-policy">
  Définir une politique de rétention
</h3>

Pour définir la durée pendant laquelle les artefacts sont conservés avant suppression automatique, allez à **Paramètres > Contrôles de données et de confidentialité**. Vous pouvez définir des périodes de rétention distinctes pour les artefacts qui sont encore privés pour leur auteur et les artefacts qui ont été partagés.

<h3 id="review-the-audit-log">
  Examiner le journal d'audit
</h3>

La publication, le partage et la suppression d'un artefact apparaissent chacun dans le journal d'audit de votre organisation sous les types d'événements `claude_artifact_*`, la même famille utilisée pour les artefacts créés dans les conversations de claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Ajouter le domaine du visualiseur à la liste blanche
</h3>

Le visualiseur sur claude.ai charge chaque artefact à partir d'une origine `*.claudeusercontent.com` en bac à sable. Si votre organisation restreint l'accès réseau sortant, ajoutez ce domaine à votre liste blanche à côté de `claude.ai`. Consultez [Exigences d'accès réseau](/docs/fr/network-config#network-access-requirements) pour la liste complète.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  Lister et supprimer les artefacts avec l'API de conformité
</h3>

L'[API de conformité](https://docs.claude.com/en/api/compliance) fournit des points de terminaison pour lister les artefacts d'une organisation, récupérer le contenu d'une version spécifique et supprimer un artefact :

| Méthode  | Point de terminaison                                                |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

Pour les schémas de demande et de réponse, consultez la [référence de l'API de conformité](https://docs.claude.com/en/api/compliance/code/artifacts).

<h2 id="related-resources">
  Ressources connexes
</h2>

* Parcourez les [modèles d'invite et flux de travail](/docs/fr/prompt-library) qui s'associent aux artefacts
* Transformez une invite d'artefact que vous réutilisez en [compétence](/docs/fr/skills) afin de pouvoir l'invoquer en tant que commande
* [Connectez les serveurs MCP](/docs/fr/mcp) afin que Claude puisse extraire les données en direct dans un artefact pendant qu'il crée la page
