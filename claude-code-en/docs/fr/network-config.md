> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuration réseau d'entreprise

> Configurez Claude Code pour les environnements d'entreprise avec des serveurs proxy, des autorités de certification (CA) personnalisées et l'authentification mutuelle Transport Layer Security (mTLS).

Claude Code prend en charge diverses configurations réseau et de sécurité d'entreprise via des variables d'environnement. Cela inclut le routage du trafic via des serveurs proxy d'entreprise, la confiance envers des autorités de certification (CA) personnalisées et l'authentification avec des certificats mTLS (Transport Layer Security mutuel) pour une sécurité renforcée.

<Note>
  Toutes les variables d'environnement affichées sur cette page peuvent également être configurées dans [`settings.json`](/docs/fr/settings).
</Note>

<h2 id="proxy-configuration">
  Configuration du proxy
</h2>

<h3 id="environment-variables">
  Variables d'environnement
</h3>

Claude Code respecte les variables d'environnement proxy standard :

```bash theme={null}
# Proxy HTTPS (recommandé)
export HTTPS_PROXY=https://proxy.example.com:8080

# Proxy HTTP (si HTTPS non disponible)
export HTTP_PROXY=http://proxy.example.com:8080

# Contourner le proxy pour des requêtes spécifiques - format séparé par des espaces
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Contourner le proxy pour des requêtes spécifiques - format séparé par des virgules
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Contourner le proxy pour toutes les requêtes
export NO_PROXY="*"
```

<Note>
  Claude Code ne prend pas en charge les proxies SOCKS.
</Note>

<h3 id="basic-authentication">
  Authentification basique
</h3>

Si votre proxy nécessite une authentification basique, incluez les identifiants dans l'URL du proxy :

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Évitez de coder en dur les mots de passe dans les scripts. Utilisez plutôt des variables d'environnement ou un stockage sécurisé des identifiants.
</Warning>

<Tip>
  Pour les proxies nécessitant une authentification avancée (NTLM, Kerberos, etc.), envisagez d'utiliser un service LLM Gateway qui prend en charge votre méthode d'authentification.
</Tip>

<h2 id="ca-certificate-store">
  Magasin de certificats CA
</h2>

Par défaut, Claude Code fait confiance à la fois aux certificats CA Mozilla fournis avec le produit et au magasin de certificats de votre système d'exploitation. La lecture du magasin du système d'exploitation nécessite un runtime avec `tls.getCACertificates` : l'installateur natif l'a toujours, et les installations npm nécessitent Node 22.15 ou une version ultérieure. Sur les versions plus anciennes de Node, seul l'ensemble fourni et `NODE_EXTRA_CA_CERTS` s'appliquent. Les proxies d'inspection TLS d'entreprise tels que CrowdStrike Falcon et Zscaler fonctionnent sans configuration supplémentaire lorsque leur certificat racine est installé dans le magasin de confiance du système d'exploitation et que le runtime peut le lire.

`CLAUDE_CODE_CERT_STORE` accepte une liste séparée par des virgules de sources. Les valeurs reconnues sont `bundled` pour l'ensemble de certificats CA Mozilla fourni avec Claude Code et `system` pour le magasin de confiance du système d'exploitation. La valeur par défaut est `bundled,system`.

Pour approuver uniquement l'ensemble de certificats CA Mozilla fourni :

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Pour approuver uniquement le magasin de certificats du système d'exploitation :

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` n'a pas de clé de schéma dédiée dans `settings.json`. Définissez-la via le bloc `env` dans `~/.claude/settings.json` ou directement dans l'environnement du processus.
</Note>

<h2 id="custom-ca-certificates">
  Certificats CA personnalisés
</h2>

Si votre environnement d'entreprise utilise une CA personnalisée, configurez Claude Code pour la faire confiance directement :

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Authentification mTLS
</h2>

Pour les environnements d'entreprise nécessitant une authentification par certificat client :

```bash theme={null}
# Certificat client pour l'authentification
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Clé privée du client
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Optionnel : phrase de passe pour la clé privée chiffrée
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code lit les fichiers de certificat et de clé au démarrage et les relit chaque fois qu'il applique les paramètres, y compris lorsque les paramètres changent au cours d'une session. Pour faire tourner le certificat et la clé, remplacez les fichiers aux mêmes chemins.

<h2 id="network-access-requirements">
  Exigences d'accès réseau
</h2>

Claude Code nécessite l'accès aux URL suivantes. Autorisez ces URL dans votre configuration proxy et vos règles de pare-feu, en particulier dans les environnements réseau conteneurisés ou restreints.

| URL                            | Requis pour                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`            | Requêtes de l'API Claude                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `claude.ai`                    | Authentification du compte claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `platform.claude.com`          | Authentification du compte Anthropic Console                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `mcp-proxy.anthropic.com`      | [Connecteurs MCP depuis claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai), y compris les connecteurs qu'un administrateur d'organisation configure. Le trafic des connecteurs est acheminé via ce proxy ; les connecteurs sont activés par défaut pour les utilisateurs authentifiés par claude.ai. Pour désactiver, définissez [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/fr/env-vars) ou le paramètre [`disableClaudeAiConnectors`](/docs/fr/settings#available-settings) |
| `downloads.claude.ai`          | Téléchargements d'exécutables de plugins ; installateur natif et mise à jour automatique native                                                                                                                                                                                                                                                                                                                                                                     |
| `storage.googleapis.com`       | Compteurs d'installation et métadonnées de plugins affichés dans `/plugin`. Les téléchargements d'[artifacts](/docs/fr/artifacts) signés essaient d'abord cet hôte ; la publication revient à `api.anthropic.com` lorsqu'il est bloqué                                                                                                                                                                                                                                   |
| `storage.googleapis.com`       | Installateur natif et mise à jour automatique native sur les versions antérieures à 2.1.116                                                                                                                                                                                                                                                                                                                                                                         |
| `bridge.claudeusercontent.com` | Pont WebSocket de l'extension [Claude in Chrome](/docs/fr/chrome)                                                                                                                                                                                                                                                                                                                                                                                                        |
| `*.claudeusercontent.com`      | Affichage des [artifacts](/docs/fr/artifacts) sur claude.ai. La visionneuse charge le contenu de chaque artifact à partir d'un sous-domaine en bac à sable de cette origine. Requis dans le navigateur de la visionneuse, pas par la CLI elle-même                                                                                                                                                                                                                       |
| `raw.githubusercontent.com`    | Flux de changelog pour [`/release-notes`](/docs/fr/commands) et les notes de version affichées après la mise à jour                                                                                                                                                                                                                                                                                                                                                      |

Si vous installez Claude Code via npm ou gérez votre propre distribution binaire, les utilisateurs finaux n'ont pas besoin de l'installateur natif et les utilisations de mise à jour automatique de `downloads.claude.ai`. Les autres utilisations du tableau s'appliquent indépendamment de la méthode d'installation.

Claude Code envoie également une télémétrie opérationnelle facultative par défaut, que vous pouvez désactiver avec des variables d'environnement. Consultez [Services de télémétrie](/docs/fr/data-usage#telemetry-services) pour savoir comment la désactiver avant de finaliser votre liste d'autorisation.

Lors de l'utilisation d'[Amazon Bedrock](/docs/fr/amazon-bedrock), de [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), de [Microsoft Foundry](/docs/fr/microsoft-foundry), ou d'une session de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) connectée, le trafic du modèle et l'authentification vont vers votre fournisseur ou passerelle au lieu de `api.anthropic.com`, `claude.ai`, ou `platform.claude.com`. L'outil WebFetch appelle toujours `api.anthropic.com` pour sa [vérification de sécurité du domaine](/docs/fr/data-usage#webfetch-domain-safety-check) sauf si vous définissez `skipWebFetchPreflight: true` dans les [paramètres](/docs/fr/settings).

[Claude Code sur le web](/docs/fr/claude-code-on-the-web) et [Code Review](/docs/fr/code-review) se connectent à vos référentiels à partir de l'infrastructure gérée par Anthropic. Si votre organisation GitHub Enterprise Cloud restreint l'accès par adresse IP, activez [l'héritage de la liste d'autorisation IP pour les applications GitHub installées](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps). L'application GitHub Claude enregistre ses plages d'adresses IP, donc l'activation de ce paramètre permet l'accès sans configuration manuelle. Pour [ajouter les plages à votre liste d'autorisation manuellement](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) à la place, ou pour configurer d'autres pare-feu, consultez les [adresses IP de l'API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

Pour les instances [GitHub Enterprise Server](/docs/fr/github-enterprise-server) auto-hébergées derrière un pare-feu, autorisez les mêmes [adresses IP de l'API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses) afin que l'infrastructure Anthropic puisse accéder à votre hôte GHES pour cloner les référentiels et publier les commentaires d'examen.

<h3 id="desktop-and-claude-ai">
  Desktop et claude.ai
</h3>

Le tableau précédent couvre principalement la CLI autonome. L'application Claude Desktop et claude.ai dans un navigateur chargent leur code d'application à partir d'hôtes CDN Anthropic supplémentaires, y compris `assets-proxy.anthropic.com`. L'autorisation de `claude.ai` tout en bloquant ces hôtes produit une page vierge plutôt qu'une erreur. Consultez [les exigences d'accès réseau](/docs/fr/desktop#network-access-requirements) sur la page Desktop.

<h2 id="additional-resources">
  Ressources supplémentaires
</h2>

* [Paramètres Claude Code](/docs/fr/settings)
* [Référence des variables d'environnement](/docs/fr/env-vars)
* [Guide de dépannage](/docs/fr/troubleshooting)
