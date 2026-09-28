> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Limites de dépenses de la passerelle Claude apps

> Limitez les dépenses de chaque développeur via la passerelle Claude apps par jour, semaine ou mois. Définissez les limites avec une API Admin et la passerelle les applique en direct à chaque requête.

Les limites de dépenses limitent le montant que chaque développeur peut dépenser via votre [passerelle Claude apps](/docs/fr/claude-apps-gateway) au cours d'un jour, d'une semaine ou d'un mois donné. Lorsqu'un développeur dépasse sa limite, la passerelle retourne `429` à sa prochaine requête et le bloque jusqu'à ce que la période se réinitialise ou qu'un administrateur augmente la limite. Utilisez les limites de dépenses pour donner à chaque développeur, groupe ou l'ensemble de l'organisation un plafond sur une credential que tout le monde partage.

Une passerelle Claude apps transfère toutes les inférences via une credential amont partagée, de sorte que la facture de votre fournisseur attribue tout à cette credential, et non aux développeurs individuels. Sans limites par développeur, une flotte d'agents incontrôlée peut dépenser l'engagement entier de l'organisation. Les limites de dépenses constituent la vue par développeur de la passerelle et le disjoncteur sur cette facture partagée.

<h2 id="set-a-cap">
  Définir une limite
</h2>

Avec le bloc [`admin:`](/docs/fr/claude-apps-gateway-config#admin) configuré dans `gateway.yaml`, la passerelle sert une API admin à `/v1/organizations/spend_limits` et applique les limites en direct à chaque requête d'inférence. Les limites elles-mêmes sont définies via cette API, pas dans `gateway.yaml` ; chaque requête `POST /v1/organizations/spend_limits` crée ou remplace une limite à partir de `{scope, amount, period}`. L'API reflète les formes de câblage des points de terminaison de limites de dépenses de l'[API Admin](https://platform.claude.com/docs/en/manage-claude/admin-api) public d'Anthropic, de sorte qu'un client HTTP écrit selon ce contrat peut cibler la passerelle en changeant son URL de base.

Cette requête définit une limite par défaut à l'échelle de l'organisation de 500 \$ par mois pour chaque développeur :

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

Cette requête ajoute une limite plus stricte de 100 \$ par jour pour chaque membre du groupe `contractors` :

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| Champ        | Valeurs                                         | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------ | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scope.type` | `user`, `rbac_group`, `organization`            | `user` cible un développeur par son OpenID Connect (OIDC) `sub`, l'ID utilisateur stable que votre fournisseur d'identité attribue ; passez-le comme `scope.user_id`. `rbac_group` cible un [groupe IdP](/docs/fr/claude-apps-gateway-config#managed) par nom ; passez-le comme `scope.rbac_group_id`. `organization` est la limite par défaut à l'échelle de l'organisation. La passerelle accepte les trois ; le `POST` public d'Anthropic est actuellement réservé aux utilisateurs uniquement. |
| `amount`     | Chaîne de nombre entier de cents USD, ou `null` | `null` est illimité. `"0"` est une limite zéro, qui bloque chaque requête.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `period`     | `daily`, `weekly`, `monthly`                    | Une portée peut contenir une limite par période, et chacune s'applique indépendamment : un développeur est bloqué s'il dépasse l'une d'elles.                                                                                                                                                                                                                                                                                                                                                 |

Une limite de groupe ou d'organisation est une limite par siège par défaut que chaque membre hérite, pas un pool partagé. Par période, la limite effective d'un développeur se résout dans cet ordre : un remplacement par utilisateur, puis la plus restrictive de ses limites de groupe, puis la limite par défaut de l'organisation, puis illimitée. [`admin.group_limit_mode: max`](/docs/fr/claude-apps-gateway-config#admin) bascule le départage multi-groupe vers la moins restrictive à la place.

<h3 id="authenticate-to-the-admin-api">
  S'authentifier auprès de l'API admin
</h3>

Envoyez l'un des éléments suivants :

* Un en-tête `x-api-key` correspondant à une clé dans [`admin.write_keys`](/docs/fr/claude-apps-gateway-config#admin) pour un accès complet, ou `admin.read_keys` pour un accès en lecture seule avec `GET`. Chaque clé porte un `id` qui apparaît dans le journal d'audit comme `admin-key:<id>`, donc donnez à Terraform, CI et à chaque automatisation la sienne.
* Un jeton bearer de passerelle dont la revendication `groups` inclut l'un des [`admin.admin_groups`](/docs/fr/claude-apps-gateway-config#admin). C'est un accès complet et s'audite comme `oidc:<sub>`, donc préférez-le pour les administrateurs humains.

<h2 id="how-enforcement-works">
  Comment l'application fonctionne
</h2>

À chaque requête `/v1/messages`, la passerelle résout les limites du développeur et les dépenses à ce jour de la période en une seule requête Postgres. S'il dépasse une limite, la requête retourne `429` avec `error.type: billing_error` et l'en-tête `x-should-retry: false`.

Le message nomme la période et l'heure de réinitialisation, par exemple `spend limit reached (daily; resets 2026-08-08 00:00 UTC)`, suivi de votre [`admin.blocked_message`](/docs/fr/claude-apps-gateway-config#admin) s'il est défini. Quand un développeur dépasse plusieurs limites à la fois, le message nomme la limite qui se réinitialise en dernier. La réponse porte également un en-tête `retry-after` avec les secondes restantes jusqu'à cette réinitialisation. Avant v2.1.225 sur le serveur de la passerelle, le message était `spend limit reached` sans période, heure de réinitialisation, ou en-tête `retry-after`.

Sur v2.1.227 ou ultérieur, la référence de protocole à `<public_url>/protocol` liste également les en-têtes de réponse de limite d'utilisation exacts et le corps `429`.

Les limites se réinitialisent sur les limites du calendrier UTC : quotidiennement à 00:00 UTC, hebdomadairement le lundi, et mensuellement le premier. La passerelle ne bloque jamais `/v1/messages/count_tokens`, car le comptage de tokens est gratuit.

<h3 id="how-requests-are-priced">
  Comment les requêtes sont tarifées
</h3>

Après chaque réponse, un compteur d'utilisation lit les nombres de tokens et ajoute le coût aux compteurs quotidiens, hebdomadaires et mensuels. Il ne touche jamais aux octets envoyés au client, de sorte qu'une défaillance de mesure ne peut pas casser une réponse. Les montants sont des estimations en USD, un disjoncteur plutôt qu'une facture ; pour la facturation, rapprochez-vous de la déclaration d'utilisation de votre fournisseur.

Le compteur choisit les tarifs de chaque requête dans cet ordre :

1. Une ligne [`pricing.overrides`](/docs/fr/claude-apps-gateway-config#pricing) correspondante pour l'amont qui a servi la requête. Nécessite v2.1.227 ou ultérieur.
2. Tarif de liste pour l'ID de modèle amont, la chaîne que la passerelle envoie au fournisseur, quand la table de coûts Claude Code la reconnaît. La table accepte les formes Anthropic, Amazon Bedrock, Google Cloud's Agent Platform, et Microsoft Foundry ID.
3. Tarif de liste pour le [`models[].id`](/docs/fr/claude-apps-gateway-config#models) que vous avez mappé à cet ID amont, pour les chaînes amont qui ne portent pas de nom de modèle, comme un ARN de profil d'inférence d'application Amazon Bedrock ou un nom de déploiement Microsoft Foundry. Nécessite v2.1.218 ou ultérieur.
4. Le tier de modèle inconnu de 5 $/25 $ par million de tokens d'entrée/sortie, de sorte qu'un ID que le compteur ne peut pas placer n'est jamais gratuit. La passerelle avertit au démarrage et une fois par ID à l'exécution quand elle utilise ce tier.

Quel que soit le tarif applicable, le compteur multiplie ensuite le montant par [`pricing.multiplier`](/docs/fr/claude-apps-gateway-config#pricing), par défaut `1`.

Les abandons de clients sont également facturés. Quand un flux se termine sans la trame d'utilisation finale de l'amont, le compteur facture une estimation de plancher d'environ quatre caractères par token de sortie pour le texte déjà envoyé au client, de sorte que l'abandon de requêtes tôt ne contourne pas une limite.

<h3 id="postgres-availability">
  Disponibilité de Postgres
</h3>

La pré-vérification interroge Postgres avec un délai d'expiration de deux secondes. Si le magasin est inaccessible ou expire, l'application échoue ouvertement par défaut : la requête procède, la passerelle enregistre un avertissement, et la réponse ne porte pas d'en-têtes `anthropic-ratelimit-unified-*`. Définissez [`enforcement.fail_closed_on_error: true`](/docs/fr/claude-apps-gateway-config#enforcement) pour échouer fermé à la place, ce qui retourne le même `429 billing_error` mais avec le message `spend limit unavailable` et sans période, heure de réinitialisation, ou en-tête `retry-after`. L'échec ouvert empêche une panne de magasin de devenir une panne d'inférence ; l'échec fermé garantit aucune dépense non mesurée.

<h3 id="usage-warnings-in-claude-code">
  Avertissements d'utilisation dans Claude Code
</h3>

Claude Code avertit un développeur à l'approche de sa limite : une fois que l'utilisation dépasse 75 %, et à nouveau au-delà de 95 % de sa limite la plus consommée. Quand la passerelle bloque une requête, Claude Code affiche le message `429` de la passerelle tel quel, y compris votre `admin.blocked_message`.

L'avertissement fonctionne à partir des en-têtes de réponse :

* Avec v2.1.225 ou ultérieur sur le serveur de la passerelle, chaque réponse `/v1/messages` réussie pour un développeur qui a une limite porte son propre taux d'utilisation de limite et l'heure de réinitialisation dans les en-têtes `anthropic-ratelimit-unified-*`.
* Avec v2.1.225 ou ultérieur sur la machine du développeur également, Claude Code lit les en-têtes et affiche l'avertissement.

Les en-têtes décrivent toujours la limite propre du développeur : la passerelle supprime les en-têtes de limite de débit du fournisseur amont, qui décrivent votre quota partagé, et ne les transmet jamais.

Avec v2.1.251 ou ultérieur sur la machine du développeur, Claude Code lit également les mêmes en-têtes pour afficher une barre **Spend limit** dans `/usage`, avec le pourcentage de leur limite utilisée et quand elle se réinitialise, et pour ajouter un objet `rate_limits.spend_limit` à la [ligne de statut](/docs/fr/statusline#rate-limit-usage) d'entrée. Claude Code affiche les deux en tant que pourcentage plutôt qu'un montant en dollars, et n'a besoin de rien de plus récent que v2.1.225 sur le serveur de la passerelle.

<h2 id="admin-api-reference">
  Référence de l'API Admin
</h2>

Les points de terminaison ci-dessous sont servis sous `/v1/organizations/spend_limits`.

| Méthode et chemin                              | Description                                                                                                                                                                 |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | Lister les limites configurées, optionnellement filtrées à un `scope_type` de `organization`, `rbac_group` ou `user`. Requête : `?limit=&after_id=&before_id=&scope_type=`. |
| `POST /v1/organizations/spend_limits`          | Créer ou remplacer une limite pour `{scope, period}`.                                                                                                                       |
| `GET /v1/organizations/spend_limits/{id}`      | Récupérer une limite par son ID préfixé `spl_`.                                                                                                                             |
| `DELETE /v1/organizations/spend_limits/{id}`   | Supprimer une limite. Retourne `{type: "spend_limit_deleted", id}`.                                                                                                         |
| `GET /v1/organizations/spend_limits/effective` | Limite résolue et dépenses à ce jour par principal par période.                                                                                                             |
| `GET /v1/organizations/spend_limits/audit`     | Piste de mutation admin, plus récente en premier. Requête : `?limit=&after_id=`.                                                                                            |

Les conventions reflètent l'API Admin d'Anthropic :

* Un `type` sur chaque objet
* IDs préfixés `spl_`
* Montants sous forme de chaînes de nombre entier de cents USD ; `POST` rejette toute autre `currency` avec `400`
* L'enveloppe d'erreur `{type: "error", error: {type, message}, request_id}`
* Un en-tête de réponse `request-id` sur chaque réponse admin, succès ou erreur ; les corps d'erreur le portent également en tant que `request_id`

Chaque mutation écrit une ligne avant/après à `admin_audit` dans la même transaction, attribuée à `admin-key:<id>` ou `oidc:<sub>`.

La passerelle sert les points de terminaison des limites de dépenses uniquement. Les autres surfaces de l'API Admin, telles que la file d'attente `spend_limit_increase_requests`, ne font pas partie de l'API admin de la passerelle.

<h3 id="/effective">
  `/effective`
</h3>

`GET /v1/organizations/spend_limits/effective` retourne le schéma `SpendSummary` d'Anthropic : chaque ligne est un principal pour une période, avec la limite résolue, les dépenses à ce jour de la période et un objet `actor`. Différences spécifiques à la passerelle :

* `user_id` est l'OIDC `sub`.
* `actor.name` et `actor.email_address` sont `null` jusqu'à la première requête d'inférence du principal via la passerelle. La passerelle n'a pas de répertoire utilisateur ; elle enregistre les valeurs dernièrement vues à partir du JWT de session de chaque utilisateur.
* Chaque ligne porte également un tableau `groups`, les groupes IdP dernièrement vus du principal. C'est une extension de passerelle pour qu'une interface utilisateur admin puisse afficher chaque niveau de limite qui s'applique ; les clients façonnés par Anthropic l'ignorent.
* Sans un filtre `user_ids[]`, il liste les principaux avec des dépenses enregistrées, car la passerelle ne peut pas énumérer tous les membres de l'organisation.

Les limites sourced par groupe se résolvent contre ces groupes dernièrement vus avec le même départage `group_limit_mode` que l'application utilise, de sorte que la visionneuse affiche la limite qui s'applique réellement.

| Paramètre de requête | Description                                                                                                         |
| -------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `user_ids[]`         | Répétable. Filtrer vers des principaux spécifiques par OIDC `sub`.                                                  |
| `period[]`           | Répétable. Filtrer vers les lignes `daily`, `weekly` ou `monthly`.                                                  |
| `sort`               | `spend_desc` liste les plus gros dépensiers en premier. Nécessite exactement un `period[]`.                         |
| `q`                  | Filtre de sous-chaîne insensible à la casse sur l'OIDC `sub`, le dernier email vu et le dernier nom d'affichage vu. |
| `limit` / `page`     | Taille de page, 1–1000 avec un défaut de 20, et le curseur opaque de la réponse précédente `next_page`.             |

<Warning>
  `q=` et `user_ids[]=` utilisent les chaînes de requête GET, de sorte que tout proxy frontal ou équilibreur de charge les capture dans ses journaux d'accès. Si votre politique de journalisation des PII est stricte, nettoyez ces paramètres là-bas.
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

Retourne la piste de mutation de limite de dépenses : qui a changé quelle limite, avec les snapshots avant/après, plus récente en premier. `has_more` est exact. Ce point de terminaison suit les conventions de l'API Admin locale plutôt qu'une forme de câblage de première partie.

<h3 id="pagination">
  Pagination
</h3>

La liste brute pagine par `after_id` et `before_id`, qui sont des IDs `spl_…` mutuellement exclusifs ; les résultats sont ordonnés par création et `has_more` reflète la direction de traversée. `/effective` pagine par le jeton opaque `next_page` repassé comme `?page=`, avec les principaux ordonnés en ordre croissant de sorte que les pages restent stables pendant que les dépenses sont enregistrées. `limit` est 1–1000, par défaut 20, sur les deux. `/audit` pagine par `after_id`, l'ID numérique `id` du dernier événement sur la page précédente, et sa `limit` par défaut est 100.

<h2 id="data-lifecycle">
  Cycle de vie des données
</h2>

La passerelle contient quatre tables liées aux dépenses ; un balayage horaire applique les fenêtres de rétention :

| Table              | Contenu                                                                                           | Rétention                                                                                                          |
| ------------------ | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `spend`            | Compteurs à ce jour de la période par principal en cents                                          | [`admin.spend_retention_months`](/docs/fr/claude-apps-gateway-config#admin), par défaut 13                              |
| `spend_limits`     | Les limites configurées                                                                           | Jusqu'à suppression via l'API                                                                                      |
| `admin_audit`      | La piste de mutation                                                                              | [`admin.audit_retention_days`](/docs/fr/claude-apps-gateway-config#admin), par défaut 365                               |
| `principal_emails` | Le dernier email vu de chaque principal, le nom d'affichage et les groupes IdP. Contient des PII. | [`admin.identity_retention_days`](/docs/fr/claude-apps-gateway-config#admin) depuis la dernière activité, par défaut 90 |

Lorsqu'un développeur part, supprimez toute limite par utilisateur via `DELETE /v1/organizations/spend_limits/{id}` ; ses lignes de dépenses et d'identité vieillissent sur les fenêtres de rétention ci-dessus. Pour effacer une personne immédiatement, pour l'offboarding ou une demande d'accès aux données (DSAR), exécutez `DELETE FROM principal_emails WHERE principal = '<sub>'` directement contre la base de données de la passerelle. Cela supprime la seule table contenant leur email, nom et groupes. Les lignes `spend` et `admin_audit` font référence uniquement au pseudonyme OIDC `sub` et vieillissent sur leurs propres fenêtres.

<h2 id="related">
  Connexes
</h2>

* [Configuration `admin` et `enforcement`](/docs/fr/claude-apps-gateway-config#admin) : activation de l'API admin et ajustement de la rétention
* [Guide de déploiement](/docs/fr/claude-apps-gateway-deploy#postgres) : schéma Postgres et conseils de sauvegarde
