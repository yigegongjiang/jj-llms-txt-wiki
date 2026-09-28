> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer le mode auto

> Indiquez au classificateur du mode auto quels dépôts, buckets et domaines votre organisation approuve. Définissez le contexte d'environnement, remplacez les règles de blocage et d'autorisation par défaut, et inspectez votre configuration effective avec les sous-commandes CLI du mode auto.

[Le mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) permet à Claude Code de s'exécuter sans invites de permission routinières en acheminant les appels d'outils via un classificateur qui bloque tout ce qui est irréversible, destructeur ou destiné en dehors de votre environnement. Les règles de refus et de demande explicite sont évaluées avant le classificateur et bloquent ou invitent toujours. Utilisez le bloc de paramètres `autoMode` pour indiquer à ce classificateur quels dépôts, buckets et domaines votre organisation approuve, afin qu'il cesse de bloquer les opérations internes routinières.

<Note>
  Le mode auto est disponible pour tous les utilisateurs sur chaque fournisseur, y compris l'API Anthropic, [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry, et les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectées. Si Claude Code signale que le mode auto n'est pas disponible pour votre compte, consultez les [exigences complètes](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), qui couvrent également les modèles pris en charge et le contrôle au niveau de l'organisation sur les plans Team et Enterprise. Dans les versions v2.1.158 à v2.1.206, le mode auto sur Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry, et les sessions de passerelle d'applications Claude nécessitaient de définir `CLAUDE_CODE_ENABLE_AUTO_MODE=1` ; v2.1.207 a supprimé cette exigence.
</Note>

Par défaut, le classificateur approuve uniquement le répertoire de travail et les télécommandes configurées du dépôt actuel. Les actions comme pousser vers l'organisation de contrôle de source de votre entreprise ou écrire dans un bucket cloud d'équipe sont bloquées jusqu'à ce que vous les ajoutiez à `autoMode.environment`.

Pour savoir comment les sessions se retrouvent en mode auto et ce que le classificateur bloque par défaut, consultez [le mode auto sur la page Modes de permission](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode). Cette page est la référence de configuration.

Cette page couvre comment :

* [Ajouter un point de contrôle humain](#add-a-human-checkpoint) pour les poussées et les demandes de tirage avec `permissions.ask`
* [Choisir où définir les règles](#where-the-classifier-reads-configuration) dans CLAUDE.md, les paramètres utilisateur et les paramètres gérés
* [Définir l'infrastructure approuvée](#define-trusted-infrastructure) avec `autoMode.environment`
* [Générer des entrées d'environnement](#generate-environment-entries) avec `/auto-mode-setup`
* [Remplacer les règles de blocage et d'autorisation](#override-the-block-and-allow-rules) quand les valeurs par défaut ne correspondent pas à votre pipeline
* [Modifier les règles depuis `/permissions`](#edit-rules-from-permissions) sans ouvrir un fichier de paramètres
* [Acheminer toutes les commandes shell via le classificateur](#route-all-shell-commands-through-the-classifier) avec `autoMode.classifyAllShell`
* [Inspectez votre configuration effective](#inspect-the-defaults-and-your-effective-config) avec les sous-commandes `claude auto-mode`
* [Examinez les refus](#review-denials) pour savoir ce qu'il faut ajouter ensuite

<h2 id="common-boundaries">
  Limites communes
</h2>

Le mode auto permet les poussées vers n'importe quelle branche du référentiel sur lequel vous travaillez, y compris la branche par défaut, et la création de demandes de tirage par défaut. Une branche non-par défaut dont le nom la marque comme cible de déploiement ou de publication, comme `production`, `release`, ou `gh-pages`, n'est pas couverte par ce défaut : le classificateur juge une poussée là-bas selon ses propres termes, y compris comme un déploiement en production. Le contenu de la poussée est également toujours vérifié, donc une poussée forcée, un secret entrant dans le commit, ou une modification qui enverrait des secrets en dehors du référentiel lorsque CI ou un pipeline de déploiement l'exécute reste bloquée.

<Info>Avant v2.1.211, le classificateur permettait les poussées uniquement vers votre branche de travail, les branches créées par Claude, et les poussées régulières vers la branche par défaut.</Info>

Si vous souhaitez un point de contrôle humain avant les commandes de poussée et de demande de tirage de Claude, ajoutez des règles de permission : les [recettes ci-dessous](#add-a-human-checkpoint) maintiennent le mode auto activé pour tout le reste.

<h3 id="add-a-human-checkpoint">
  Ajouter un point de contrôle humain
</h3>

Le mécanisme le plus direct est [`permissions.ask`](/docs/fr/permissions#permission-rule-syntax). Les règles ask délimitées par le contenu comme celles ci-dessous sont évaluées avant le classificateur et forcent toujours une invite de permission, même en mode auto, car une règle ask explicite est votre intention déclarée d'être invité pour cette action. Ajoutez les règles dans vos [paramètres](/docs/fr/settings#where-settings-live) :

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

Ces règles correspondent aux commandes qui commencent par `git push` ou `gh pr create`. Une poussée que Claude écrit d'une autre manière, comme `git -C <dir> push` ou `git -c <key>=<value> push`, [ne correspond pas à la règle](/docs/fr/permissions#bash-rule-limits), donc elle n'est pas contrôlée. Pour un point de contrôle qui inspecte le texte complet de la commande, ajoutez un [hook PreToolUse](/docs/fr/hooks#pretooluse).

Choisissez le mécanisme qui correspond à la fermeté requise de la limite :

| Limite                               | Mécanisme                                                                          | Comportement en mode auto                                                                                                                                                                                                                         |
| :----------------------------------- | :--------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Inviter avant l'action               | `permissions.ask`                                                                  | Invite toujours pour une commande qui correspond à une règle délimitée par le contenu comme la recette ci-dessus. Le classificateur ne peut pas approuver automatiquement une action correspondante.                                              |
| Ne jamais exécuter l'action          | `permissions.deny`                                                                 | Bloque avant que le classificateur ne soit consulté. Ni le classificateur ni l'intention de l'utilisateur ne peuvent le contourner.                                                                                                               |
| Limite ponctuelle pour cette session | Énoncez-la dans la conversation, comme « ne pas pousser jusqu'à ce que j'examine » | Le classificateur bloque les actions correspondantes, mais la limite peut être perdue si la [compaction de contexte](/docs/fr/costs#reduce-token-usage) supprime le message qui l'a énoncée. Utilisez une règle ask ou deny pour une garantie durable. |

<h2 id="where-the-classifier-reads-configuration">
  Où le classificateur lit la configuration
</h2>

Le classificateur lit le même contenu [CLAUDE.md](/docs/fr/memory) que Claude lui-même charge, donc une instruction comme « ne jamais forcer un push » dans le CLAUDE.md de votre projet oriente à la fois Claude et le classificateur. Commencez par là pour les conventions de projet et les règles de comportement.

Pour les règles qui s'appliquent à plusieurs projets, comme l'infrastructure de confiance ou les règles de refus à l'échelle de l'organisation, utilisez le bloc de paramètres `autoMode`. Le classificateur lit `autoMode` à partir des portées suivantes :

| Portée                            | Fichier                                         | Utilisation                                                    |
| :-------------------------------- | :---------------------------------------------- | :------------------------------------------------------------- |
| Un développeur                    | `~/.claude/settings.json`                       | Infrastructure de confiance personnelle                        |
| À l'échelle de l'organisation     | [Paramètres gérés](/docs/fr/server-managed-settings) | Infrastructure de confiance distribuée à tous les développeurs |
| Drapeau `--settings` ou Agent SDK | JSON en ligne                                   | Remplacements par invocation pour l'automatisation             |

Le classificateur ne lit pas `autoMode` à partir des paramètres de projet dans `.claude/settings.json` ou `.claude/settings.local.json`. Les deux fichiers se trouvent dans le répertoire du dépôt, donc un dépôt archivé ou une étape de construction pourrait autrement injecter ses propres règles d'autorisation. Avant la v2.1.207, le classificateur lisait également `.claude/settings.local.json` ; déplacez tout bloc `autoMode` dans ce fichier vers `~/.claude/settings.json`. L'exclusion de `.claude/settings.local.json` ferme également le cas où un dépôt valide le fichier ou un outil local ou une étape de construction l'écrit.

Les entrées de chaque portée sont combinées. Un développeur peut étendre `environment`, `allow`, `soft_deny` et `hard_deny` avec des entrées personnelles mais ne peut pas supprimer les entrées que les paramètres gérés fournissent. Parce que les règles d'autorisation agissent comme des exceptions aux règles de blocage logiciel à l'intérieur du classificateur, une entrée `allow` ajoutée par un développeur peut remplacer une entrée `soft_deny` d'organisation : la combinaison est additive, pas une limite de politique stricte.

<Note>
  Le classificateur est une deuxième porte qui s'exécute après le [système de permissions](/docs/fr/permissions). Pour les actions qui ne doivent jamais s'exécuter indépendamment de l'intention de l'utilisateur ou de la configuration du classificateur, utilisez `permissions.deny` dans les paramètres gérés, qui bloque l'action avant que le classificateur ne soit consulté et ne peut pas être remplacée.
</Note>

<h2 id="define-trusted-infrastructure">
  Définir l'infrastructure de confiance
</h2>

Pour la plupart des organisations, `autoMode.environment` est le seul champ que vous devez définir. Il indique au classificateur quels dépôts, buckets et domaines sont de confiance : le classificateur l'utilise pour décider ce que signifie « externe », donc toute destination non listée est une cible d'exfiltration potentielle.

À partir de Claude Code v2.1.198, `claude auto-mode defaults` affiche trois types d'entrées d'environnement. Les versions antérieures à v2.1.195 affichent uniquement les cinq premiers emplacements de confiance.

* **Emplacements de contexte** : décrivez votre organisation, votre pile technologique et votre posture de sécurité afin que le classificateur lise les autres règles dans votre contexte. Chacun par défaut à `None configured` ou à l'hypothèse conservatrice nommée à côté :
  * **Organisation**
  * **Utilisation principale de Claude Code** : par défaut développement logiciel
  * **Fournisseur(s) cloud**
  * **Visibilité du dépôt** : un dépôt est supposé privé sauf si son hôte distant et son nom l'indiquent autrement, ou le classificateur lit une vérification de visibilité antérieure dans la conversation montrant qu'il est public.

    Dans les demandes du classificateur envoyées par Claude Code lui-même, le classificateur lit vos messages et les commandes que Claude exécute, pas leur sortie. La preuve doit être quelque chose que le classificateur peut lire, comme votre propre message nommant le dépôt comme public ; la sortie d'un `gh repo view` seul ne l'atteint pas. La vérification des preuves de transcription nécessite Claude Code v2.1.200 ou ultérieur
  * **Partage interne / hébergement d'extraits** : les services publics de pâte et de gist sont traités comme en dehors de la limite de confiance jusqu'à ce que vous en nomiez un
  * **CLI spécifiques à l'organisation**
  * **Gestion des secrets**
  * **Cibles de déploiement CI/CD**
  * **Posture réseau**
  * **Confinement d'hôte** : par défaut une machine de développeur ordinaire ou un exécuteur CI avec internet ouvert. Si Claude Code s'exécute dans un conteneur, une VM ou un pod avec une liste d'autorisation de sortie ou des voisins qu'il ne doit pas toucher, nommez les hôtes autorisés, si le point de terminaison des métadonnées cloud doit être accessible, et quel projet cloud, cluster ou registre la tâche utilise et sous quelle identité. Jusqu'à ce que cette entrée nomme cette identité, le classificateur [bloque](/docs/fr/permission-modes#what-the-classifier-blocks-by-default) les demandes pour les propres identifiants de l'hôte. Nécessite Claude Code v2.1.257 ou ultérieur
  * **Espaces de déploiement protégés / environnements** : revient à l'heuristique des cibles distantes sensibles jusqu'à ce que vous les nomiez
  * **Rétention des données / déclassification**
* **Emplacements de confiance** : nommez ce que le classificateur traite comme à l'intérieur de votre limite. Les emplacements sont Dépôt de confiance, Contrôle de source, Domaines internes de confiance, Buckets cloud de confiance, Services internes clés et Registre de paquets interne. Les entrées de dépôt et de contrôle de source par défaut au dépôt de travail et ses télécommandes configurées. Chaque autre emplacement de confiance par défaut à `None configured`, donc rien d'autre n'est de confiance jusqu'à ce que vous l'ajoutiez. La visibilité d'un dépôt ne s'applique qu'au matériel confidentiel : un dépôt privé est une destination acceptable pour le matériel confidentiel, mais rendre un dépôt privé ne vide jamais les secrets ou les données personnelles ou confiées en lui, et le classificateur traite le contenu porté, réorienté ou d'abord lu de l'extérieur du dépôt de travail comme n'étant pas le propre travail de ce dépôt. Cette portée nécessite Claude Code v2.1.203 ou ultérieur.
* **Emplacements de sensibilité** : nommez ce que les règles de protection traitent comme à haut risque. Les emplacements sont Emplacements de données sensibles et audiences, Cibles distantes sensibles et Portées IaC protégées. Chacun par défaut à une large heuristique, comme traiter tout hôte ou espace de noms dont le nom porte `prod` ou `production` comme une cible distante sensible, donc les règles de protection sont actives avant que vous configuriez quoi que ce soit. Nommer des cibles concrètes dans un emplacement de sensibilité fait appliquer ces règles aux cibles nommées au lieu de l'heuristique.

<Info>Avant v2.1.211, les emplacements de contexte incluaient également une entrée Branches par défaut / protégées qui traitait `main` et `master` comme protégées jusqu'à ce que vous en nomiez d'autres. v2.1.211 l'a supprimée : [les poussées vers n'importe quelle branche du dépôt sur lequel vous travaillez](#common-boundaries) sont autorisées par défaut, donc il n'y a pas de défaut de branche protégée à configurer.</Info>

Pour ajouter vos propres entrées aux côtés des valeurs par défaut, incluez la chaîne littérale `"$defaults"` dans le tableau. Les entrées par défaut sont épissées à cette position, donc vos entrées personnalisées peuvent aller avant ou après elles.

L'exemple suivant conserve les entrées par défaut et ajoute les dépôts, buckets, domaines et services d'une organisation.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

Après avoir enregistré vos paramètres, exécutez `claude auto-mode config` pour [confirmer que les règles effectives](#inspect-the-defaults-and-your-effective-config) incluent vos entrées.

Les entrées sont de la prose, pas des regex ou des modèles d'outils. Le classificateur les lit comme des règles en langage naturel. Écrivez-les comme vous décririez votre infrastructure à un nouvel ingénieur. Une section d'environnement approfondie couvre :

* **Organisation** : le nom de votre entreprise et ce pour quoi Claude Code est principalement utilisé, comme le développement logiciel, l'automatisation de l'infrastructure ou l'ingénierie des données
* **Contrôle de source** : chaque organisation GitHub, GitLab ou Bitbucket vers laquelle vos développeurs poussent
* **Fournisseurs cloud et buckets de confiance** : noms ou préfixes de buckets que Claude devrait pouvoir lire et écrire
* **Domaines internes de confiance** : noms d'hôtes pour les API, tableaux de bord et services à l'intérieur de votre réseau, comme `*.internal.example.com`
* **Services internes clés** : CI, registres d'artefacts, index de paquets internes, outils d'incident
* **Registre de paquets interne** : le registre npm, PyPI ou autre privé par lequel les installations doivent être acheminées, donc les installations qui le contournent pour un registre public sont bloquées
* **Emplacements de données sensibles et audiences** : les buckets, bases de données ou chemins qui contiennent des données personnelles, des données commerciales confidentielles, des identifiants, des données réglementées ou du matériel similaire sensible, et les audiences avec lesquelles les données de chaque emplacement peuvent être partagées, afin que le classificateur protège ces emplacements au lieu de deviner à partir du contenu. Claude Code v2.1.195 à v2.1.197 nomme cette entrée Emplacements PII / données réglementées et couvrent uniquement les emplacements qui contiennent des données personnelles ou réglementées, sans la dimension d'audience
* **Cibles distantes sensibles** : les espaces de noms, hôtes ou conteneurs qui comptent comme production, donc les shells distants et les transferts de port vers eux ont besoin de votre approbation explicite
* **Portées IaC protégées** : les ressources d'infrastructure dont l'application ou la destruction doivent toujours vous obliger à nommer le changement
* **Contexte supplémentaire** : contraintes de secteur réglementé, infrastructure multi-locataire ou exigences de conformité qui affectent ce que le classificateur devrait traiter comme risqué

Le Registre de paquets interne, les Emplacements de données sensibles et audiences, les Cibles distantes sensibles et les entrées Portées IaC protégées nécessitent Claude Code v2.1.195 ou ultérieur. Les versions antérieures les lisent toujours comme du contexte simple mais n'ont pas les règles intégrées qui les ciblent.

Un modèle de démarrage utile : remplissez les champs entre crochets et supprimez les lignes qui ne s'appliquent pas.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

Plus le contexte que vous donnez est spécifique, mieux le classificateur peut distinguer les opérations internes de routine des tentatives d'exfiltration.

Vous n'avez pas besoin de tout remplir à la fois. Un déploiement raisonnable : commencez par les valeurs par défaut et ajoutez votre organisation de contrôle de source et vos services internes clés, ce qui résout les faux positifs les plus courants comme pousser vers vos propres dépôts. Ajoutez ensuite les domaines de confiance et les buckets cloud. Remplissez le reste à mesure que les blocages surviennent.

<h2 id="generate-environment-entries">
  Générer des entrées d'environnement avec `/auto-mode-setup`
</h2>

Exécutez `/auto-mode-setup` pour que Claude Code rédige des entrées `autoMode.environment`, et parfois aussi des [entrées de règles](#override-the-block-and-allow-rules), à partir de votre projet et de vos sessions récentes dans celui-ci. Si vous acceptez le brouillon, Claude Code l'écrit dans `~/.claude/settings.json`.

<Note>
  `/auto-mode-setup` nécessite un plan Pro, Max ou Team et Claude Code v2.1.228 ou ultérieur. Sur Windows natif, il nécessite v2.1.233 ou ultérieur. Vous ne pouvez pas l'exécuter dans une [session cloud](/docs/fr/claude-code-on-the-web). Il a également besoin de [récupération de drapeaux de fonctionnalités](/docs/fr/env-vars#features-that-need-feature-flag-fetching), donc vous ne pouvez pas l'exécuter dans une session où vous avez désactivé la récupération de drapeaux.
</Note>

<h3 id="what-auto-mode-setup-reads">
  Ce que `/auto-mode-setup` lit
</h3>

Si `~/.claude/settings.json` contient déjà des entrées `autoMode`, Claude Code commence par vous demander si vous souhaitez ajouter à votre liste d'environnement ou la remplacer, et conserve les règles que vous avez écrites de toute façon. Claude Code vous demande ensuite comment vous utilisez ce projet et propose deux analyses optionnelles avant d'analyser quoi que ce soit. Dans l'analyse, Claude Code lit toujours ces sources :

* Le `CLAUDE.md`, `README.md`, les fichiers de configuration et les télécommandes git de ce projet
* Vos paramètres `autoMode` et `permissions.allow`
* Les hôtes, les buckets et les noms de commandes à partir des commandes que Claude a exécutées dans vos sessions récentes dans ce projet, jamais vos messages

Les deux analyses optionnelles ajoutent une source chacune :

* Le premier mot de chaque commande dans votre historique shell
* Les hôtes distants et les noms des référentiels sous votre répertoire personnel

<h3 id="review-and-save-the-draft">
  Examiner et enregistrer le brouillon
</h3>

Claude Code analyse en arrière-plan, puis vous montre le brouillon. Vous l'acceptez ou le rejetez dans son ensemble, donc modifiez `~/.claude/settings.json` après pour ajuster les entrées individuelles. Lorsque vous acceptez, Claude Code écrit le brouillon et le réconcilie avec les paramètres que vous avez déjà :

* Claude Code écrit la liste `environment` sans `"$defaults"`, car le brouillon énumère les entrées intégrées qu'il a laissées inchangées
* Claude Code inclut `"$defaults"` dans chacune des listes `allow`, `soft_deny` et `hard_deny` auxquelles le brouillon ajoute des entrées, sauf si vous avez déjà écrit une liste `allow` sans elle, de sorte que les [règles intégrées](#override-the-block-and-allow-rules) que vous n'avez pas remplacées restent en vigueur
* Après l'enregistrement, Claude Code propose de supprimer les règles `permissions.allow` dans `~/.claude/settings.json` que le mode auto ignore, telles que `Bash(*)`, ou qui approuvent automatiquement les commandes destructrices

Exécutez ensuite `claude auto-mode config` pour [voir le résultat effectif](#inspect-the-defaults-and-your-effective-config).

<h3 id="turn-off-auto-mode-setup">
  Désactiver `/auto-mode-setup`
</h3>

Une fois que le mode auto a bloqué plusieurs actions et que vous n'avez toujours pas d'entrées `autoMode.environment`, Claude Code affiche une boîte de dialogue intitulée « Enseigner au mode auto votre environnement ? » à la fin d'un tour et vous propose d'exécuter `/auto-mode-setup` pour vous. Pour arrêter l'offre mais conserver la commande, sélectionnez **Ne plus afficher** dans cette boîte de dialogue.

Pour désactiver à la fois la commande et l'offre, ajoutez cette entrée [`skillOverrides`](/docs/fr/skills#override-skill-visibility-from-settings) à `~/.claude/settings.json` :

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup` est une commande intégrée plutôt qu'une [compétence groupée](/docs/fr/skills#bundled-skills), donc cette entrée `skillOverrides` s'y applique toujours, mais [`disableBundledSkills`](/docs/fr/settings-reference#disablebundledskills) ne la désactive pas.

<h2 id="override-the-block-and-allow-rules">
  Remplacer les règles de blocage et d'autorisation
</h2>

Trois champs supplémentaires vous permettent de remplacer les listes de règles intégrées du classificateur :

* `autoMode.hard_deny` : limites de sécurité inconditionnelles
* `autoMode.soft_deny` : actions destructrices que l'intention de l'utilisateur peut lever
* `autoMode.allow` : exceptions aux règles de blocage logiciel

Chacun est un tableau de descriptions en prose, lu comme des règles en langage naturel. Pour les blocages durs basés sur des motifs d'outils qui s'exécutent avant le classificateur, utilisez [`permissions.deny`](/docs/fr/permissions).

À l'intérieur du classificateur, la précédence fonctionne en quatre niveaux :

* Les règles `hard_deny` bloquent inconditionnellement. L'intention de l'utilisateur et les exceptions `allow` ne s'appliquent pas.
* Les règles `soft_deny` bloquent ensuite. L'intention de l'utilisateur et les exceptions `allow` peuvent remplacer celles-ci.
* Les règles `allow` remplacent ensuite les règles `soft_deny` correspondantes comme exceptions.
* L'intention explicite de l'utilisateur remplace les blocages souples restants : si le message de l'utilisateur décrit directement et spécifiquement l'action exacte que Claude est sur le point de prendre, le classificateur l'autorise même quand une règle `soft_deny` correspond.

Les demandes générales ne comptent pas comme une intention explicite. Demander à Claude de « nettoyer le dépôt » n'autorise pas la poussée forcée, mais demander à Claude de « forcer la poussée de cette branche » le fait.

Pour assouplir, ajoutez à `allow` quand le classificateur signale à plusieurs reprises un motif courant que les exceptions par défaut ne couvrent pas. Pour renforcer, ajoutez à `soft_deny` pour les risques destructeurs spécifiques à votre environnement que les valeurs par défaut manquent, ou à `hard_deny` pour les limites de sécurité qui ne doivent jamais être franchies.

Pour conserver les règles intégrées tout en ajoutant les vôtres, incluez la chaîne littérale `"$defaults"` dans le tableau. Les règles par défaut sont insérées à cette position, donc vos règles personnalisées peuvent aller avant ou après elles, et vous continuez à hériter des mises à jour à mesure que la liste intégrée change entre les versions.

L'exemple suivant conserve les valeurs par défaut dans les quatre listes et ajoute des règles spécifiques à l'organisation à chacune.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  La définition de l'un de `environment`, `allow`, `soft_deny` ou `hard_deny` sans `"$defaults"` remplace la liste par défaut entière pour cette section. Si vous définissez un tableau sans `"$defaults"`, vous rejetez les règles intégrées pour cette section :

  * `soft_deny` : chaque règle de blocage intégrée, y compris la poussée forcée, `curl | bash`, les déploiements en production et le contournement du mode auto
  * `hard_deny` : la règle intégrée d'exfiltration de données
</Danger>

Chaque section est évaluée indépendamment, donc la définition de `environment` seule laisse les listes `allow`, `soft_deny` et `hard_deny` par défaut intactes.

Omettez `"$defaults"` uniquement quand vous avez l'intention de prendre la responsabilité complète de la liste. Pour ce faire en toute sécurité, exécutez `claude auto-mode defaults` pour imprimer les règles intégrées, copiez-les dans votre fichier de paramètres, puis examinez chaque règle par rapport à votre propre pipeline et tolérance au risque.

<h2 id="edit-rules-from-permissions">
  Modifier les règles depuis `/permissions`
</h2>

Pour afficher et modifier les règles du classificateur sans ouvrir un fichier de paramètres, exécutez [`/permissions`](/docs/fr/permissions#manage-permissions) et sélectionnez l'onglet **Auto mode**. L'onglet nécessite Claude Code v2.1.246 ou une version ultérieure, et il n'apparaît que lorsque [le mode auto est disponible](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) pour votre session.

L'onglet répertorie les entrées `allow`, `soft_deny`, `hard_deny` et `environment` de chacune des [portées que le classificateur lit](#where-the-classifier-reads-configuration), et indique si les règles intégrées sont en vigueur pour chaque section. Claude Code affiche les entrées des [paramètres gérés](/docs/fr/server-managed-settings) ou l'indicateur `--settings` en lecture seule, et enregistre chaque modification que vous apportez à l'onglet dans `~/.claude/settings.json`. À partir de l'onglet, vous pouvez :

* Ajouter, modifier ou supprimer des règles dans les sections `allow`, `soft_deny` et `hard_deny`. Lorsque vous ajoutez la première règle à une section, Claude Code insère également `"$defaults"` afin que les [règles intégrées](#override-the-block-and-allow-rules) restent en vigueur.
* Désactiver ou réactiver les règles intégrées pour `allow`, `soft_deny` ou `hard_deny`. Claude Code enregistre le choix en ajoutant ou en supprimant `"$defaults"` dans votre liste pour cette section, donc une section a besoin d'au moins une règle de votre part avant que vous puissiez désactiver ses règles intégrées.
* Modifier les entrées `environment` en tant que document unique dans votre éditeur. Si vous n'avez pas encore configuré d'entrées `environment`, Claude Code vous demande d'abord si vous souhaitez remplacer l'environnement intégré, puis ouvre l'éditeur sur le texte intégré complet. Lorsque vous enregistrez, Claude Code remplace votre tableau `autoMode.environment` par le document. Incluez la ligne `"$defaults"` pour [conserver les entrées intégrées](#define-trusted-infrastructure).

<h2 id="route-all-shell-commands-through-the-classifier">
  Acheminer toutes les commandes shell via le classificateur
</h2>

Par défaut, les règles étroites Bash et PowerShell telles que `Bash(npm test)` restent en vigueur en mode auto, et Claude Code les résout avant que le classificateur ne s'exécute, sauf si la commande porte des [domaines autorisés par commande](/docs/fr/sandboxing#per-command-allowed-domains-in-auto-mode). Claude Code suspend uniquement les règles larges qui accordent l'exécution de code arbitraire, telles que `Bash(*)` ou les interpréteurs avec caractères génériques, ainsi que chaque règle qui nomme [`Monitor`](/docs/fr/tools-reference#monitor-tool), car les commandes Monitor s'exécutent via le shell. Cela signifie qu'une règle étroite peut toujours laisser passer un argument destructeur sans que le classificateur ne le voie, par exemple un chemin de script ou un drapeau que le préfixe de la règle n'avait pas anticipé.

Définissez `autoMode.classifyAllShell` sur `true` pour suspendre chaque règle d'autorisation Bash et PowerShell pendant que le mode auto est actif, afin que le classificateur évalue chaque commande shell indépendamment de votre liste d'autorisation.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Cela échange la latence contre la couverture : une commande qu'une règle d'autorisation aurait approuvée instantanément attend maintenant une décision du classificateur, et chaque commande shell compte comme un appel au classificateur.

Le paramètre s'applique uniquement pendant que le mode auto est actif, et vos règles d'autorisation se comportent normalement dans les autres modes de permission.

<Note>
  `autoMode.classifyAllShell` nécessite Claude Code v2.1.193 ou version ultérieure. Les versions antérieures ignorent la clé et continuent à appliquer les règles d'autorisation shell étroites en mode auto.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Inspectez les paramètres par défaut et votre configuration effective
</h2>

Les sous-commandes `claude auto-mode` vous aident à inspecter, valider et réinitialiser votre configuration.

Imprimez les règles `environment`, `allow`, `soft_deny` et `hard_deny` intégrées au format JSON :

```bash theme={null}
claude auto-mode defaults
```

Pour lire la formulation complète d'une règle sans passer par `jq`, passez `--label` avec le début du libellé de la règle, par exemple `claude auto-mode defaults --label 'Git Destructive'`. La correspondance est un préfixe insensible à la casse sur le libellé de chaque règle, et les sections sans correspondance s'affichent comme des listes vides. Nécessite Claude Code v2.1.208 ou version ultérieure.

Imprimez ce que le classificateur utilise réellement au format JSON, avec vos paramètres appliqués où définis et les valeurs par défaut sinon :

```bash theme={null}
claude auto-mode config
```

`defaults` et `config` impriment tous les quatre listes de règles comme un seul objet JSON, avec chaque règle sous forme de chaîne de prose. Voici un exemple tronqué :

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, or hardcoding test cases. Placeholder means authored as a placeholder — a file or value copied from a real secret or sensitive path is never a test artifact (see Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), deleting remote branches, tags, or releases, or rewriting remote history. Also `git commit --amend` when the commit being rewritten is not the agent's own unpushed work: either no prior `git commit` is visible (HEAD pre-dates the session), or a `git push` of the current branch is visible after the most recent commit (it has been pushed). Clears when the user asked to amend/reword/fixup, or when it is a message-only reword (`--amend -m …`, nothing newly staged) of a commit the agent visibly created this session.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: The git repository the agent started in (its working directory) and its configured remote(s). When the repo's public/private visibility is given — by the Repository visibility entry or the user's own message — use it to scope what is OK to commit or push there: confidential material is fine in a private repo; in a public one, only that repo's own work is — and content ported, repointed, or first read from outside this session's repo is not its own work, whoever directed the port. Visibility scopes confidential material only: secrets and sensitive data (personal & entrusted) are never cleared into any repo by its visibility (see Definitions).",
    ...
  ]
}
```

Obtenez des commentaires d'IA sur vos règles personnalisées `allow`, `soft_deny` et `hard_deny` :

```bash theme={null}
claude auto-mode critique
```

Exécutez `claude auto-mode config` après avoir enregistré vos paramètres pour confirmer que les règles effectives sont celles que vous attendez, avec `"$defaults"` développé en place. Si vous avez écrit des règles personnalisées, `claude auto-mode critique` les examine et signale les entrées ambiguës, redondantes ou susceptibles de causer des faux positifs.

Pour abandonner vos personnalisations et revenir aux paramètres par défaut intégrés, exécutez la sous-commande reset. Elle nécessite Claude Code v2.1.212 ou version ultérieure et supprime la section `autoMode` de votre fichier de paramètres utilisateur :

```bash theme={null}
claude auto-mode reset
```

La commande résume ce qu'elle supprimera et demande `Reset auto mode configuration to defaults?` avant d'écrire ; passez `--yes` pour ignorer la confirmation. Reset modifie uniquement `~/.claude/settings.json` : les règles `autoMode` des [paramètres gérés par le serveur](/docs/fr/server-managed-settings) ou l'indicateur `--settings` s'appliquent toujours.

<h2 id="review-denials">
  Examiner les refus
</h2>

Pour examiner et réessayer les actions que le classificateur du mode automatique a refusées, ouvrez `/permissions` et sélectionnez l'onglet **Recently denied**, où Claude Code enregistre chaque refus. Appuyez sur `r` sur une action refusée pour la marquer pour réessai : lorsque vous quittez la boîte de dialogue, Claude Code envoie un message indiquant au modèle qu'il peut réessayer cet appel d'outil et reprend la conversation.

Lorsque le classificateur produit [aucun verdict sur l'action](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action), parce qu'une vérification de sécurité distincte du mode automatique a refusé la propre demande du classificateur ou sa réponse n'a pas été analysée, Claude Code refuse l'action sans l'enregistrer sous **Recently denied**. L'entrée d'erreur liée couvre ce que Claude est informé et comment exécuter l'action si vous en avez besoin.

<h3 id="fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry">
  Corriger un refus avec une règle d'autorisation, une entrée d'environnement ou un réessai
</h3>

Pour voir ce que le classificateur a bloqué, trouvez l'appel d'outil dans la conversation. Si l'appel apparaît raccourci ou plié dans une ligne de résumé telle que `Ran 3 shell commands`, appuyez sur `Ctrl+O` pour ouvrir le [visionneuse de transcription](/docs/fr/interactive-mode#transcript-viewer), qui l'étend.

Deux autres endroits à l'écran qui signalent les refus omettent la commande ou l'URL : l'avis près de la zone de saisie, tel que `bash denied by auto mode · [Data Exfiltration] · /permissions`, indique l'outil et la raison, et l'onglet **Recently denied** répertorie une commande shell par la description que Claude a écrite pour elle. Pour capturer l'entrée exacte de ces refus par programmation, ajoutez un [`PermissionDenied` hook](/docs/fr/hooks#permissiondenied), qui la reçoit en tant que `tool_input`.

Le texte sous l'appel vous indique s'il y a quelque chose à corriger. Le texte qui signale un problème avec le classificateur lui-même, tel qu'un modèle qui `is temporarily unavailable` ou une erreur du classificateur, signifie que Claude Code a bloqué l'appel sans verdict final du classificateur ; consultez [Auto mode cannot determine the safety of an action](/docs/fr/errors#auto-mode-cannot-determine-the-safety-of-an-action) pour savoir quoi faire. Sinon, une ligne lisant `Denied by auto mode classifier` avec une raison telle que `[Production Deploy]` ou `Blocked by classifier` signifie que le classificateur a jugé l'appel non sécurisé, alors choisissez la correction parmi ce que l'appel tentait d'atteindre ou de faire :

* Une destination dont Claude a besoin tout au long de la tâche, telle qu'un registre de paquets, un domaine interne ou un hôte de référentiel : ajoutez-la à `autoMode.environment`.
* Une commande que vous souhaitez exécuter sans révision à partir de maintenant : ajoutez une règle `allow`.
* Une action ponctuelle que vous aviez l'intention de faire : indiquez cette intention dans votre prochain message et laissez Claude réessayer.

Vous pouvez ajouter l'entrée d'environnement ou la règle `allow` à partir de l'onglet [**Auto mode** de la boîte de dialogue `/permissions`](#edit-rules-from-permissions).

Dans la plupart des sessions, le nom de la raison nomme la règle que le classificateur a mise en correspondance, entre crochets, telle que `[Data Exfiltration]` ou `[Production Deploy]`, et certaines sessions exécutent un modèle de classificateur qui ajoute une brève explication. Claude Code sélectionne le modèle de classificateur, donc la forme que vous voyez n'est pas quelque chose que vous configurez.

<h3 id="fix-repeated-denials">
  Corriger les refus répétés
</h3>

Les refus répétés pour la même destination signifient généralement que le classificateur manque de contexte. Ajoutez cette destination à `autoMode.environment`, ou [exécutez `/auto-mode-setup`](#generate-environment-entries) pour que Claude Code rédige les entrées, puis exécutez `claude auto-mode config` pour confirmer que la modification a pris effet.

Pour réagir aux refus par programmation, utilisez le [`PermissionDenied` hook](/docs/fr/hooks#permissiondenied).

<h2 id="see-also">
  Voir aussi
</h2>

* [Modes de permission](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) : qu'est-ce que le mode auto, ce qu'il bloque par défaut et quelles sessions le démarrent
* [Paramètres gérés](/docs/fr/server-managed-settings) : déployer la configuration `autoMode` dans votre organisation
* [Permissions](/docs/fr/permissions) : règles d'autorisation, de demande et de refus qui s'appliquent avant l'exécution du classificateur
* [Tous les paramètres](/docs/fr/settings-reference#automode) : chaque clé de paramètres, y compris `autoMode`
