> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escanea tu base de código en busca de vulnerabilidades

> Instala el plugin de seguridad de Claude para escanear tu base de código en busca de vulnerabilidades en una sesión de Claude Code y convierte los hallazgos en parches que revisas y aplicas.

El plugin de seguridad de Claude ejecuta un escaneo de vulnerabilidades multiagente de tu base de código dentro de una sesión de Claude Code. Un equipo de agentes de Claude mapea tu arquitectura, construye un modelo de amenazas, busca vulnerabilidades y revisa de forma independiente cada hallazgo antes de escribir el informe. Usa el plugin para escanear un repositorio completo o [solo un conjunto de cambios](#scan-only-your-changes), como el diff de una rama, el diff de una solicitud de extracción o una única confirmación, luego convierte los hallazgos que elijas en parches que revisas y aplicas tú mismo.

El plugin se ejecuta localmente en tu sesión, utiliza los modelos a los que tienes acceso en Claude Code, y cada escaneo cuenta contra los límites de uso de tu plan. Si deseas un servicio administrado que monitoree tus repositorios, o deseas ejecutar escaneos en [Claude Mythos 5](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5), consulta el producto [Claude Security](https://claude.com/product/claude-security), disponible en el plan Enterprise. El plugin accede a código que el producto administrado no puede alcanzar, como repositorios alojados en GitLab o Bitbucket, o en redes que no permiten conexiones entrantes.

El plugin también es distinto de las herramientas de revisión ya presentes en Claude Code: el [plugin de orientación de seguridad](/docs/es/security-guidance) revisa el código mientras Claude lo escribe, [`/security-review`](/docs/es/commands#all-commands) ejecuta un único paso sobre tu rama, y [Code Review](/docs/es/code-review) revisa solicitudes de extracción. Para ver cómo se apilan las capas, consulta [Cómo se ajusta el plugin con otras herramientas de seguridad](#how-the-plugin-fits-with-other-security-tools).

<h2 id="prerequisites">
  Requisitos previos
</h2>

Para ejecutar el plugin, necesitas:

* Un plan de pago, para los [flujos de trabajo dinámicos](/docs/es/workflows) que el escaneo utiliza para orquestar sus agentes. En Pro, actívalos desde la fila Dynamic workflows en `/config`.
* Python 3.9 o posterior disponible en tu `PATH` como `python3`. Verifica con `python3 --version`. Las herramientas del plugin utilizan solo la biblioteca estándar de Python, por lo que no se instala nada.
* Linux, macOS o Windows.
* Git, para escaneos de cambios y para convertir hallazgos en parches; esos trabajos no admiten otros sistemas de control de versiones. Un escaneo completo funciona en cualquier directorio, con o sin control de versiones.

<h2 id="install-the-plugin">
  Instala el plugin
</h2>

En una sesión de Claude Code, instala desde el [mercado oficial de Anthropic](/docs/es/plugins/anthropic-marketplaces):

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

El comando abre los detalles del plugin, donde eliges un [alcance de instalación](/docs/es/plugins/install#install-a-plugin) para iniciar la instalación.

Si la instalación falla, la solución depende del mensaje que reporte Claude Code:

* Si reporta `Marketplace "claude-plugins-official" not found`, añade el mercado con `/plugin marketplace add anthropics/claude-plugins-official`, luego reintenta la instalación.
* Si reporta que no puede [encontrar el plugin en el mercado](/docs/es/plugins/install#install-a-plugin), verifica el nombre del plugin para detectar errores tipográficos.

Verifica el resumen de instalación. Si reporta `Run /reload-plugins to activate.`, consulta [Aplicar cambios de plugins sin reiniciar](/docs/es/plugins/cli-reference#reload-plugins) para activar el plugin en tu sesión actual.

Una vez que el plugin esté activo, estás listo para [escanear y corregir tu base de código](#scan-and-fix-your-codebase).

<h3 id="uninstall-the-plugin">
  Desinstala el plugin
</h3>

Para eliminar el plugin, desinstálalo desde el menú `/plugin`, o ejecuta `claude plugin uninstall claude-security` en tu terminal.

<h2 id="scan-and-fix-your-codebase">
  Escanea y corrige tu base de código
</h2>

El plugin añade un comando, `/claude-security`, que abre un menú de sus tres trabajos: escanear la base de código, escanear un conjunto de cambios y sugerir parches. El camino feliz ejecuta un escaneo completo, luego convierte sus hallazgos en parches:

<Steps>
  <Step title="Abre el menú de seguridad de Claude">
    Ejecuta `/claude-security` y elige **Scan codebase**.
  </Step>

  <Step title="Elige qué escanear">
    El plugin lee tu repositorio primero, luego ofrece el repositorio completo o un área enfocada, con el recuento de archivos y el costo relativo de cada opción indicados. Elige el repositorio completo, o responde "I don't know" y el plugin elige un valor predeterminado sensato para el tamaño de tu repositorio.
  </Step>

  <Step title="Confirma la ejecución">
    Un escaneo puede tomar un tiempo, puede usar una cantidad significativa de tokens, y necesita que Claude Code permanezca abierto mientras se completa. Nada se ejecuta hasta que confirmes.
  </Step>

  <Step title="Lee el informe">
    Mientras el escaneo se ejecuta, reporta cada etapa cuando comienza, con el detalle disponible bajo [`/workflows`](/docs/es/workflows). Los resultados se guardan en un directorio con marca de tiempo en tu repositorio, descrito en [Lee los resultados del escaneo](#read-the-scan-results).
  </Step>

  <Step title="Convierte hallazgos en parches">
    Ejecuta `/claude-security` nuevamente y elige **Suggest patches**, luego selecciona qué hallazgos abordar. Los parches revisados se guardan en la carpeta `patches/` del informe; [Corrige hallazgos](#fix-findings) cubre cómo se construye y revisa cada parche.
  </Step>

  <Step title="Aplica los parches que aceptas">
    Aplica cada parche desde tu shell con `git apply`, en su propia solicitud de extracción. Los parches nunca se aplican automáticamente.
  </Step>
</Steps>

No tienes que comenzar desde el menú: solicita un trabajo directamente, como argumentos del comando, como `/claude-security scan my branch`, o en lenguaje natural, como "scan commit abc1234". El plugin funciona mejor en [modo automático](/docs/es/permission-modes), que permite que los agentes del escaneo procedan sin un aviso de permiso en cada paso.

<h3 id="scan-only-your-changes">
  Escanea solo tus cambios
</h3>

Cuando tu rama tiene confirmaciones que su base no tiene, el menú `/claude-security` ofrece escanear solo ese diff, para que puedas verificar una rama antes de fusionarla. También puedes escanear una de tus solicitudes de extracción abiertas, o una única confirmación solicitándola, como "scan commit abc1234". Solo se escanean los cambios confirmados: confirma o haz stash de los cambios en progreso primero, o ejecuta un escaneo completo, que lee el árbol de trabajo.

Los escaneos de cambios necesitan un repositorio git; los escaneos completos de un directorio sin versión aún funcionan. Encontrar tus solicitudes de extracción abiertas es el único paso que alcanza la red, y se ofrece solo cuando tu sesión ya tiene permiso para ejecutar la CLI de GitHub y `gh` está conectado.

<h3 id="scope-large-repositories">
  Limita repositorios grandes
</h3>

En un repositorio grande, escanea un área a la vez en lugar de todo el árbol. Elige uno de los alcances enfocados que ofrece el plugin, como tu capa de API o tu código de autenticación, y la ejecución se dimensiona a lo que elijas. La sección de cobertura del informe indica qué fue y qué no fue examinado. Ejecuta otro escaneo en un área diferente en cualquier momento.

<h3 id="read-the-scan-results">
  Lee los resultados del escaneo
</h3>

Cada escaneo escribe sus resultados en un directorio `CLAUDE-SECURITY-<timestamp>/` con marca de tiempo en tu repositorio:

* **`CLAUDE-SECURITY-RESULTS.md`**: el informe, con el ID de cada hallazgo, como `F1`, más su impacto, escenario de explotación, severidad, confianza y recomendación
* **`CLAUDE-SECURITY-RESULTS.jsonl`**: los mismos hallazgos en forma legible por máquina, un objeto JSON por línea
* **`CLAUDE-SECURITY-RESULTS.sarif`**: los mismos hallazgos como un registro [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) para escaneo de código de GitHub y cualquier otra herramienta que lea el estándar. El escaneo clasifica hallazgos bajo sus categorías de debilidad [CWE](https://cwe.mitre.org/)
* **`CLAUDE-SECURITY-REVISION-<commit>.json`**: el sello de revisión, registrando qué confirmación fue escaneada, con qué esfuerzo, si los cambios no confirmados fueron parte del árbol escaneado, y cuán minuciosamente se verificó la ejecución, para que un informe siempre esté vinculado al código que describe. Un escaneo fuera del control de versiones marca `UNVERSIONED` en lugar de la confirmación

Ese directorio es el único cambio que un escaneo hace en tu copia de trabajo, y lleva su propio `.gitignore`, por lo que un `git add` extraviado nunca arrastra un informe a una confirmación. Para mantener un informe en el historial para un registro de auditoría, elimina ese único archivo `.gitignore` y confirma el directorio como cualquier otro.

Los hallazgos solo aparecen en el informe después de que agentes verificadores independientes los analicen, lo que mantiene los informes cortos y dignos de leer. Los escaneos son no deterministas: dos escaneos del mismo código pueden revelar hallazgos diferentes. Ejecuta escaneos regularmente, y usa los sellos de revisión para atribuir cada informe al código exacto y la configuración que cubrió.

<h2 id="fix-findings">
  Corrige hallazgos
</h2>

Inicia el flujo de corrección eligiendo **Suggest patches** desde el menú `/claude-security`, o pregunta en lenguaje natural, como "fix finding F3", luego elige qué hallazgos del informe abordar. Los parches se construyen contra código confirmado, y el informe tiene que describir aún el código que tienes: los hallazgos cuyo código ha cambiado desde entonces se omiten con una nota, y el plugin ofrece un escaneo fresco en lugar de aplicar parches desde un informe obsoleto. Cada parche se redacta en una copia de borrador de tu repositorio, por lo que tus archivos fuente permanecen intactos hasta que apliques un parche tú mismo.

Antes de la entrega, cada parche es revisado por un agente independiente del que lo escribió, que ejecuta las pruebas de tu proyecto contra el cambio cuando el código las tiene y lee el diff por sus propios términos para cualquier cosa nueva que pueda introducir. Un parche se escribe solo cuando esa revisión puede garantizar que el cambio aborda el hallazgo único, no introduce ninguna vulnerabilidad nueva, y deja el comportamiento sin cambios. Cuando no puede garantizar los tres, obtienes una nota breve explicando por qué en lugar de un parche.

<h3 id="patches-are-never-applied-automatically">
  Los parches nunca se aplican automáticamente
</h3>

Aplicar un parche siempre es tu decisión. Los parches se guardan en la carpeta `patches/` del informe, uno `F<n>.patch` por hallazgo con una nota al lado explicando el cambio. Aplica uno desde tu shell, o pide a Claude que lo aplique y abra una solicitud de extracción:

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

Cuando el código parcheado no tiene pruebas, la nota del parche lo dice, para que sepas que su revisión se ejecutó sin un paso de prueba. Aplica cada parche en su propia solicitud de extracción para que pueda ser revisado y probado por su cuenta.

<h2 id="how-the-plugin-fits-with-other-security-tools">
  Cómo se ajusta el plugin con otras herramientas de seguridad
</h2>

El plugin de seguridad de Claude es la capa de escaneo profundo bajo demanda en una pila de defensa en profundidad, junto con el [plugin de orientación de seguridad](/docs/es/security-guidance), [`/security-review`](/docs/es/commands#all-commands), [Code Review](/docs/es/code-review), el producto administrado [Claude Security](https://claude.com/product/claude-security), y tus escáneres existentes:

| Etapa                          | Herramienta                                                                    | Qué cubre                                                                                          |
| :----------------------------- | :----------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| En sesión                      | [Plugin de orientación de seguridad](/docs/es/security-guidance)                    | Vulnerabilidades comunes en código que Claude escribe, corregidas en la misma sesión               |
| Bajo demanda, paso único       | [`/security-review`](/docs/es/commands#all-commands)                                | Un paso de seguridad único en la rama actual                                                       |
| Bajo demanda, escaneo profundo | Plugin de seguridad de Claude                                                  | Escaneo multiagente de un repositorio o diff, con hallazgos revisados independientemente y parches |
| En solicitud de extracción     | [Code Review](/docs/es/code-review), planes Team y Enterprise                       | Revisión multiagente de corrección y seguridad con contexto de base de código completo             |
| Administrado                   | [Claude Security](https://claude.com/product/claude-security), plan Enterprise | Escaneo alojado que monitorea repositorios conectados                                              |
| En CI                          | Tus escáneres de análisis estático y dependencias existentes                   | Reglas específicas del lenguaje, verificaciones de cadena de suministro y aplicación de políticas  |

El plugin no reemplaza tus herramientas de seguridad de código fuente existentes. Ejecútalo junto con análisis estático, escaneo de dependencias y revisión de código: razona sobre tu código de la manera que lo haría un investigador de seguridad humano, lo que complementa las verificaciones deterministas que esas herramientas proporcionan.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

**El menú `/claude-security` se abre con una advertencia de Python.** El plugin necesita `python3` 3.9 o posterior en tu `PATH`. Cuando no puede encontrar `python3` en absoluto, el menú advierte que Claude Security no funcionará hasta que se instale uno; cuando el primer `python3` en tu `PATH` es más antiguo, la advertencia nombra la versión que encontró. Instala Python 3, o coloca un `python3` más nuevo primero en tu `PATH`, luego inicia una nueva sesión.

**Puedes ver un aviso "safeguards flagged this message" al escanear en un modelo Fable.** El mensaje nombra el modelo, por ejemplo "Fable 5.1's safeguards flagged this message". Los clasificadores de seguridad de ciberseguridad de Fable marcan ciertas solicitudes, y Claude Code vuelve a ejecutar una solicitud marcada en un modelo Opus a través de [fallback automático de modelo](/docs/es/model-config#automatic-model-fallback). Esto es esperado, y el escaneo aún debería completarse exitosamente.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para profundizar en los temas que esta página toca:

* [Plugin de orientación de seguridad](/docs/es/security-guidance): detecta problemas en el código mientras Claude lo escribe, en la misma sesión
* [Code Review](/docs/es/code-review): configura la revisión multiagente en el momento de la solicitud de extracción
* [Claude Security](https://claude.com/product/claude-security): el servicio administrado que monitorea repositorios conectados
* [Seguridad de Claude Code](/docs/es/security): cómo Claude Code aborda la confianza, los permisos y las salvaguardas
* [Instalar y administrar plugins](/docs/es/plugins/install): busca e instala otros plugins del marketplace oficial
