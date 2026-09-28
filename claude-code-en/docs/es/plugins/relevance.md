> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recomendar plugins para su organización

> Agregue un bloque de relevancia a las entradas de plugins del marketplace para que Claude Code los sugiera cuando el trabajo de un usuario coincida, y permita el marketplace en la configuración administrada.

Claude Code puede sugerir la instalación de un plugin del marketplace de su organización cuando la sesión de un usuario coincide con señales que usted define para ese plugin. Las señales incluyen el directorio de trabajo, los archivos que Claude ha leído y los comandos que Claude ha ejecutado. Las define agregando un bloque `relevance` a la entrada del plugin en `marketplace.json`.

Un operador del marketplace escribe las entradas de `relevance`. Un administrador luego permite el marketplace en la configuración administrada. Los usuarios no ven sugerencias de un marketplace hasta que se permita.

<Note>
  Estos casos se cubren en otras páginas:

  * **Desea instalar plugins**: consulte [Instalar y administrar plugins](/docs/es/plugins/install)
  * **Desea desactivar sugerencias**: consulte [Entender cómo funciona la relevancia de plugins](#understand-how-plugin-relevance-works)
</Note>

Comience con las secciones para su rol:

* **Operadores del marketplace**: lea [cómo funcionan las sugerencias](#understand-how-plugin-relevance-works), luego [agregue relevancia a una entrada de plugin](#add-relevance-to-a-plugin-entry) y [valide su marketplace](#validate-your-marketplace)
* **Administradores**: [habilite sugerencias en la configuración administrada](#enable-suggestions-in-managed-settings)

<h2 id="understand-how-plugin-relevance-works">
  Entender cómo funciona la relevancia de plugins
</h2>

Cada entrada de plugin en `marketplace.json` puede incluir un objeto `relevance`. El objeto nombra un tema y una o más señales. Una señal es un patrón que Claude Code prueba contra la sesión actual, como el directorio de trabajo o los archivos que Claude ha leído.

La coincidencia de señales ocurre localmente en la máquina del usuario y no agrega tráfico de red. Claude Code no reporta qué señales coincidieron o sus valores a Anthropic o al operador del marketplace.

Cuando una señal coincide y el plugin aún no está instalado, Claude Code sugiere el plugin en estos lugares:

* **Spinner tip**: un mensaje con el comando `/plugin install` aparece debajo del spinner mientras Claude responde.
* **Notificación de inicio de sesión**: si una señal `cwd` coincide con el directorio de trabajo, aparece una notificación de una línea antes de que el usuario envíe un primer mensaje.
* **Pestaña Discover de `/plugin`**: el plugin se fija en la parte superior de la lista Discover.

[Vista previa de lo que ve el usuario](#preview-what-the-user-sees) muestra el texto exacto de cada uno y con qué frecuencia se repiten.

Claude Code nunca instala el plugin automáticamente. El usuario siempre confirma.

El spinner tip y la notificación de inicio de sesión ambos dejan de aparecer cuando el usuario o proyecto establece [`spinnerTipsEnabled`](/docs/es/settings-reference#spinnertipsenabled) en `false`, o cuando un [`spinnerTipsOverride`](/docs/es/settings-reference#spinnertipsoverride) con `excludeDefault` reemplaza los consejos integrados. El pin de la pestaña Discover no se ve afectado por ninguna de las dos configuraciones.

<h2 id="add-relevance-to-a-plugin-entry">
  Agregar relevancia a una entrada de plugin
</h2>

Agregue un objeto `relevance` a la entrada del plugin en su `marketplace.json`. El siguiente ejemplo declara que el plugin `terraform-helpers` es relevante cuando Claude lee un archivo `.tf` o ejecuta `terraform`:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
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

Mientras ninguna de sus señales coincida, el plugin mantiene su posición normal en la lista Discover y no aparece como un spinner tip.

Para verificar el bloque antes de publicar, [valide su marketplace](#validate-your-marketplace).

<h2 id="field-reference">
  Referencia de campos
</h2>

El objeto `relevance` y su objeto anidado `signals` aceptan los campos en las siguientes tablas.

Los clientes más antiguos aún cargan un marketplace que usa campos `relevance` que no reconocen, porque los campos desconocidos bajo `relevance` y `relevance.signals` se ignoran en el momento de la carga. Un campo reconocido cuyo valor excede su límite en la [referencia de campos](#field-reference) invalida toda la entrada del plugin, y los usuarios no pueden instalar ese plugin desde el marketplace hasta que lo corrija; `claude plugin validate` reporta los mismos límites.

<h3 id="relevance">
  `relevance`
</h3>

| Campo     | Tipo   | Descripción                                                                                                                                                                                |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Opcional. La frase que completa "¿Trabajando con *topic*?" en el spinner tip. Por defecto es el nombre del plugin con cada segmento de guión en mayúscula. Máximo 64 caracteres.           |
| `signals` | object | Coincidencias que determinan cuándo el plugin es relevante. Claude Code sugiere el plugin solo si al menos una señal está establecida. Consulte [`relevance.signals`](#relevance-signals). |

El `topic` suele ser el nombre del producto, por ejemplo `Terraform`. Use un dominio como `design` cuando el nombre del plugin no suena natural como un tema.

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

El objeto `signals` acepta los siguientes campos.

| Campo          | Tipo             | Descripción                                                                                                                                                                                                                                                                    | Límite                                                                                                         |
| :------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Patrones Glob coincididos contra el directorio de trabajo de la sesión. Consulte [coincidencia de directorio de trabajo](#working-directory-matching).                                                                                                                         | 10 patrones de 256 caracteres cada uno                                                                         |
| `cli`          | array of strings | Nombres de comandos de comandos shell que Claude ha ejecutado esta sesión, por ejemplo `["terraform"]`. Coincidencia exacta. Consulte [coincidencia de nombre de comando](#command-name-matching).                                                                             | 10 entradas de 64 caracteres cada una                                                                          |
| `hosts`        | array of strings | Nombres de host vistos en URLs `http://` o `https://` en comandos Bash esta sesión, por ejemplo `["registry.terraform.io"]`. Solo nombre de host en minúsculas: sin esquema, puerto o ruta. Coincidencia exacta sin distinción de mayúsculas y minúsculas.                     | 20 entradas de 128 caracteres cada una                                                                         |
| `filesRead`    | array of strings | Patrones Glob coincididos contra las rutas de archivos que Claude ha leído esta sesión, por ejemplo `["**/*.tf"]`. Normalizado con barra diagonal hacia adelante y sin distinción de mayúsculas y minúsculas.                                                                  | 10 patrones de 256 caracteres cada uno                                                                         |
| `manifestDeps` | array of objects | Dependencias declaradas en manifiestos de paquetes que Claude ha leído esta sesión. Cada entrada es `{ "file": "...", "pattern": "..." }`, donde ambos valores son expresiones regulares. Consulte [coincidencia de dependencia de manifiesto](#manifest-dependency-matching). | 10 entradas, cada valor como máximo 256 caracteres. Los archivos de manifiesto más grandes de 512 KB se omiten |

Las señales `filesRead` y `manifestDeps` también coinciden contra archivos que Claude ha escrito o editado esta sesión y contra los archivos de memoria `CLAUDE.md` cargados automáticamente del proyecto.

<h4 id="working-directory-matching">
  Coincidencia de directorio de trabajo
</h4>

`cwd` es la única señal que puede coincidir al inicio de la sesión, antes de que el usuario envíe un primer mensaje.

Claude Code coincide cada patrón `cwd` de la siguiente manera:

* El patrón se compara contra el directorio de trabajo como una ruta absoluta. Cuando la sesión está dentro de un repositorio git, también se compara contra la ruta del directorio de trabajo relativa a la raíz del repositorio.
* La coincidencia se normaliza con barra diagonal hacia adelante y sin distinción de mayúsculas y minúsculas.
* Cada patrón coincide con el directorio en sí y todo lo que hay debajo, por lo que `infra`, `infra/` e `infra/**` se comportan de manera idéntica.

<h4 id="command-name-matching">
  Coincidencia de nombre de comando
</h4>

Claude Code registra un nombre de comando para cada comando shell que Claude ejecuta: el primer token después de cualquier asignación de variable de entorno inicial y `sudo`. Los comandos compuestos contribuyen solo con su comando inicial, por lo que `cd infra && terraform plan` registra `cd`, no `terraform`.

<h4 id="manifest-dependency-matching">
  Coincidencia de dependencia de manifiesto
</h4>

Cada entrada de `manifestDeps` empareja dos cadenas de origen de JavaScript `RegExp`:

* `file`: coincidido sin distinción de mayúsculas y minúsculas contra la ruta del archivo de manifiesto. La ruta es típicamente absoluta, por lo que ancle el patrón al final en lugar del inicio. Las rutas no se normalizan por separador para esta señal, por lo que las rutas de Windows usan barras invertidas.
* `pattern`: coincidido con distinción de mayúsculas y minúsculas contra el contenido de ese archivo.

El siguiente ejemplo usa `manifestDeps` para sugerir su plugin una vez que Claude ha leído un `package.json` que depende del paquete npm de su SDK, nombrado `your-sdk` aquí.

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

En este ejemplo, el patrón `file` usa `[/\\\\]` para que coincida tanto con separadores de ruta de barra diagonal como de barra invertida, y `\\.` para que el punto sea literal. En JSON, cada barra invertida en la expresión regular se escribe dos veces.

<h2 id="validate-your-marketplace">
  Validar su marketplace
</h2>

En su shell, ejecute `claude plugin validate` contra su directorio de marketplace para verificar el bloque `relevance` antes de publicar:

```bash theme={null}
claude plugin validate ./my-marketplace
```

El validador reporta errores y advertencias en el bloque `relevance`, incluyendo estos:

* Reporta claves desconocidas bajo `relevance` y `relevance.signals` como advertencias
* Marca un valor `relevance` que no es un objeto
* Rechaza una entrada `signals.hosts` que incluye un esquema, puerto o ruta

Cada hallazgo se imprime con la ruta del campo que le concierne, y la salida termina con `Validation passed`, `Validation passed with warnings` o `Validation failed`.

<h2 id="enable-suggestions-in-managed-settings">
  Habilitar sugerencias en la configuración administrada
</h2>

Los usuarios no ven sugerencias de un marketplace hasta que un administrador lo permita en [configuración administrada](/docs/es/plugins/org), incluso cuando su `marketplace.json` declara `relevance`.

Para permitir un marketplace, edite su configuración administrada de la siguiente manera:

* Agregue el nombre del marketplace a `pluginSuggestionMarketplaces`.
* Para cualquier marketplace que no sea el marketplace oficial de Anthropic, también declare la fuente del marketplace, ya sea como entrada de ese nombre en [`extraKnownMarketplaces`](/docs/es/plugins/org#require-a-marketplace-and-its-plugins) o como entrada en [`strictKnownMarketplaces`](/docs/es/plugins/org#allowlist-with-strictknownmarketplaces).

En una máquina donde el marketplace no está registrado, o está registrado bajo el nombre permitido desde una fuente diferente, no aparecen sugerencias de él. La verificación de fuente evita que una fuente no relacionada se registre bajo un nombre permitido para que sus plugins se sugieran en toda su organización.

El siguiente `managed-settings.json` registra un marketplace de organización desde un repositorio de GitHub y habilita sus sugerencias:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

El nombre del marketplace oficial solo puede registrarse desde la fuente oficial de Anthropic, por lo que no necesita declaración de fuente. Para el marketplace oficial, permita solo el nombre:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

<h2 id="preview-what-the-user-sees">
  Vista previa de lo que ve el usuario
</h2>

Cuando la señal `relevance` de un plugin coincide durante una sesión, el consejo debajo del spinner lee:

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

Cuando una señal `cwd` coincide al inicio de la sesión, la notificación de una línea lee:

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

En la pestaña Discover de `/plugin`, el plugin se fija encima de los otros resultados con una anotación que nombra la señal coincidente, como `suggested for this directory` o `suggested for terraform commands`.

Claude Code limita con qué frecuencia sugiere un plugin dado:

* La sugerencia aparece como máximo una vez cada tres sesiones en el spinner tip y la notificación de inicio de sesión combinados.
* La notificación de inicio de sesión deja de aparecer una vez que el spinner tip y la notificación han mostrado el plugin un total combinado de dos veces.
* Ni el spinner tip ni la notificación de inicio de sesión se repiten una vez que el plugin está instalado.
* La pestaña Discover fija el plugin la primera vez que el usuario abre la pestaña mientras las señales del plugin coinciden. Claude Code registra eso en `~/.claude.json`, por lo que cada vez posterior que el usuario abre `/plugin` en esa máquina, el plugin aparece en orden normal.

<h2 id="see-also">
  Ver también
</h2>

* [Alojar un marketplace](/docs/es/plugins/host-marketplace): ejecute el marketplace que aloja sus plugins
* [Referencia de marketplace](/docs/es/plugins/marketplace-reference#plugin-entries): cada campo que acepta una entrada de plugin
* [Recomendar su plugin desde su CLI](/docs/es/plugins/cli-hints): solicite a los usuarios desde su propio CLI en lugar de desde las señales de sesión de Claude Code
* [Administrar plugins para su organización](/docs/es/plugins/org): `extraKnownMarketplaces`, `strictKnownMarketplaces` y el resto de las claves de política de plugins
