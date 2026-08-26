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

À chaque requête `/v1/messages`, la passerelle résout les limites du développeur et les dépenses à ce jour de la période en une seule requête Postgres. S'il dépasse une limite, la requête retourne `429` avec `error.type: billing_error` et l'en-tête `x-should-retry: false`. Le message est `spend limit reached`, suivi de votre [`admin.blocked_message`](/docs/fr/claude-apps-gateway-config#admin) s'il est défini.

`/v1/messages/count_tokens` est exempté. Le comptage de tokens est gratuit, il s'exécute donc indépendamment de l'état de la limite.

Après chaque réponse, un compteur d'utilisation lit les nombres de tokens de la réponse au fur et à mesure qu'elle est diffusée au client, les évalue au prix de liste USD et incrémente les compteurs Postgres pour les trois buckets de période. Le compteur est un lecteur unique sur le flux, de sorte que les octets du client ne sont pas modifiés et une défaillance de mesure ne casse pas la réponse.

Les limites de dépenses estiment les dépenses à partir des nombres de tokens au prix de liste USD ; c'est un disjoncteur, pas une facture. Pour une facturation faisant autorité, rapprochez-vous de la déclaration d'utilisation propre de votre fournisseur, comme l'API Admin Utilisation & Coûts d'Anthropic, les journaux d'invocation sur Bedrock, ou Cloud Monitoring sur Google Cloud.

La tarification utilise la même table que Claude Code CLI utilise pour son propre affichage des coûts, avec la même canonicalisation d'ID de modèle sur les formes Anthropic, Bedrock (`us.anthropic.…-v1:0`), Agent Platform (`claude-…@date`) et Foundry ID. Un ID de modèle que la table ne peut pas placer, comme un nom de déploiement Foundry ou un ARN de profil d'inférence, est tarifé au niveau de tier par défaut de modèle inconnu de 5 $/25 $ par million de tokens d'entrée/sortie plutôt que zéro, de sorte qu'un ID non reconnu ne peut pas contourner une limite en restant non mesuré. La passerelle avertit au démarrage et une fois par ID à l'exécution lorsqu'un modèle est tarifé via le fallback.

Les abandons de clients sont également facturés. L'amont rapporte les tokens de sortie uniquement dans la trame terminale du flux, de sorte qu'un flux abandonné ne les porte pas. Le compteur maintient une estimation de plancher conservatrice à partir de la taille du contenu diffusé, environ quatre caractères par token, et la facture quand et seulement quand la trame d'utilisation terminale est manquante. Un flux complet facture toujours le nombre rapporté en amont. Sans cela, un développeur limité pourrait diffuser la sortie et abandonner chaque requête immédiatement avant la fin, dépensant sans jamais être compté.

<h3 id="postgres-availability">
  Disponibilité de Postgres
</h3>

La pré-vérification interroge Postgres avec un délai d'expiration de deux secondes. Si le magasin est inaccessible ou expire, l'application échoue ouvertement par défaut : la requête procède et la passerelle enregistre un avertissement. Définissez [`enforcement.fail_closed_on_error: true`](/docs/fr/claude-apps-gateway-config#enforcement) pour échouer fermé à la place, ce qui retourne le même `429 billing_error` avec le message `spend limit unavailable`. L'échec ouvert empêche une panne de magasin de devenir une panne d'inférence ; l'échec fermé garantit aucune dépense non mesurée.

<h2 id="admin-api-reference">
  Référence de l'API Admin
</h2>

Les points de terminaison ci-dessous sont servis sous `/v1/organizations/spend_limits`.

| Méthode et chemin                              | Description                                                               |
| ---------------------------------------------- | ------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | Lister les limites configurées. Requête : `?limit=&after_id=&before_id=`. |
| `POST /v1/organizations/spend_limits`          | Créer ou remplacer une limite pour `{scope, period}`.                     |
| `GET /v1/organizations/spend_limits/{id}`      | Récupérer une limite par son ID préfixé `spl_`.                           |
| `DELETE /v1/organizations/spend_limits/{id}`   | Supprimer une limite. Retourne `{type: "spend_limit_deleted", id}`.       |
| `GET /v1/organizations/spend_limits/effective` | Limite résolue et dépenses à ce jour par principal par période.           |
| `GET /v1/organizations/spend_limits/audit`     | Piste de mutation admin, plus récente en premier. Requête : `?limit=`.    |

Les conventions reflètent l'API Admin d'Anthropic :

* Un `type` sur chaque objet
* IDs préfixés `spl_`
* Montants sous forme de chaînes de nombre entier de cents USD ; `POST` rejette toute autre `currency` avec `400`
* L'enveloppe d'erreur `{type: "error", error: {type, message}, request_id}`
* Un en-tête de réponse `request-id` sur chaque réponse admin, succès ou erreur, correspondant au `request_id` du corps

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
| `limit` / `page`     | Taille de page (1–1000, par défaut 20) et le curseur opaque de la réponse précédente `next_page`.                   |

<Warning>
  `q=` et `user_ids[]=` utilisent les chaînes de requête GET, de sorte que tout proxy frontal ou équilibreur de charge les capture dans ses journaux d'accès. Si votre politique de journalisation des PII est stricte, nettoyez ces paramètres là-bas.
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

Retourne la piste de mutation de limite de dépenses : qui a changé quelle limite, les snapshots avant/après et la raison optionnelle, plus récente en premier. `has_more` est exact. Ce point de terminaison suit les conventions de l'API Admin locale plutôt qu'une forme de câblage de première partie.

<h3 id="pagination">
  Pagination
</h3>

La liste brute pagine par `after_id` et `before_id`, qui sont des IDs `spl_…` mutuellement exclusifs ; les résultats sont ordonnés par création et `has_more` reflète la direction de traversée. `/effective` pagine par le jeton opaque `next_page` repassé comme `?page=`, avec les principaux ordonnés en ordre croissant de sorte que les pages restent stables pendant que les dépenses sont enregistrées. `limit` est 1–1000, par défaut 20, sur les deux.

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

`identity_retention_days` est délibérément plus court que `spend_retention_months` : une identité déprovisionée cesse de se rafraîchir et vieillit, tandis que ses compteurs de dépenses anonymes restent pour les rapports d'année en année.

Lorsqu'un développeur part, supprimez toute limite par utilisateur via `DELETE /v1/organizations/spend_limits/{id}` ; ses lignes de dépenses et d'identité vieillissent sur les fenêtres de rétention ci-dessus. Pour effacer une personne immédiatement, pour l'offboarding ou une demande d'accès aux données (DSAR), exécutez `DELETE FROM principal_emails WHERE principal = '<sub>'` directement contre la base de données de la passerelle. Cela supprime la seule table contenant leur email, nom et groupes. Les lignes `spend` et `admin_audit` font référence uniquement au pseudonyme OIDC `sub` et vieillissent sur leurs propres fenêtres.

<h2 id="related">
  Connexes
</h2>

* [Configuration `admin` et `enforcement`](/docs/fr/claude-apps-gateway-config#admin) : activation de l'API admin et ajustement de la rétention
* [Guide de déploiement](/docs/fr/claude-apps-gateway-deploy#postgres) : schéma Postgres et conseils de sauvegarde
