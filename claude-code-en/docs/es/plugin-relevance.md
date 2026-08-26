> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recomendar plugins para su organización

> Agregue un bloque de relevancia a las entradas de plugins del marketplace para que Claude Code los sugiera cuando el trabajo de un usuario coincida.

Si opera un marketplace de plugins para su organización, puede hacer que Claude Code sugiera plugins específicos a los usuarios en función de lo que están haciendo. Agregue un bloque `relevance` a la entrada de un plugin en `marketplace.json`, luego incluya el marketplace en la lista de permitidos en la configuración administrada. Cuando la sesión de un usuario coincide con una de las señales declaradas, Claude Code muestra una sugerencia de instalación para ese plugin.

Las sugerencias declaradas por el marketplace son opcionales por marketplace a través de [configuración administrada](/docs/es/settings#settings-files). Ninguna declaración de `relevance` del marketplace produce sugerencias hasta que un administrador la agregue a la lista de permitidos, incluido el marketplace oficial de Anthropic. Claude Code también incluye una sugerencia integrada que es independiente de esta lista de permitidos; esa sugerencia y todas las sugerencias declaradas por el marketplace se deshabilitan cuando [`spinnerTipsEnabled`](/docs/es/settings#available-settings) se establece en `false`.

Esta función requiere Claude Code v2.1.152 o posterior. Los clientes más antiguos ignoran el campo `relevance`.

Esta página es para operadores de marketplace y administradores empresariales. Si está buscando instalar plugins, consulte [Descubrir e instalar plugins](/docs/es/discover-plugins).

<h2 id="how-it-works">
  Cómo funciona
</h2>

Cada entrada de plugin en `marketplace.json` puede llevar un objeto `relevance`. El objeto nombra un tema y una o más señales. Una señal es un patrón que Claude Code prueba contra la sesión actual, como el directorio de trabajo o los archivos que Claude ha leído.

La coincidencia de señales ocurre localmente en la máquina del usuario. La coincidencia no agrega tráfico de red y no reporta qué señales coincidieron, o sus valores, a Anthropic o al operador del marketplace.

Cuando una señal coincide y el plugin aún no está instalado, Claude Code muestra el plugin en tres lugares:

* **Sugerencia de spinner**: un mensaje "¿Trabajando con *tema*? Instale el plugin *plugin*" con el comando `/plugin install` aparece debajo del spinner mientras Claude está respondiendo.
* **Sugerencia de inicio de sesión**: si la señal `cwd` coincide con el directorio de trabajo, aparece una notificación de una línea `plugin suggestion: <name>@<marketplace> · /plugin` antes del primer turno. Esta superficie requiere Claude Code v2.1.153 o posterior.
* **Pestaña Discover de `/plugin`**: el plugin se fija en la parte superior de la lista Discover con una anotación como "sugerido para este directorio" o "sugerido para comandos stripe". Esta superficie requiere Claude Code v2.1.154 o posterior.

La sugerencia de spinner y la notificación de inicio de sesión son parte del sistema de sugerencias de spinner. Ambas se deshabilitan cuando el usuario o proyecto establece `spinnerTipsEnabled` en `false`, o cuando se configura un `spinnerTipsOverride` personalizado con `excludeDefault`. El pin de la pestaña Discover es independiente de la configuración de sugerencias.

Claude Code nunca instala un plugin automáticamente. El usuario siempre confirma.

<h2 id="add-relevance-to-a-plugin-entry">
  Agregar relevancia a una entrada de plugin
</h2>

Agregue un objeto `relevance` a la entrada del plugin en su `marketplace.json`. El siguiente ejemplo declara que el plugin `terraform-helpers` es relevante cuando Claude lee un archivo `.tf` o cuando Claude ejecuta `terraform`:

```json theme={null}
{
  "name": "acme-corp-plugins",
  "owner": { "name": "Acme Platform Team" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Acme conventions and helpers for Terraform",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

Un plugin con un bloque `relevance` pero sin una señal coincidente se comporta como cualquier otra entrada del marketplace. Aparece en la lista Discover en su posición normal y nunca aparece como una sugerencia de spinner.

<h2 id="field-reference">
  Referencia de campos
</h2>

<h3 id="relevance">
  `relevance`
</h3>

| Campo     | Tipo   | Descripción                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Opcional. La frase que completa "¿Trabajando con *tema*?" en la sugerencia de spinner. A menudo el nombre del producto, por ejemplo `Stripe`. Use un dominio como `design` cuando el nombre del plugin no se lee naturalmente como un tema. Por defecto, el nombre del plugin con cada segmento de guión en mayúsculas. La notificación de inicio de sesión no utiliza este valor. Máximo 64 caracteres. |
| `signals` | object | Coincidencias que determinan cuándo el plugin es relevante. Se requiere al menos una señal para que el plugin sea sugerible. Consulte la tabla a continuación.                                                                                                                                                                                                                                           |

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

| Campo          | Tipo             | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Patrones Glob coincididos contra el directorio de trabajo de la sesión. Coincidido como una ruta absoluta y, cuando está dentro de un repositorio git, como una ruta relativa a la raíz del repositorio. Normalizado con barra diagonal y sin distinción de mayúsculas y minúsculas. Cada patrón coincide con el directorio en sí y todo lo que hay debajo, por lo que `infra`, `infra/`, e `infra/**` se comportan de manera idéntica. Esta es la única señal que puede coincidir al inicio de la sesión, antes del primer turno. Máximo 10 patrones de 256 caracteres cada uno.                                                                                                                                                                                                                                                                                                                                                                                             |
| `cli`          | array of strings | Nombres de comandos de comandos de shell que Claude ha ejecutado en esta sesión, por ejemplo `["stripe"]`. Se aplica en todas las plataformas: los comandos ejecutados en Windows a través de PowerShell o Git Bash se registran de la misma manera. Claude Code registra un nombre de comando por invocación de herramienta de shell: el primer token después de cualquier asignación de variable de entorno inicial y `sudo`. Los comandos compuestos contribuyen solo con su comando inicial, por lo que `cd infra && terraform plan` registra `cd`, no `terraform`. Coincidencia exacta. Máximo 10 entradas de 64 caracteres cada una.                                                                                                                                                                                                                                                                                                                                    |
| `hosts`        | array of strings | Nombres de host vistos en URLs `http://` o `https://` en comandos Bash en esta sesión, por ejemplo `["api.stripe.com"]`. Solo nombre de host en minúsculas: sin esquema, puerto o ruta. Coincidencia exacta sin distinción de mayúsculas y minúsculas. Máximo 20 entradas de 128 caracteres cada una.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `filesRead`    | array of strings | Patrones Glob coincididos contra las rutas de archivos que Claude ha leído en esta sesión, por ejemplo `["**/*.tf"]`. Normalizado con barra diagonal y sin distinción de mayúsculas y minúsculas. Máximo 10 patrones de 256 caracteres cada uno.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `manifestDeps` | array of objects | Dependencias declaradas en manifiestos de paquetes que Claude ha leído en esta sesión. Cada entrada es `{ "file": "...", "pattern": "..." }`, donde `file` es una expresión regular coincidida contra la ruta del archivo de manifiesto tal como se registra en el estado de la sesión, típicamente una ruta absoluta, y `pattern` es una expresión regular coincidida contra el contenido de ese archivo. Ancle `file` al final, por ejemplo `[/\\\\]package\\.json$` en forma escapada en JSON, porque un patrón anclado al inicio nunca coincide con una ruta absoluta. Las rutas no se normalizan de separador para esta señal, por lo que las rutas de Windows usan barras invertidas. Los archivos de manifiesto más grandes de 512 KB se omiten. Ambos valores son cadenas de fuente `RegExp` de JavaScript de como máximo 256 caracteres. `file` coincide sin distinción de mayúsculas y minúsculas. `pattern` distingue mayúsculas y minúsculas. Máximo 10 entradas. |

Las señales `cli`, `hosts`, `filesRead` y `manifestDeps` necesitan historial de sesión, por lo que solo pueden coincidir en la sugerencia de spinner y la pestaña Discover. Solo `cwd` puede coincidir al inicio de la sesión. Las señales `filesRead` y `manifestDeps` prueban el estado de archivo registrado de la sesión, que también incluye archivos que Claude ha escrito o editado y archivos de memoria `CLAUDE.md` cargados automáticamente.

El siguiente ejemplo usa `manifestDeps` para sugerir un plugin de Stripe una vez que Claude ha leído un `package.json` que depende de `stripe`. El patrón `file` usa `[/\\\\]` para que coincida tanto con separadores de barra diagonal como de barra invertida, y `\\.` para que el punto sea literal. En JSON, cada barra invertida en la expresión regular se escribe dos veces.

```json theme={null}
{
  "name": "stripe-helpers",
  "source": "./plugins/stripe-helpers",
  "relevance": {
    "topic": "Stripe",
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"stripe\"\\s*:"
        }
      ]
    }
  }
}
```

<Note>
  Los campos desconocidos bajo `relevance` y `relevance.signals` se ignoran en el tiempo de carga para que los clientes más antiguos de Claude Code continúen cargando su marketplace. Ejecute `claude plugin validate` para mostrarlos como advertencias.
</Note>

<h2 id="enable-suggestions-in-managed-settings">
  Habilitar sugerencias en la configuración administrada
</h2>

Declarar `relevance` en `marketplace.json` no es suficiente por sí solo. Un administrador debe incluir el marketplace en la lista de permitidos en [configuración administrada](/docs/es/settings#settings-files) antes de que sus sugerencias aparezcan a los usuarios.

Agregue el nombre del marketplace a `pluginSuggestionMarketplaces`. Para cualquier marketplace que no sea el marketplace oficial de Anthropic, también declare la fuente del marketplace en la misma configuración administrada, ya sea como entrada de ese nombre en `extraKnownMarketplaces` o como entrada en `strictKnownMarketplaces`. El nombre incluido en la lista de permitidos se ignora si el marketplace registrado en la máquina proviene de una fuente diferente. Esto evita que una fuente no relacionada se registre bajo un nombre incluido en la lista de permitidos para que sus plugins se sugieran en toda su organización.

El siguiente `managed-settings.json` registra un marketplace de organización desde un repositorio de GitHub y habilita sus sugerencias:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-corp-plugins": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

El marketplace oficial está exento del requisito de declaración de fuente porque su nombre solo puede registrarse desde la fuente oficial de Anthropic. Incluir el nombre en la lista de permitidos es suficiente:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

Consulte la [referencia de configuración](/docs/es/settings) para `pluginSuggestionMarketplaces` y [`extraKnownMarketplaces`](/docs/es/settings#extraknownmarketplaces) para obtener detalles de configuración completos.

<h2 id="what-the-user-sees">
  Lo que ve el usuario
</h2>

Cuando una señal coincide durante una sesión, la sugerencia de spinner dice:

```text theme={null}
¿Trabajando con Terraform? Instale el plugin terraform-helpers:
/plugin install terraform-helpers@acme-corp-plugins
```

Al inicio de la sesión, una señal `cwd` coincidente muestra la notificación de una línea:

```text theme={null}
plugin suggestion: terraform-helpers@acme-corp-plugins · /plugin
```

La sugerencia de un plugin determinado aparece como máximo una vez cada tres sesiones en la sugerencia de spinner y la notificación de inicio de sesión combinadas, y ninguna se repite una vez que el plugin está instalado. La notificación de inicio de sesión además deja de aparecer después de que la sugerencia se ha mostrado dos veces.

En la pestaña Discover de `/plugin`, el plugin se fija encima de los otros resultados con una anotación que nombra la señal coincidente, como `suggested for this directory` o `suggested for terraform commands`. La pestaña Discover fija un plugin determinado una vez; las visitas posteriores lo enumeran en orden normal. El pin de la pestaña Discover requiere Claude Code v2.1.154 o posterior. En v2.1.152 solo aparece la sugerencia de spinner; la notificación de inicio de sesión se agrega en v2.1.153.

<h2 id="validate-your-marketplace">
  Validar su marketplace
</h2>

Ejecute `claude plugin validate` contra su directorio de marketplace para verificar el bloque `relevance` antes de publicar:

```
claude plugin validate ./my-marketplace
```

El validador reporta claves desconocidas bajo `relevance` y `relevance.signals` como advertencias, marca un valor `relevance` que no es un objeto, y rechaza una entrada `signals.hosts` que incluye un esquema, puerto o ruta.

<h2 id="see-also">
  Véase también
</h2>

* [Crear y distribuir un marketplace de plugins](/docs/es/plugin-marketplaces): construya el marketplace que aloja sus plugins
* [Recomendar su plugin desde su CLI](/docs/es/plugin-hints): solicite a los usuarios desde su propia CLI en lugar de desde las señales de sesión de Claude Code
* [Configuración](/docs/es/settings): referencia completa para `pluginSuggestionMarketplaces` y `extraKnownMarketplaces`
