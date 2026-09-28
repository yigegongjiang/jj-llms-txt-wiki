> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code con GitHub Enterprise Server

> Conecte Claude Code a su instancia de GitHub Enterprise Server autohospedada para sesiones en la nube, revisión de código y mercados de plugins.

<Note>
  La compatibilidad con GitHub Enterprise Server está disponible para planes Team y Enterprise.
</Note>

La compatibilidad con GitHub Enterprise Server (GHES) permite que su organización use Claude Code con repositorios alojados en su instancia de GitHub autogestionada en lugar de github.com. Una vez que un propietario conecta su instancia de GHES, los desarrolladores pueden ejecutar sesiones en la nube y obtener revisiones de código automatizadas sin ninguna configuración por repositorio. Los mercados de plugins alojados en su instancia también son compatibles; los requisitos de credenciales varían según la superficie, como se describe en [Plugin marketplaces on GHES](#plugin-marketplaces-on-ghes).

Para repositorios en github.com, consulte [Claude Code en la nube](/docs/es/claude-code-on-the-web) y [Revisión de código](/docs/es/code-review). Para ejecutar Claude en su propia infraestructura de CI, consulte [GitHub Actions](/docs/es/github-actions).

<h2 id="what-works-with-github-enterprise-server">
  Qué funciona con GitHub Enterprise Server
</h2>

La tabla a continuación muestra qué características de Claude Code admiten GHES y cualquier diferencia del comportamiento de github.com.

| Característica           | Compatibilidad con GHES | Notas                                                                                                                                                       |
| :----------------------- | :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sesiones en la nube      | ✅ Compatible            | Un propietario conecta la instancia de GHES una vez; los desarrolladores usan `claude --cloud` o [claude.ai/code](https://claude.ai/code) como de costumbre |
| Revisión de código       | ✅ Compatible            | Las mismas revisiones automatizadas de PR que en github.com                                                                                                 |
| Claude Security          | ✅ Compatible            | Disponible en versión beta pública para planes Enterprise en [claude.ai/security](https://claude.ai/security)                                               |
| Sesiones de Teleport     | ✅ Compatible            | Mover sesiones entre nube y terminal con `--teleport`                                                                                                       |
| Mercados de plugins      | ✅ Compatible            | Los requisitos de credenciales difieren según la superficie. Consulte [Mercados de plugins en GHES](#plugin-marketplaces-on-ghes)                           |
| Métricas de contribución | ✅ Compatible            | Entregadas a través de webhooks al [panel de análisis](/docs/es/analytics)                                                                                       |
| GitHub Actions           | ✅ Compatible            | Requiere configuración manual del flujo de trabajo; `/install-github-app` es solo para github.com                                                           |
| Servidor GitHub MCP      | ❌ No compatible         | El servidor GitHub MCP no funciona con instancias de GHES                                                                                                   |

<h2 id="admin-setup">
  Configuración del administrador
</h2>

Un propietario conecta su instancia de GHES a Claude Code una sola vez. Después de eso, los desarrolladores en su organización pueden usar repositorios de GHES sin ninguna configuración adicional. Necesita el rol de Propietario o Propietario Principal en su organización de Claude y permiso para crear GitHub Apps en su instancia de GHES.

La configuración guiada genera un manifiesto de GitHub App y lo redirige a su instancia de GHES para crear la aplicación en un clic. Si su entorno bloquea el flujo de redirección, hay una [configuración manual alternativa](#manual-setup) disponible.

<Steps>
  <Step title="Abrir la configuración de administrador de Claude Code">
    Vaya a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) y encuentre la sección GitHub Enterprise Server.
  </Step>

  <Step title="Iniciar la configuración guiada">
    Haga clic en **Conectar**. Ingrese un nombre para mostrar de hasta 20 caracteres para la conexión y el nombre de host de su GHES, por ejemplo `github.example.com`. Si su instancia de GHES usa un certificado autofirmado o una autoridad de certificación privada, pegue el certificado de CA en el campo opcional.
  </Step>

  <Step title="Crear la GitHub App">
    Haga clic en **Continuar a GitHub Enterprise**. Su navegador se redirige a su instancia de GHES con un manifiesto de aplicación previamente rellenado. Revise la configuración y haga clic en **Crear GitHub App**. GHES lo redirige de vuelta a Claude con las credenciales de la aplicación almacenadas automáticamente.
  </Step>

  <Step title="Instalar la aplicación en sus repositorios">
    Desde la página de GitHub App en su instancia de GHES, instale la aplicación en los repositorios u organizaciones a los que desea que Claude tenga acceso. Puede comenzar con un subconjunto y agregar más más adelante.
  </Step>

  <Step title="Habilitar características">
    Vuelva a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) y habilite [Revisión de código](/docs/es/code-review#set-up-code-review), Claude Security y [métricas de contribución](/docs/es/analytics#enable-contribution-metrics) para sus repositorios de GHES usando la misma configuración que github.com.
  </Step>
</Steps>

<h3 id="github-app-permissions">
  Permisos de GitHub App
</h3>

El manifiesto configura la GitHub App con los permisos y eventos de webhook que se muestran a continuación, que en conjunto cubren sesiones web, Revisión de código, Claude Security, mercados de plugins y métricas de contribución:

| Permiso              | Acceso              | Usado para                                                                                                                                                                                                                                           |
| :------------------- | :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Contents             | Lectura y escritura | Clonar repositorios e insertar ramas                                                                                                                                                                                                                 |
| Pull requests        | Lectura y escritura | Crear PR y publicar comentarios de revisión                                                                                                                                                                                                          |
| Issues               | Lectura y escritura | Responder a menciones de problemas                                                                                                                                                                                                                   |
| Checks               | Lectura y escritura | Publicar ejecuciones de verificación de Revisión de código                                                                                                                                                                                           |
| Actions              | Lectura             | Leer estado de CI para corrección automática                                                                                                                                                                                                         |
| Commit statuses      | Lectura             | Leer estado de CI de proveedores que reportan estados de confirmación en lugar de ejecuciones de verificación                                                                                                                                        |
| Repository hooks     | Lectura y escritura | Crear un webhook en un repositorio de mercado de plugins cuando **Sincronizar automáticamente** está activado para un mercado en [**Configuración de la organización > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=marketplaces) |
| Metadata             | Lectura             | Requerido por GitHub para todas las aplicaciones                                                                                                                                                                                                     |
| Organization members | Lectura             | Coincidencia de la GitHub App de Claude en github.com, que la usa para verificar el rol de organización de un usuario que se conecta al vincular una instalación                                                                                     |

La aplicación se suscribe a eventos `pull_request`, `issue_comment`, `pull_request_review_comment`, `pull_request_review`, `check_run` y `status`.

GitHub aplica un manifiesto solo cuando se crea la aplicación, por lo que una aplicación creada a partir de una versión anterior del manifiesto mantiene los permisos y eventos con los que se creó. Si su aplicación no tiene ninguno de los permisos o eventos anteriores, agréguelos en la configuración de la aplicación en su instancia de GHES. GitHub luego le pide al propietario de cada instalación que apruebe los nuevos permisos, y la instalación mantiene sus permisos antiguos hasta que lo hagan.

<h3 id="manual-setup">
  Configuración manual
</h3>

Si el flujo de redirección guiado está bloqueado por su configuración de red, haga clic en **Agregar manualmente** en lugar de Conectar. Cree una GitHub App en su instancia de GHES con los [permisos y eventos anteriores](#github-app-permissions), luego ingrese los detalles de la conexión en el formulario: un nombre para mostrar, el nombre de host de su GHES y puerto opcional, y el ID de la aplicación, ID de cliente, secreto de cliente, secreto de webhook y clave privada. El formulario también acepta un certificado de CA personalizado opcional y nombres de host de réplica de lectura.

Claude genera la URL del webhook de la aplicación cuando guarda la conexión. Después de hacer clic en **Agregar configuración**, abra el menú **Más opciones** de la conexión, seleccione **Copiar URL del webhook** y pegue la URL en la configuración del webhook de la aplicación en su instancia de GHES. Use el mismo secreto de webhook que ingresó en el formulario.

<h3 id="network-requirements">
  Requisitos de red
</h3>

Para sesiones alojadas en Anthropic, su instancia de GHES debe ser accesible desde la infraestructura de Anthropic para que Claude pueda clonar repositorios y publicar comentarios de revisión. Si su instancia de GHES está detrás de un firewall, agregue a la lista blanca las [direcciones IP salientes](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) de Anthropic. Las sesiones en un [entorno autohospedado](/docs/es/self-hosted-environments-deploy#configure-git) clonan desde dentro de su red en su lugar, a menos que el ejecutor opte por el [proxy de git de Anthropic](/docs/es/self-hosted-environments-deploy#use-the-anthropic-git-proxy), que obtiene desde el lado de Anthropic y necesita la misma accesibilidad; el [conector de SCM](/docs/es/self-hosted-environments-reference#scm-connector-flags) cubre los flujos previos a la sesión alojados, como el selector de repositorio, para un host de GHES que solo es enrutable internamente.

<h2 id="developer-workflow">
  Flujo de trabajo del desarrollador
</h2>

Una vez que su administrador ha conectado la instancia de GHES, no se necesita ninguna configuración del lado del desarrollador. Claude Code detecta automáticamente el nombre de host de su GHES desde el remoto de git en su directorio de trabajo.

Clone un repositorio de su instancia de GHES como lo haría normalmente, reemplazando `github.example.com` y la ruta del repositorio con su nombre de host de GHES y repositorio:

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

Luego inicie una sesión en la nube. Claude detecta el host de GHES desde su remoto de git y enruta la sesión a través de su instancia configurada de la organización:

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

La sesión clona su repositorio desde GHES e inserta cambios en una rama. Monitoree el progreso en [claude.ai/code](https://claude.ai/code). Consulte [Claude Code en la nube](/docs/es/claude-code-on-the-web) para el flujo de trabajo completo de sesión en la nube, incluida revisión de diferencias, corrección automática y rutinas.

<h3 id="teleport-sessions-to-your-terminal">
  Sesiones de Teleport a su terminal
</h3>

Extraiga una sesión en la nube a su terminal local con `claude --teleport`. Teleport verifica que esté en un checkout del mismo repositorio de GHES antes de obtener la rama y cargar el historial de sesiones. Consulte [requisitos de teleport](/docs/es/claude-code-on-the-web#teleport-requirements) para obtener detalles.

<h2 id="plugin-marketplaces-on-ghes">
  Mercados de plugins en GHES
</h2>

Aloje mercados de plugins en su instancia de GHES para distribuir herramientas internas en toda su organización. La estructura del mercado es idéntica a los mercados alojados en github.com, pero la instalación funciona de manera diferente según dónde agregue el mercado, y las credenciales varían según la superficie:

| Superficie                                               | Cómo funciona la instalación                                                                                                                                                                                                                                      | Lo que cada usuario necesita                                                                                                                                                                                                                      |
| :------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Claude Code CLI y escritorio                             | Claude Code clona el repositorio del mercado usando las credenciales de git existentes de la máquina                                                                                                                                                              | Acceso a Git a su host de GHES desde su máquina                                                                                                                                                                                                   |
| Configuración administrada (`extraKnownMarketplaces`)    | Claude Code registra la entrada y clona el repositorio usando las credenciales de git existentes de la máquina                                                                                                                                                    | Acceso a Git a su host de GHES desde su máquina                                                                                                                                                                                                   |
| Configuración de plugins de la organización en claude.ai | Un Propietario selecciona la instancia de GHES como fuente; el backend de Anthropic obtiene y sincroniza el repositorio usando la GitHub App de [configuración de administrador](#admin-setup)                                                                    | Nada por usuario una vez agregado. El Propietario que lo agrega necesita su propia cuenta de GitHub Enterprise conectada como verificación de acceso, y la GitHub App debe estar instalada en el repositorio del mercado                          |
| Configuración de usuario en claude.ai                    | El backend de Anthropic obtiene el repositorio usando la conexión de GitHub Enterprise del usuario que lo envía                                                                                                                                                   | Su propia cuenta de GitHub Enterprise conectada a Claude                                                                                                                                                                                          |
| Sesiones en la nube                                      | Las sesiones en la nube clonan mercados dentro del sandbox de la sesión. El sandbox solo puede alcanzar su instancia de GHES cuando el repositorio de la sesión está en esa misma instancia, y sus credenciales de git se limitan a los repositorios de la sesión | No es confiable para mercados alojados en GHES: un host diferente al repositorio de la sesión no es accesible, e incluso las instalaciones en la misma instancia pueden fallar. Use la CLI, la configuración administrada o claude.ai en su lugar |

<Warning>
  Las conexiones de GitHub Enterprise en claude.ai son por usuario cuando se agrega un mercado desde la configuración de usuario. La [configuración de administrador](#admin-setup) conecta su instancia de GHES a su organización, pero no conecta cuentas de usuario individuales: cada usuario que agregue un mercado de GHES desde su propia configuración debe conectar primero su propia cuenta de GitHub Enterprise, y la conexión de un usuario, incluida la del Propietario, no cubre a nadie más. Los mercados agregados por un Propietario en la configuración de plugins de la organización no imponen este requisito a los usuarios, porque las obtenciones continuas usan la GitHub App de la organización. El Propietario que agrega el mercado aún necesita su propia cuenta de GitHub Enterprise conectada en el momento de la adición.
</Warning>

<h3 id="add-a-ghes-marketplace">
  Agregar un mercado de GHES
</h3>

La abreviatura `owner/repo` siempre se resuelve en github.com. Para mercados alojados en GHES, use la URL de git completa, reemplazando `github.example.com` y la ruta del repositorio con la suya propia. Se recomiendan las URLs de HTTPS:

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

Las URLs de SSH funcionan si la máquina ya confía en su host de GHES:

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code ejecuta git de forma no interactiva y rechaza las conexiones SSH a hosts que no están en el archivo `known_hosts` de la máquina. Una URL de HTTPS con un asistente de credenciales de git evita el requisito de `known_hosts`.

Consulte [Crear y distribuir un mercado de plugins](/docs/es/plugins/create-marketplace) para la guía completa sobre cómo construir mercados.

<h3 id="pre-register-ghes-marketplaces-with-managed-settings">
  Preregistrar mercados de GHES con configuración administrada
</h3>

La configuración `extraKnownMarketplaces` preregistra un mercado para que los desarrolladores lo obtengan sin configuración manual. Funciona desde [cualquier archivo de configuración](/docs/es/settings-reference#extraknownmarketplaces), incluido el `.claude/settings.json` de un repositorio; la configuración administrada lo entrega en toda la organización:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code instala estos mercados localmente: registra cada entrada y clona el repositorio con las credenciales de git existentes de la máquina. Esta ruta no pasa por claude.ai, por lo que no se requiere la conexión de GitHub Enterprise por usuario. Para un despliegue exitoso:

* **Use una URL de git completa.** La abreviatura `owner/repo` siempre se resuelve en github.com y no puede hacer referencia a un host de GHES.
* **Prefiera URLs de HTTPS.** Los clones de SSH fallan en máquinas que no confían en la clave de host de GHES. Una URL de HTTPS con el asistente de credenciales de git estándar de su organización funciona en cualquier máquina con credenciales configuradas.
* **Confirme que cada máquina pueda clonar desde su host de GHES.** Si una máquina carece de credenciales, el mercado se registra pero nunca se instala, y sus plugins se reportan como no encontrados en lugar de solicitar credenciales.
* **Confirme que la configuración llega a cada máquina.** Un archivo de configuración administrada solo tiene efecto en las máquinas en las que se implementa, por ejemplo a través de su sistema de administración de dispositivos. Consulte [Implementar configuración administrada](/docs/es/managed-settings#delivery-mechanisms) para ubicaciones de archivos.

<h3 id="allowlist-ghes-marketplaces-in-managed-settings">
  Agregar mercados de GHES a la lista blanca en configuración administrada
</h3>

Si su organización usa [configuración administrada](/docs/es/settings) para restringir qué mercados pueden agregar los desarrolladores, use el tipo de fuente `hostPattern` para permitir todos los mercados de su instancia de GHES sin enumerar cada repositorio. Consulte [Mecanismos de entrega](/docs/es/managed-settings#delivery-mechanisms) para ubicaciones de archivos en cada plataforma. Agregue el JSON a su archivo `managed-settings.json` o política MDM equivalente:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Consulte la referencia de configuración [strictKnownMarketplaces](/docs/es/settings-reference#strictknownmarketplaces) y [extraKnownMarketplaces](/docs/es/settings-reference#extraknownmarketplaces) para el esquema completo.

<h2 id="limitations">
  Limitaciones
</h2>

Algunas características se comportan de manera diferente en GHES que en github.com. La [tabla de características](#what-works-with-github-enterprise-server) resume la compatibilidad; esta sección cubre las soluciones alternativas.

* **Comando `/install-github-app`**: siga el flujo de [configuración del administrador](#admin-setup) en claude.ai en su lugar. Si también desea flujos de trabajo de GitHub Actions en GHES, adapte el [flujo de trabajo de ejemplo](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) manualmente.
* **Servidor GitHub MCP**: use la CLI `gh` configurada para su host de GHES en su lugar. Ejecute `gh auth login --hostname github.example.com` para autenticarse, luego Claude puede usar comandos `gh` en sesiones.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="cloud-session-fails-to-clone-repository">
  La sesión en la nube falla al clonar el repositorio
</h3>

Si `claude --cloud` falla con un error de clonación, verifique que un Propietario haya completado la configuración para su instancia de GHES y que la GitHub App esté instalada en el repositorio en el que está trabajando. Pida al Propietario que conectó la instancia que confirme que el nombre de host registrado en la configuración de Claude coincida con el nombre de host en su remoto de git.

<h3 id="marketplace-add-fails-with-a-policy-error">
  Agregar mercado falla con un error de política
</h3>

Si `/plugin marketplace add` está bloqueado para su URL de GHES, su organización ha restringido las fuentes del mercado. Pida a su administrador que agregue una entrada `hostPattern` para su nombre de host de GHES en [configuración administrada](#allowlist-ghes-marketplaces-in-managed-settings).

<h3 id="marketplace-add-on-claude-ai-fails-with-a-github-access-error">
  Agregar mercado en claude.ai falla con un error de acceso a GitHub
</h3>

Si agregar un mercado de GHES desde la configuración de usuario falla con un error genérico como "No se pudo agregar el mercado", primero verifique su conexión de GitHub Enterprise. Esto es lo que aparece cuando su propia cuenta de GitHub Enterprise no está conectada a Claude, incluso si la instancia de GHES de su organización está configurada y otros usuarios están conectados. El diálogo no apunta al flujo de conexión de GitHub Enterprise, y la opción "Conectar a GitHub" en la pestaña Examinar inicia sesión en github.com, lo que no otorga acceso a repositorios de GHES.

Para conectar su cuenta de GitHub Enterprise: el selector de repositorio en [claude.ai/code](https://claude.ai/code) ofrece una opción de conexión para cada instancia de GHES configurada, y los Propietarios también pueden conectarse desde la sección de GitHub Enterprise de la [configuración de administrador de Claude Code](https://claude.ai/admin-settings/claude-code). Luego agregue el mercado nuevamente. Alternativamente, pida a un Propietario que agregue el mercado en la configuración de plugins de la organización, lo que elimina el requisito de conexión por usuario.

En otras superficies de claude.ai, un error "Repositorio no encontrado. Si es privado, se requiere acceso a GitHub" en un mercado de GHES generalmente indica la misma conexión faltante. Conecte su cuenta de GitHub Enterprise a través de una de las rutas anteriores y luego intente de nuevo.

<h3 id="ghes-instance-not-reachable">
  Instancia de GHES no accesible
</h3>

Si las revisiones o sesiones en la nube alojadas por Anthropic se agotan, su instancia de GHES puede no ser accesible desde la infraestructura de Anthropic. Confirme que su firewall permite conexiones entrantes desde las [direcciones IP de salida](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) de Anthropic. Las sesiones en un [entorno autohospedado](/docs/es/self-hosted-environments) alcanzan GHES desde dentro de su red, así que para ellas verifique la ruta de red propia del ejecutor y el [conector SCM](/docs/es/self-hosted-environments-reference#scm-connector-flags) en su lugar.

<h3 id="session-start-fails-with-unable-to-get-organization-uuid">
  El inicio de sesión falla con `Unable to get organization UUID`
</h3>

Las sesiones en la nube requieren una organización de Equipo o Empresa. Inicie sesión con `/login` usando su cuenta de organización. Si se autentica con una clave de API en su lugar, las sesiones en la nube fallan antes con un mensaje pidiéndole que ejecute `/login`.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Estas páginas cubren las características referenciadas en toda esta guía con más profundidad:

* [Claude Code en la web](/docs/es/claude-code-on-the-web): ejecutar sesiones de Claude Code en infraestructura en la nube
* [Revisión de código](/docs/es/code-review): revisiones automatizadas de PR
* [Mercados de plugins](/docs/es/plugins/host-marketplace): construir y distribuir catálogos de plugins
* [Analytics](/docs/es/analytics): rastrear uso y métricas de contribución
* [Configuración administrada](/docs/es/settings): configuración de política en toda la organización
* [Configuración de red](/docs/es/network-config): requisitos de firewall y lista blanca de IP
