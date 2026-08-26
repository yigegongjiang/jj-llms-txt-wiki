> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurer le mode auto

> Indiquez au classificateur du mode auto quels dépôts, buckets et domaines votre organisation approuve. Définissez le contexte d'environnement, remplacez les règles de blocage et d'autorisation par défaut, et inspectez votre configuration effective avec les sous-commandes CLI du mode auto.

[Le mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) permet à Claude Code de s'exécuter sans invites de permission routinières en acheminant les appels d'outils via un classificateur qui bloque tout ce qui est irréversible, destructeur ou destiné en dehors de votre environnement. Les règles de refus et de demande explicite sont évaluées avant le classificateur et bloquent ou invitent toujours. Utilisez le bloc de paramètres `autoMode` pour indiquer à ce classificateur quels dépôts, buckets et domaines votre organisation approuve, afin qu'il cesse de bloquer les opérations internes routinières.

<Note>
  Le mode auto est disponible pour tous les utilisateurs sur chaque fournisseur, y compris l'API Anthropic, Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry, et les sessions de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectées. Si Claude Code signale que le mode auto n'est pas disponible pour votre compte, consultez les [exigences complètes](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), qui couvrent également les modèles pris en charge et l'activation du propriétaire sur les plans Team et Enterprise. Dans les versions v2.1.158 à v2.1.206, le mode auto sur Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry, et les sessions de passerelle d'applications Claude nécessitaient de définir `CLAUDE_CODE_ENABLE_AUTO_MODE=1` ; v2.1.207 a supprimé cette exigence.
</Note>

Par défaut, le classificateur approuve uniquement le répertoire de travail et les télécommandes configurées du dépôt actuel. Les actions comme pousser vers l'organisation de contrôle de source de votre entreprise ou écrire dans un bucket cloud d'équipe sont bloquées jusqu'à ce que vous les ajoutiez à `autoMode.environment`.

Pour savoir comment activer le mode auto et ce qu'il bloque par défaut, consultez [Modes de permission](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode). Cette page est la référence de configuration.

Cette page couvre comment :

* [Ajouter un point de contrôle humain](#common-boundaries) pour les poussées et les demandes de tirage avec `permissions.ask`
* [Choisir où définir les règles](#where-the-classifier-reads-configuration) dans CLAUDE.md, les paramètres utilisateur et les paramètres gérés
* [Définir l'infrastructure approuvée](#define-trusted-infrastructure) avec `autoMode.environment`
* [Remplacer les règles de blocage et d'autorisation](#override-the-block-and-allow-rules) quand les valeurs par défaut ne correspondent pas à votre pipeline
* [Acheminer toutes les commandes shell via le classificateur](#route-all-shell-commands-through-the-classifier) avec `autoMode.classifyAllShell`
* [Inspectez votre configuration effective](#inspect-the-defaults-and-your-effective-config) avec les sous-commandes `claude auto-mode`
* [Examinez les refus](#review-denials) pour savoir ce qu'il faut ajouter ensuite

<h2 id="common-boundaries">
  Limites communes
</h2>

Le mode auto permet les poussées vers votre branche de travail, les poussées régulières vers la branche par défaut du référentiel et la création de demandes de tirage par défaut. Le classificateur bloque une poussée uniquement lorsqu'elle présente un risque, comme une poussée forcée ou un contenu qui contourne un examen que vous avez configuré. Si vous souhaitez un point de contrôle humain avant chaque poussée ou demande de tirage, ajoutez des règles de permission : les recettes ci-dessous maintiennent le mode auto activé pour tout le reste.

Le mécanisme le plus direct est [`permissions.ask`](/docs/fr/permissions#permission-rule-syntax). Les règles ask délimitées par le contenu comme celles ci-dessous sont évaluées avant le classificateur et forcent toujours une invite de permission, même en mode auto, car une règle ask explicite est votre intention déclarée d'être invité pour cette action. Ajoutez les règles dans vos [paramètres](/docs/fr/settings#settings-files) :

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

Choisissez le mécanisme qui correspond à la fermeté requise de la limite :

| Limite                               | Mécanisme                                                                          | Comportement en mode auto                                                                                                                                                                                                                         |
| :----------------------------------- | :--------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Inviter avant l'action               | `permissions.ask`                                                                  | Invite toujours pour les règles délimitées par le contenu comme la recette ci-dessus. Le classificateur ne peut pas approuver automatiquement une action correspondante.                                                                          |
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
  Définir l'infrastructure approuvée
</h2>

Pour la plupart des organisations, `autoMode.environment` est le seul champ que vous devez définir. Il indique au classificateur quels dépôts, buckets et domaines sont approuvés : le classificateur l'utilise pour décider ce que signifie « externe », donc toute destination non listée est une cible d'exfiltration potentielle.

À partir de Claude Code v2.1.198, `claude auto-mode defaults` imprime trois types d'entrées d'environnement. Les versions antérieures à v2.1.195 impriment uniquement les cinq premiers emplacements de confiance.

* **Emplacements de contexte** : décrivent votre organisation, votre pile technologique et votre posture de sécurité afin que le classificateur lise les autres règles dans votre contexte. Contrairement aux deux autres types, les emplacements de contexte n'ont pas de règles propres qui les ciblent. Chacun est par défaut `Aucun configuré` ou à l'hypothèse conservatrice nommée à côté :
  * **Organisation**
  * **Utilisation principale de Claude Code** : par défaut développement logiciel
  * **Fournisseur(s) cloud**
  * **Visibilité du dépôt** : un dépôt est supposé privé sauf si son hôte distant et son nom l'indiquent autrement, ou une vérification de visibilité antérieure dans la conversation que le classificateur lit montre qu'il est public. Le classificateur lit vos messages et les commandes que Claude exécute, pas leur sortie, donc la preuve doit être quelque chose qu'il peut lire, comme votre propre message nommant le dépôt comme public ; la sortie d'un `gh repo view` seul ne l'atteint pas. La vérification des preuves de transcription nécessite Claude Code v2.1.200 ou ultérieur
  * **Partage interne / hébergement d'extraits** : les services publics de paste et gist sont traités comme en dehors de la limite de confiance jusqu'à ce que vous en nomiez un
  * **CLI spécifiques à l'organisation**
  * **Gestion des secrets**
  * **Branches par défaut / protégées** : `main` et `master` sont traités comme protégés jusqu'à ce que vous en nomiez d'autres
  * **Cibles de déploiement CI/CD**
  * **Posture réseau**
  * **Espaces de noms / environnements de déploiement protégés** : revient à l'heuristique des cibles distantes sensibles jusqu'à ce que vous les nomiez
  * **Rétention des données / déclassification**
* **Emplacements de confiance** : nomment ce que le classificateur traite comme à l'intérieur de votre limite. Les emplacements sont Dépôt approuvé, Contrôle de source, Domaines internes approuvés, Buckets cloud approuvés, Services internes clés et Registre de packages interne. Les entrées de dépôt et de contrôle de source sont par défaut le dépôt de travail et ses remotes configurées. Tous les autres emplacements de confiance sont par défaut `Aucun configuré`, donc rien d'autre n'est approuvé jusqu'à ce que vous l'ajoutiez. La visibilité d'un dépôt ne s'applique qu'au matériel confidentiel : un dépôt privé est une destination acceptable pour le matériel confidentiel, mais rendre un dépôt privé ne supprime jamais les secrets ou les données personnelles ou confiées qu'il contient, et le classificateur traite le contenu porté, réorienté ou d'abord lu de l'extérieur du dépôt de travail comme n'étant pas le travail propre de ce dépôt. Cette portée nécessite Claude Code v2.1.203 ou ultérieur.
* **Emplacements de sensibilité** : nomment ce que les règles de protection traitent comme à haut risque. Les emplacements sont Emplacements de données sensibles et audiences, Cibles distantes sensibles et Portées IaC protégées. Chacun est par défaut une heuristique large, comme traiter tout hôte ou espace de noms dont le nom porte `prod` ou `production` comme une cible distante sensible, donc les règles de protection sont actives avant que vous configuriez quoi que ce soit. Nommer des cibles concrètes dans un emplacement de sensibilité fait que ces règles s'appliquent aux cibles nommées au lieu de l'heuristique.

Pour ajouter vos propres entrées aux côtés des valeurs par défaut, incluez la chaîne littérale `"$defaults"` dans le tableau. Les entrées par défaut sont insérées à cette position, donc vos entrées personnalisées peuvent aller avant ou après elles.

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

Les entrées sont en prose, pas en regex ou en motifs d'outil. Le classificateur les lit comme des règles en langage naturel. Écrivez-les comme vous décririez votre infrastructure à un nouvel ingénieur. Une section d'environnement approfondie couvre :

* **Organisation** : le nom de votre entreprise et ce pour quoi Claude Code est principalement utilisé, comme le développement logiciel, l'automatisation de l'infrastructure ou l'ingénierie des données
* **Contrôle de source** : chaque organisation GitHub, GitLab ou Bitbucket vers laquelle vos développeurs poussent
* **Fournisseurs cloud et buckets approuvés** : noms de buckets ou préfixes que Claude devrait pouvoir lire et écrire
* **Domaines internes approuvés** : noms d'hôtes pour les API, tableaux de bord et services à l'intérieur de votre réseau, comme `*.internal.example.com`
* **Services internes clés** : CI, registres d'artefacts, index de packages internes, outils d'incident
* **Registre de packages interne** : le registre npm, PyPI ou autre privé par lequel les installations doivent être acheminées, donc les installations qui le contournent pour un registre public sont bloquées
* **Emplacements de données sensibles et audiences** : les buckets, bases de données ou chemins qui contiennent des données personnelles, des données commerciales confidentielles, des identifiants, des données réglementées ou du matériel similaire sensible, et les audiences avec lesquelles les données de chaque emplacement peuvent être partagées, afin que le classificateur protège ces emplacements au lieu de deviner à partir du contenu. Claude Code v2.1.195 à v2.1.197 nomme cette entrée Emplacements de données PII / réglementées et couvre uniquement les emplacements qui contiennent des données personnelles ou réglementées, sans la dimension d'audience
* **Cibles distantes sensibles** : les espaces de noms, hôtes ou conteneurs qui comptent comme production, donc les shells distants et les redirections de ports vers eux ont besoin de votre approbation explicite
* **Portées IaC protégées** : les ressources d'infrastructure dont l'application ou la destruction doivent toujours vous demander de nommer le changement
* **Contexte supplémentaire** : contraintes d'industrie réglementée, infrastructure multi-locataire ou exigences de conformité qui affectent ce que le classificateur devrait traiter comme risqué

Les entrées Registre de packages interne, Emplacements de données sensibles et audiences, Cibles distantes sensibles et Portées IaC protégées nécessitent Claude Code v2.1.195 ou ultérieur. Les versions antérieures les lisent toujours comme du contexte simple mais n'ont pas les règles intégrées qui les ciblent.

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

Plus le contexte que vous fournissez est spécifique, mieux le classificateur peut distinguer les opérations internes courantes des tentatives d'exfiltration.

Vous n'avez pas besoin de tout remplir à la fois. Un déploiement raisonnable : commencez par les valeurs par défaut et ajoutez votre organisation de contrôle de source et vos services internes clés, ce qui résout les faux positifs les plus courants comme pousser vers vos propres dépôts. Ajoutez ensuite les domaines approuvés et les buckets cloud. Remplissez le reste à mesure que les blocages surviennent.

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

<h2 id="route-all-shell-commands-through-the-classifier">
  Router tous les commandes shell via le classificateur
</h2>

Par défaut, les règles d'autorisation Bash et PowerShell étroites comme `Bash(npm test)` sont reportées en mode auto et résolues avant l'exécution du classificateur. Le mode auto suspend uniquement les règles larges qui accordent l'exécution de code arbitraire, comme `Bash(*)` ou les interpréteurs avec caractères génériques. Cela signifie qu'une règle étroite peut toujours laisser passer un argument destructeur sans que le classificateur le voie, par exemple un chemin de script ou un drapeau que le préfixe de la règle n'a pas anticipé.

Définissez `autoMode.classifyAllShell` à `true` pour suspendre chaque règle d'autorisation Bash et PowerShell pendant que le mode auto est actif, afin que le classificateur évalue chaque commande shell indépendamment de votre liste d'autorisation.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Cela échange la latence pour la couverture : une commande qu'une règle d'autorisation aurait approuvée instantanément attend maintenant une décision du classificateur, et chaque commande shell compte comme un appel du classificateur.

Le paramètre s'applique uniquement pendant que le mode auto est actif, et vos règles d'autorisation se comportent normalement dans les autres modes de permission.

<Note>
  `autoMode.classifyAllShell` nécessite Claude Code v2.1.193 ou ultérieur. Les versions antérieures ignorent la clé et continuent à reporter les règles d'autorisation shell étroites en mode auto.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Inspecter les valeurs par défaut et votre configuration effective
</h2>

Trois sous-commandes CLI vous aident à inspecter et valider votre configuration.

Imprimez les règles `environment`, `allow`, `soft_deny` et `hard_deny` intégrées en JSON :

```bash theme={null}
claude auto-mode defaults
```

Pour lire le libellé complet d'une règle sans passer par `jq`, passez `--label` avec le début du libellé de la règle, tel que `claude auto-mode defaults --label 'Git Destructive'`. La correspondance est un préfixe insensible à la casse sur le libellé de chaque règle, et les sections sans correspondance s'affichent comme des listes vides. Nécessite Claude Code v2.1.208 ou version ultérieure.

Imprimez ce que le classificateur utilise réellement en JSON, avec vos paramètres appliqués où définis et les valeurs par défaut sinon :

```bash theme={null}
claude auto-mode config
```

Obtenez des commentaires IA sur vos règles `allow`, `soft_deny` et `hard_deny` personnalisées :

```bash theme={null}
claude auto-mode critique
```

Exécutez `claude auto-mode config` après avoir enregistré vos paramètres pour confirmer que les règles effectives sont ce que vous attendez, avec `"$defaults"` développé en place. Si vous avez écrit des règles personnalisées, `claude auto-mode critique` les examine et signale les entrées qui sont ambiguës, redondantes ou susceptibles de causer des faux positifs.

Si vous devez supprimer ou réécrire une règle intégrée plutôt que d'en ajouter une à côté, enregistrez la sortie de `claude auto-mode defaults` dans un fichier, modifiez les listes, et collez le résultat dans votre fichier de paramètres à la place de `"$defaults"`.

<h2 id="review-denials">
  Examiner les refus
</h2>

Quand le mode auto refuse un appel d'outil, le refus est enregistré dans `/permissions` sous l'onglet Récemment refusé. Appuyez sur `r` sur une action refusée pour la marquer pour réessai : quand vous quittez la boîte de dialogue, Claude Code envoie un message indiquant au modèle qu'il peut réessayer cet appel d'outil et reprend la conversation.

Dans Claude Code v2.1.193 et ultérieur, la raison du classificateur pour chaque refus apparaît aux côtés de l'appel d'outil bloqué dans la transcription, dans la notification de refus et sous chaque entrée de l'onglet Récemment refusé. Utilisez la raison pour décider si la correction est une entrée `environment`, une exception `allow` ou un réessai avec intention explicite dans votre prochain message.

Les refus répétés pour la même destination signifient généralement que le classificateur manque de contexte. Ajoutez cette destination à `autoMode.environment`, puis exécutez `claude auto-mode config` pour confirmer que cela a pris effet.

Pour réagir aux refus par programmation, utilisez le [hook `PermissionDenied`](/docs/fr/hooks#permissiondenied).

<h2 id="see-also">
  Voir aussi
</h2>

* [Modes de permission](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) : ce qu'est le mode auto, ce qu'il bloque par défaut et comment l'activer
* [Paramètres gérés](/docs/fr/server-managed-settings) : déployez la configuration `autoMode` dans votre organisation
* [Permissions](/docs/fr/permissions) : règles d'autorisation, de demande et de refus qui s'appliquent avant l'exécution du classificateur
* [Paramètres](/docs/fr/settings) : la référence complète des paramètres, y compris la clé `autoMode`
